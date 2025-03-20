use alloy::providers::Provider;
use alloy_contract::CallBuilder;
use alloy_contract::CallDecoder;
use alloy_primitives::Address;
use alloy_transport::TransportError;
use mcr_protocol_client_core_util::McrClientError;
use std::marker::PhantomData;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum McrEthConnectorError {
	#[error(
		"MCR Settlement Transaction failed because gas estimation is too high. Estimated gas: {0} gas limit:{1}"
	)]
	GasLimitExceed(u128, u128),
	#[error("MCR Settlement Transaction failed because account funds were insufficient: {0}")]
	InsufficientFunds(String),
	#[error("MCR Settlement Transaction send failed: {0}")]
	SendTransactionError(#[from] alloy_contract::Error),
	#[error("MCR Settlement Transaction send failed during its execution: {0}")]
	RpcTransactionExecution(String),
	#[error("MCR Settlement BlockAccepted event notification error: {0}")]
	EventNotificationError(#[from] alloy_sol_types::Error),
	#[error("MCR Settlement BlockAccepted event notification stream close")]
	EventNotificationStreamClosed,
}

impl From<McrEthConnectorError> for McrClientError {
	fn from(err: McrEthConnectorError) -> Self {
		McrClientError::Internal(Box::new(err))
	}
}

// Define a rule to verify the error generated when a transaction is send to determine if:
// * the Transaction must me resend with more gas: return Ok(true)
// * a specific error must be return: return Err(McrEthConnectorError::xxx);
// * the rule doesn't apply: return Ok(false)
pub trait VerifyRule: Sync + Send {
	fn verify(&self, error: &alloy_contract::Error) -> Result<bool, McrEthConnectorError>;
}

pub struct SendTransactionErrorRule<Kind> {
	_kind: PhantomData<Kind>,
}

impl<Kind> SendTransactionErrorRule<Kind> {
	pub fn new() -> Self {
		SendTransactionErrorRule { _kind: PhantomData }
	}
}

// Define the current 2 errors managed.
pub struct UnderPriced;
pub struct InsufficentFunds;

impl VerifyRule for SendTransactionErrorRule<UnderPriced> {
	fn verify(&self, error: &alloy_contract::Error) -> Result<bool, McrEthConnectorError> {
		let alloy_contract::Error::TransportError(TransportError::ErrorResp(payload)) = error
		else {
			return Ok(false);
		};

		if payload.code == -32000 && payload.message.contains("transaction underpriced") {
			Ok(true)
		} else {
			Ok(false)
		}
	}
}

impl VerifyRule for SendTransactionErrorRule<InsufficentFunds> {
	fn verify(&self, error: &alloy_contract::Error) -> Result<bool, McrEthConnectorError> {
		let alloy_contract::Error::TransportError(TransportError::ErrorResp(payload)) = error
		else {
			return Ok(false);
		};

		if payload.code == -32000 && payload.message.contains("insufficient funds") {
			Err(McrEthConnectorError::InsufficientFunds(payload.message.to_string()))
		} else {
			Ok(false)
		}
	}
}

pub async fn send_transaction<P: Provider + Clone, D: CallDecoder + Clone>(
	signer_address: Address,
	base_call_builder: CallBuilder<(), &&P, D>,
	send_transaction_error_rules: &[Box<dyn VerifyRule>],
	number_retry: u32,
	gas_limit: u128,
) -> Result<(), McrClientError> {
	let base_call_builder = base_call_builder.from(signer_address);

	// Fetch initial gas estimate
	let mut estimate_gas = base_call_builder
		.estimate_gas()
		.await
		.map_err(|_| McrClientError::Internal("Gas estimation failed".into()))?
		as u128;

	// Apply an initial 20% buffer
	estimate_gas += (estimate_gas * 20) / 100;
	let max_gas_limit = 30_000_000; // Cap max gas to avoid runaway values
	estimate_gas = estimate_gas.min(max_gas_limit);

	info!("Initial estimated gas: {}", estimate_gas);

	for attempt in 0..number_retry {
		info!("Retry attempt: {}", attempt + 1);

		// Clone the base call builder and set gas limit
		let mut call_builder = base_call_builder.clone().gas(estimate_gas as u64);

		// Fetch gas price with a 20% buffer
		let mut gas_price = call_builder
			.provider
			.get_gas_price()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		gas_price += (gas_price * 20) / 100;

		call_builder = call_builder.gas_price(gas_price);

		let transaction_fee_wei = estimate_gas * gas_price;
		if transaction_fee_wei > gas_limit {
			return Err(McrEthConnectorError::GasLimitExceed(transaction_fee_wei, gas_limit).into());
		}

		// Attempt to send the transaction
		let pending_transaction = match call_builder.send().await {
			Ok(tx) => tx,
			Err(err) => {
				// Apply verification rules on failure
				for rule in send_transaction_error_rules {
					if rule.verify(&err)? {
						estimate_gas += (estimate_gas * 10) / 100;
						estimate_gas = estimate_gas.min(max_gas_limit); // Prevent excessive gas increases
						continue;
					}
				}
				return Err(McrEthConnectorError::from(err).into());
			}
		};

		// Check transaction receipt
		match pending_transaction.get_receipt().await {
			Ok(receipt) if !receipt.status() => {
				tracing::debug!(
					"Transaction failed. Gas used: {} / Estimated gas: {}",
					receipt.gas_used,
					estimate_gas
				);

				let tx_gas_threshold = estimate_gas - (estimate_gas * 10) / 100;
				if receipt.gas_used as u128 >= tx_gas_threshold {
					tracing::info!(
						"Transaction failed due to insufficient gas, retrying with increased gas."
					);
					estimate_gas += (estimate_gas * 30) / 100;
					estimate_gas = estimate_gas.min(max_gas_limit);
					continue;
				} else {
					return Err(McrEthConnectorError::RpcTransactionExecution(format!(
						"Transaction failed and was aborted. Receipt: {:?}",
						receipt
					))
					.into());
				}
			}
			Ok(_) => return Ok(()), // Transaction succeeded
			Err(err) => {
				return Err(McrEthConnectorError::RpcTransactionExecution(err.to_string()).into());
			}
		};
	}

	// If all retries fail
	Err(McrEthConnectorError::RpcTransactionExecution(
		"Transaction failed after max retries.".to_string(),
	)
	.into())
}

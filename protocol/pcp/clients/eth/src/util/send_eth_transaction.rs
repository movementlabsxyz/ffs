use alloy::providers::Provider;
use alloy_contract::CallBuilder;
use alloy_contract::CallDecoder;
use alloy_network::Ethereum;
use alloy_transport::{Transport, TransportError};
use pcp_protocol_client_core_util::McrClientError;
use std::marker::PhantomData;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum McrEthConnectorError {
	#[error(
		"PCP Settlement Transaction failed because gas estimation is too high. Estimated gas: {0} gas limit:{1}"
	)]
	GasLimitExceed(u128, u128),
	#[error("PCP Settlement Transaction failed because account funds were insufficient: {0}")]
	InsufficientFunds(String),
	#[error("PCP Settlement Transaction send failed: {0}")]
	SendTransactionError(#[from] alloy_contract::Error),
	#[error("PCP Settlement Transaction send failed during its execution: {0}")]
	RpcTransactionExecution(String),
	#[error("PCP Settlement BlockAccepted event notification error: {0}")]
	EventNotificationError(#[from] alloy_sol_types::Error),
	#[error("PCP Settlement BlockAccepted event notification stream close")]
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
			Err(McrEthConnectorError::InsufficientFunds(payload.message.clone()))
		} else {
			Ok(false)
		}
	}
}

pub async fn send_transaction<
	P: Provider<T, Ethereum> + Clone,
	T: Transport + Clone,
	D: CallDecoder + Clone,
>(
	base_call_builder: CallBuilder<T, &&P, D, Ethereum>,
	send_transaction_error_rules: &[Box<dyn VerifyRule>],
	number_retry: u32,
	gas_limit: u128,
) -> Result<(), McrClientError> {
	info!("Sending transaction with gas limit: {}", gas_limit);
	//validate gas price.
	let mut estimate_gas = base_call_builder.estimate_gas().await.expect("Failed to estimate gas");
	// Add 20% because initial gas estimate are too low.
	estimate_gas += (estimate_gas * 20) / 100;

	info!("estimated_gas: {}", estimate_gas);

	// Sending Transaction automatically can lead to errors that depend on the state for Eth.
	// It's convenient to manage some of them automatically to avoid to fail commitment Transaction.
	// I define a first one but other should be added depending on the test with mainnet.
	for _ in 0..number_retry {
		let call_builder = base_call_builder.clone().gas(estimate_gas);

		//detect if the gas price doesn't execeed the limit.
		let gas_price = call_builder
			.provider
			.get_gas_price()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		let transaction_fee_wei = estimate_gas * gas_price;
		if transaction_fee_wei > gas_limit {
			return Err(McrEthConnectorError::GasLimitExceed(transaction_fee_wei, gas_limit).into());
		}

		info!("Sending transaction with gas: {}", estimate_gas);

		//send the Transaction and detect send error.
		let pending_transaction = match call_builder.send().await {
			Ok(pending_transaction) => pending_transaction,
			Err(err) => {
				//apply defined rules.
				for rule in send_transaction_error_rules {
					// Verify all rules. If one rule return true or an error stop verification.
					// If true retry with more gas else return the error.
					if rule.verify(&err)? {
						//increase gas of 10% and retry
						estimate_gas += (estimate_gas * 10) / 100;
						tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
						continue;
					}
				}

				return Err(McrEthConnectorError::from(err).into());
			}
		};

		match pending_transaction.get_receipt().await {
			// Transaction execution fail
			Ok(transaction_receipt) if !transaction_receipt.status() => {
				tracing::debug!(
					"transaction_receipt.gas_used: {} / estimate_gas: {estimate_gas}",
					transaction_receipt.gas_used
				);
				// Some valid Tx can abort cause of insufficient gas without consuming all its gas.
				// Define a threshold a little less than estimated gas to detect them.
				let tx_gas_consumption_threshold = estimate_gas - (estimate_gas * 10) / 100;
				if transaction_receipt.gas_used >= tx_gas_consumption_threshold {
					tracing::info!("Send commitment Transaction  fail because of insufficient gas, receipt:{transaction_receipt:?} ");
					estimate_gas += (estimate_gas * 30) / 100;
					continue;
				} else {
					return Err(McrEthConnectorError::RpcTransactionExecution(format!(
						"Send commitment Transaction fail, abort Transaction, receipt:{transaction_receipt:?}"
					))
					.into());
				}
			}
			Ok(_) => return Ok(()),
			Err(err) => {
				return Err(McrEthConnectorError::RpcTransactionExecution(err.to_string()).into())
			}
		};
	}

	//Max retry exceed
	Err(McrEthConnectorError::RpcTransactionExecution(
		"Send commitment Transaction fail because of exceed max retry".to_string(),
	)
	.into())
}

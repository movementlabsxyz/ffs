use crate::util::send_eth_transaction::send_transaction;
use crate::util::send_eth_transaction::PcpEthConnectorError;
use crate::util::send_eth_transaction::VerifyRule;
use alloy::providers::Provider;
use alloy_primitives::Address;
use alloy_primitives::U256;
use alloy_sol_types::sol;
use anyhow::Context;
use pcp_protocol_client_core_util::{CommitmentStream, PcpClientError, PcpClientOperations};
use pcp_types::commitment::{Commitment, CommitmentValue, CommitmentId};
use serde_json::Value as JsonValue;
use std::array::TryFromSliceError;
use std::fs;
use std::path::Path;
use tokio_stream::StreamExt;

// Note: we prefer using the ABI because the [`sol!`](alloy_sol_types::sol) macro, when used with smart contract code directly, will not handle inheritance.
sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	PCP,
	"abis/PCP.json"
);

// Note: we prefer using the ABI because the [`sol!`](alloy_sol_types::sol) macro, when used with smart contract code directly, will not handle inheritance.
sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	MovementStaking,
	"abis/MovementStaking.json"
);

// Note: we prefer using the ABI because the [`sol!`](alloy_sol_types::sol) macro, when used with smart contract code directly, will not handle inheritance.
sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	MOVEToken,
	"abis/MOVEToken.json"
);

pub struct Client<R, W> {
	pub(crate) run_commitment_admin_mode: bool,
	pub(crate) rpc_provider: R,
	pub(crate) ws_provider: W,
	pub(crate) signer_address: Address,
	pub(crate) contract_address: Address,
	pub(crate) send_transaction_error_rules: Vec<Box<dyn VerifyRule>>,
	pub(crate) gas_limit: u64,
	pub(crate) send_transaction_retries: u32,
}

impl<R, W> PcpClientOperations for Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	async fn post_commitment(
		&self,
		commitment: Commitment,
	) -> Result<(), PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.rpc_provider);

		let eth_commitment = PCPStorage::Commitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(commitment.height()),
			commitmentValue: alloy_primitives::FixedBytes(
				commitment.commitment_value().as_bytes().clone(),
			),
			commitmentId: alloy_primitives::FixedBytes(commitment.commitment_id().as_bytes().clone()),
		};

		if self.run_commitment_admin_mode {
			let call_builder = contract.forceLatestCommitment(eth_commitment);
			send_transaction(
				self.signer_address.clone(),
				call_builder,
				&self.send_transaction_error_rules,
				self.send_transaction_retries,
				self.gas_limit as u128,
			)
			.await
		} else {
			let call_builder = contract.submitCommitment(eth_commitment);
			send_transaction(
				self.signer_address.clone(),
				call_builder,
				&self.send_transaction_error_rules,
				self.send_transaction_retries,
				self.gas_limit as u128,
			)
			.await
		}
	}

	async fn post_commitment_batch(
		&self,
		commitments: Vec<Commitment>,
	) -> Result<(), PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.rpc_provider);

		let eth_commitment: Vec<_> = commitments
			.into_iter()
			.map(|commitment| {
				Ok(PCPStorage::Commitment {
					// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
					height: U256::from(commitment.height()),
					commitmentValue: alloy_primitives::FixedBytes(
						commitment.commitment_value().as_bytes().clone(),
					),
					commitmentId: alloy_primitives::FixedBytes(
						commitment.commitment_id().as_bytes().clone(),
					),
				})
			})
			.collect::<Result<Vec<_>, TryFromSliceError>>()
			.map_err(|e| PcpClientError::Internal(Box::new(e)))?;

		let call_builder = contract.submitBatchCommitment(eth_commitment);

		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn force_commitment(
		&self,
		commitment: Commitment,
	) -> Result<(), PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.rpc_provider);

		let eth_commitment = PCPStorage::Commitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(commitment.height()),
			commitmentValue: alloy_primitives::FixedBytes(
				commitment.commitment_value().as_bytes().clone(),
			),
			commitmentId: alloy_primitives::FixedBytes(commitment.commitment_id().as_bytes().clone()),
		};

		let call_builder = contract.forceLatestCommitment(eth_commitment);
		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn stream_commitments(&self) -> Result<CommitmentStream, PcpClientError> {
		// Register to contract CommitmentSubmitted event

		let contract = PCP::new(self.contract_address, &self.ws_provider);
		let event_filter = contract
			.CommitmentPostconfirmed_filter()
			.watch()
			.await
			.map_err(|e| PcpClientError::StreamCommitments(Box::new(e)))?;

		let stream = event_filter.into_stream().map(|event| {
			event
				.and_then(|(commitment, _)| {
					let height = commitment.height.try_into().map_err(
						|err: alloy::primitives::ruint::FromUintError<u64>| {
							alloy_sol_types::Error::Other(err.to_string().into())
						},
					)?;
					Ok(Commitment::new(
						height,
						CommitmentId::new(commitment.blockHash.0),
						CommitmentValue::new(commitment.stateCommitment.0),
					))
				})
				.map_err(|err| PcpEthConnectorError::EventNotificationError(err).into())
		});
		Ok(Box::pin(stream) as CommitmentStream)
	}

	async fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.ws_provider);
		let PCP::getPostconfirmedCommitmentReturn { _0: commitment } = contract
			.getPostconfirmedCommitment(U256::from(height))
			.call()
			.await
			.map_err(|e| PcpClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| PcpClientError::Internal(e.into()))?;
		// Commitment with height 0 mean not found
		Ok((return_height != 0).then_some(Commitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| PcpClientError::Internal(e.into()))?,
			CommitmentId::new(commitment.commitmentId.into()),
			CommitmentValue::new(commitment.commitmentValue.into()),
		)))
	}

	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.ws_provider);
		let PCP::getValidatorCommitmentAtCommitmentHeightReturn { _0: commitment } = contract
			.getValidatorCommitmentAtCommitmentHeight(U256::from(height), self.signer_address)
			.call()
			.await
			.map_err(|e| PcpClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| PcpClientError::Internal(e.into()))?;

		Ok((return_height != 0).then_some(Commitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| PcpClientError::Internal(e.into()))?,
			CommitmentId::new(commitment.commitmentId.into()),
			CommitmentValue::new(commitment.commitmentValue.into()),
		)))
	}

	async fn get_max_tolerable_commitment_height(&self) -> Result<u64, PcpClientError> {
		let contract = PCP::new(self.contract_address, &self.ws_provider);
		let PCP::getMaxTolerableCommitmentHeightReturn { _0: block_height } = contract
			.getMaxTolerableCommitmentHeight()
			.call()
			.await
			.map_err(|e| PcpClientError::Internal(Box::new(e)))?;
		Ok(block_height
			.try_into()
			.context("Failed to convert the max tolerable block height from U256 to u64")
			.map_err(|e| PcpClientError::Internal(e.into()))?)
	}
}

pub struct AnvilAddressEntry {
	pub address: String,
	pub private_key: String,
}

/// Read the Anvil config file keys and return all address/private keys.
pub fn read_anvil_json_file_addresses<P: AsRef<Path>>(
	anvil_conf_path: P,
) -> Result<Vec<AnvilAddressEntry>, PcpClientError> {
	let file_content =
		fs::read_to_string(anvil_conf_path).map_err(|e| PcpClientError::Internal(Box::new(e)))?;

	let json_value: JsonValue =
		serde_json::from_str(&file_content).map_err(|e| PcpClientError::Internal(Box::new(e)))?;

	// Extract the available_accounts and private_keys fields.
	let available_accounts_iter = json_value["available_accounts"]
		.as_array()
		.expect("Available_accounts should be an array")
		.iter()
		.map(|v| {
			let s = v.as_str().expect("Available_accounts elements should be strings");
			s.to_owned()
		});

	let private_keys_iter = json_value["private_keys"]
		.as_array()
		.expect("Private_keys should be an array")
		.iter()
		.map(|v| {
			let s = v.as_str().expect("Private_keys elements should be strings");
			s.to_owned()
		});

	let res = available_accounts_iter
		.zip(private_keys_iter)
		.map(|(address, private_key)| AnvilAddressEntry { address, private_key })
		.collect::<Vec<_>>();
	Ok(res)
}

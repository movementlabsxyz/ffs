use crate::util::send_eth_transaction::send_transaction;
use crate::util::send_eth_transaction::McrEthConnectorError;
use crate::util::send_eth_transaction::VerifyRule;
use alloy::providers::Provider;
use alloy_primitives::Address;
use alloy_sol_types::sol;
use anyhow::Context;
use mcr_protocol_client_core_util::{CommitmentStream, McrClientError, McrClientOperations, U256};
use mcr_types::block_commitment::{BlockCommitment, Commitment, Id};
use serde_json::Value as JsonValue;
use std::array::TryFromSliceError;
use std::fs;
use std::path::Path;
use tokio_stream::StreamExt;

// Note: we prefer using the ABI because the [`sol!`](alloy_sol_types::sol) macro, when used with smart contract code directly, will not handle inheritance.
sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	MCR,
	"abis/MCR.json"
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

impl<R, W> McrClientOperations for Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	async fn post_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_block_commitment = MCRStorage::BlockCommitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(block_commitment.height()),
			commitment: alloy_primitives::FixedBytes(
				block_commitment.commitment().as_bytes().clone(),
			),
			blockId: alloy_primitives::FixedBytes(block_commitment.block_id().as_bytes().clone()),
		};

		if self.run_commitment_admin_mode {
			let call_builder = contract.forceLatestCommitment(eth_block_commitment);
			send_transaction(
				self.signer_address.clone(),
				call_builder,
				&self.send_transaction_error_rules,
				self.send_transaction_retries,
				self.gas_limit as u128,
			)
			.await
		} else {
			let call_builder = contract.submitBlockCommitment(eth_block_commitment);
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

	async fn post_block_commitment_batch(
		&self,
		block_commitments: Vec<BlockCommitment>,
	) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_block_commitment_batch: Vec<_> = block_commitments
			.into_iter()
			.map(|block_commitment| {
				Ok(MCRStorage::BlockCommitment {
					// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
					height: U256::from(block_commitment.height()),
					commitment: alloy_primitives::FixedBytes(
						block_commitment.commitment().as_bytes().clone(),
					),
					blockId: alloy_primitives::FixedBytes(
						block_commitment.block_id().as_bytes().clone(),
					),
				})
			})
			.collect::<Result<Vec<_>, TryFromSliceError>>()
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let call_builder = contract.submitBatchBlockCommitment(eth_block_commitment_batch);

		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn force_block_commitment(
		&self,
		block_commitment: BlockCommitment,
	) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_block_commitment = MCRStorage::BlockCommitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(block_commitment.height()),
			commitment: alloy_primitives::FixedBytes(
				block_commitment.commitment().as_bytes().clone(),
			),
			blockId: alloy_primitives::FixedBytes(block_commitment.block_id().as_bytes().clone()),
		};

		let call_builder = contract.forceLatestCommitment(eth_block_commitment);
		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn stream_block_commitments(&self) -> Result<CommitmentStream, McrClientError> {
		// Register to contract BlockCommitmentSubmitted event

		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let event_filter = contract
			.BlockAccepted_filter()
			.watch()
			.await
			.map_err(|e| McrClientError::StreamBlockCommitments(Box::new(e)))?;

		let stream = event_filter.into_stream().map(|event| {
			event
				.and_then(|(commitment, _)| {
					let height = commitment.height.try_into().map_err(
						|err: alloy::primitives::ruint::FromUintError<u64>| {
							alloy_sol_types::Error::Other(err.to_string().into())
						},
					)?;
					Ok(BlockCommitment::new(
						height,
						Id::new(commitment.blockHash.0),
						Commitment::new(commitment.stateCommitment.0),
					))
				})
				.map_err(|err| McrEthConnectorError::EventNotificationError(err).into())
		});
		Ok(Box::pin(stream) as CommitmentStream)
	}

	async fn get_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<BlockCommitment>, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getAcceptedCommitmentAtBlockHeightReturn { _0: commitment } = contract
			.getAcceptedCommitmentAtBlockHeight(U256::from(height))
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		// Commitment with height 0 mean not found
		Ok((return_height != 0).then_some(BlockCommitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			Id::new(commitment.blockId.into()),
			Commitment::new(commitment.commitment.into()),
		)))
	}

	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<BlockCommitment>, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getValidatorCommitmentAtBlockHeightReturn { _0: commitment } = contract
			.getValidatorCommitmentAtBlockHeight(U256::from(height), self.signer_address)
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?;

		Ok((return_height != 0).then_some(BlockCommitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			Id::new(commitment.blockId.into()),
			Commitment::new(commitment.commitment.into()),
		)))
	}

	async fn get_max_tolerable_block_height(&self) -> Result<u64, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getMaxTolerableBlockHeightReturn { _0: block_height } = contract
			.getMaxTolerableBlockHeight()
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		Ok(block_height
			.try_into()
			.context("Failed to convert the max tolerable block height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?)
	}

	async fn stake(&self, amount: U256) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let call_builder = contract.stake(U256::from(amount));
		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn unstake(&self, amount: U256) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let call_builder = contract.unstake(U256::from(amount));
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

pub struct AnvilAddressEntry {
	pub address: String,
	pub private_key: String,
}

/// Read the Anvil config file keys and return all address/private keys.
pub fn read_anvil_json_file_addresses<P: AsRef<Path>>(
	anvil_conf_path: P,
) -> Result<Vec<AnvilAddressEntry>, McrClientError> {
	let file_content =
		fs::read_to_string(anvil_conf_path).map_err(|e| McrClientError::Internal(Box::new(e)))?;

	let json_value: JsonValue =
		serde_json::from_str(&file_content).map_err(|e| McrClientError::Internal(Box::new(e)))?;

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

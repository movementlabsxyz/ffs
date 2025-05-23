use crate::util::send_eth_transaction::send_transaction;
use crate::util::send_eth_transaction::McrEthConnectorError;
use crate::util::send_eth_transaction::VerifyRule;
use alloy::providers::Provider;
use alloy_primitives::Address;
use alloy_sol_types::sol;
use anyhow::Context;
use mcr_protocol_client_core_util::{CommitmentStream, McrClientError, McrClientOperations, U256};
use mcr_types::commitment::{Commitment, Id, Vote};
use serde_json::Value as JsonValue;
use std::array::TryFromSliceError;
use std::fs;
use std::path::Path;
use tokio_stream::StreamExt;
pub mod request;

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

pub struct Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	pub(crate) run_commitment_admin_mode: bool,
	pub(crate) rpc_provider: R,
	pub(crate) ws_provider: W,
	pub(crate) signer_address: Address,
	pub(crate) contract_address: Address,
	pub(crate) move_token_address: Address,
	// pub(crate) staking_address: Address,
	pub(crate) send_transaction_error_rules: Vec<Box<dyn VerifyRule>>,
	pub(crate) gas_limit: u64,
	pub(crate) send_transaction_retries: u32,
}

impl<R, W> McrClientOperations for Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	async fn post_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		// block commitment that is made ready for eth
		let eth_commitment = MCRStorage::Commitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(commitment.height()),
			vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
			id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
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
			let call_builder = contract.submitCommitment(eth_commitment).from(self.signer_address);

			println!("Debug [post_commitment] - About to send transaction");
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
	) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		// batch of block commitment that is made ready for eth
		let eth_commitment_batch: Vec<_> = commitments
			.into_iter()
			.map(|commitment| {
				Ok(MCRStorage::Commitment {
					// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
					height: U256::from(commitment.height()),
					vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
					id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
				})
			})
			.collect::<Result<Vec<_>, TryFromSliceError>>()
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let call_builder = contract.submitBatchCommitment(eth_commitment_batch);

		send_transaction(
			self.signer_address.clone(),
			call_builder,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
	}

	async fn force_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_commitment = MCRStorage::Commitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(commitment.height()),
			vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
			id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
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

	async fn stream_commitments(&self) -> Result<CommitmentStream, McrClientError> {
		// Register to contract CommitmentSubmitted event

		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let event_filter = contract
			.CommitmentAccepted_filter()
			.watch()
			.await
			.map_err(|e| McrClientError::StreamCommitments(Box::new(e)))?;

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
						Id::new(commitment.blockHash.0),
						Vote::new(commitment.stateCommitment.0),
					))
				})
				.map_err(|err| McrEthConnectorError::EventNotificationError(err).into())
		});
		Ok(Box::pin(stream) as CommitmentStream)
	}

	async fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getAcceptedCommitmentAtCommitmentHeightReturn { _0: commitment } = contract
			.getAcceptedCommitmentAtCommitmentHeight(U256::from(height))
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?;
		// Commitment with height 0 mean not found
		Ok((return_height != 0).then_some(Commitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			Id::new(commitment.id.into()),
			Vote::new(commitment.vote.into()),
		)))
	}

	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getValidatorCommitmentAtHeightReturn { _0: commitment } = contract
			.getValidatorCommitmentAtHeight(U256::from(height), self.signer_address)
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?;

		Ok((return_height != 0).then_some(Commitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			Id::new(commitment.id.into()),
			Vote::new(commitment.vote.into()),
		)))
	}

	async fn get_max_tolerable_commitment_height(&self) -> Result<u64, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let MCR::getMaxTolerableCommitmentHeightReturn { _0: block_height } = contract
			.getMaxTolerableCommitmentHeight()
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		Ok(block_height
			.try_into()
			.context("Failed to convert the max tolerable block height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?)
	}

	async fn get_validator_commitment_at_height(
		&self,
		height: u64,
		attester: String,
	) -> Result<Option<Commitment>, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let MCR::getValidatorCommitmentAtHeightReturn { _0: commitment } = contract
			.getValidatorCommitmentAtHeight(U256::from(height), attester_addr)
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let return_height: u64 = commitment
			.height
			.try_into()
			.context("failed to convert the commitment height from U256 to u64")
			.map_err(|e| McrClientError::Internal(e.into()))?;

		Ok((return_height != 0).then_some(Commitment::new(
			commitment
				.height
				.try_into()
				.context("failed to convert the commitment height from U256 to u64")
				.map_err(|e| McrClientError::Internal(e.into()))?,
			Id::new(commitment.id.into()),
			Vote::new(commitment.vote.into()),
		)))
	}

	/// Get the MOVE-octas token balance of the specified address
	async fn get_balance(&self, address: String) -> Result<u64, McrClientError> {
		let token = MOVEToken::new(self.move_token_address, &self.rpc_provider);
		let addr = address.parse::<Address>().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let balance = token
			.balanceOf(addr)
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(balance._0.try_into().map_err(|e| McrClientError::Internal(Box::new(e)))?)
	}

	async fn get_last_accepted_block_height(&self) -> Result<u64, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let MCR::lastAcceptedCommitmentHeightReturn { _0: height } = contract
			.lastAcceptedCommitmentHeight()
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		Ok(height.try_into().unwrap())
	}

	async fn get_leading_commitment_tolerance(&self) -> Result<u64, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let MCR::leadingCommitmentToleranceReturn { _0: tolerance } = contract
			.leadingCommitmentTolerance()
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;
		Ok(tolerance.try_into().unwrap())
	}

	/// Grants TRUSTED_ATTESTER role to the specified address
	async fn grant_trusted_attester(&self, attester: String) -> Result<(), McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let tx = contract.grantTrustedAttester(attester_addr).from(self.signer_address);

		send_transaction(
			self.signer_address.clone(),
			tx,
			&self.send_transaction_error_rules,
			self.send_transaction_retries,
			self.gas_limit as u128,
		)
		.await
		.map_err(|e| McrClientError::AdminFunction(Box::new(e)))?;

		Ok(())
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

	/// Get the current epoch stake for an attester
	async fn get_stake(&self, custodian: String, attester: String) -> Result<u64, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let custodian_addr =
			custodian.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let MCR::getCurrentEpochStakeReturn { _0: stake } = contract
			.getCurrentEpochStake(custodian_addr, attester_addr)
			.call()
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(stake.try_into().map_err(|e| McrClientError::Internal(Box::new(e)))?)
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

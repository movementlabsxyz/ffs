use alloy::providers::Provider;
use alloy_network::eip2718::Encodable2718;
use alloy_network::EthereumWallet;
use alloy_network::TransactionBuilder;
use alloy_rpc_types::Filter;
use alloy_rpc_types::TransactionRequest;
use alloy_sol_types::SolEvent;
use alloy_sol_types::SolType;
use helios::common::types::BlockTag;
use helios::ethereum::{database::FileDB, EthereumClient};
use mcr_protocol_client_core_util::{CommitmentStream, McrClientError, McrClientOperations, U256};
use mcr_protocol_client_eth_core::client::{Client as CoreClient, MCRStorage, MCR};
use mcr_types::commitment::{Commitment, Id, Vote};
use std::sync::Arc;
use std::time::Duration;
pub struct AlloyCompatibility;

impl AlloyCompatibility {
	pub fn transaction_request_backwards(
		request: TransactionRequest,
	) -> Result<helios::ethereum::alloy::rpc::types::TransactionRequest, McrClientError> {
		let new_serialized =
			serde_json::to_vec(&request).map_err(|e| McrClientError::Internal(e.into()))?;

		let old_deserialized = serde_json::from_slice(&new_serialized)
			.map_err(|e| McrClientError::Internal(e.into()))?;

		Ok(old_deserialized)
	}

	pub fn filter_backwards(
		filter: Filter,
	) -> Result<helios::ethereum::alloy::rpc::types::Filter, McrClientError> {
		let new_serialized =
			serde_json::to_vec(&filter).map_err(|e| McrClientError::Internal(e.into()))?;

		let old_deserialized = serde_json::from_slice(&new_serialized)
			.map_err(|e| McrClientError::Internal(e.into()))?;

		Ok(old_deserialized)
	}
}

pub struct PollingParameters {
	pub interval: Duration,
	pub timeout: Duration,
}

pub struct Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	pub(crate) signer: EthereumWallet,
	pub(crate) core_client: CoreClient<R, W>,
	pub(crate) eth_client: Arc<EthereumClient<FileDB>>,
	pub(crate) polling_parameters: PollingParameters,
	pub(crate) block_finality: BlockTag,
}

impl<R, W> McrClientOperations for Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	async fn post_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.submit_commitment_request(commitment)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn post_commitment_batch(
		&self,
		commitments: Vec<Commitment>,
	) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.submit_commitment_batch_request(commitments)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn force_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.force_commitment_request(commitment)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		// Form the request
		let transaction_request =
			self.core_client.get_accepted_commitment_at_height_request(height)?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let commitment_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let data = commitment_raw.to_vec(); // TODO: haha this version of alloy calls

		// Decode the result
		let commitment = MCRStorage::Commitment::abi_decode(data.as_slice(), true)
			.map_err(|e| McrClientError::Internal(e.into()))?;

		if commitment.height == U256::ZERO {
			return Ok(None);
		}

		Ok(Some(Commitment::new(
			commitment.height.try_into().map_err(
				|e: alloy::primitives::ruint::FromUintError<u64>| {
					McrClientError::Internal(e.into())
				},
			)?,
			Id::new(commitment.id.0),
			Vote::new(commitment.vote.0),
		)))
	}

	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		// Form the request
		let transaction_request =
			self.core_client.get_posted_commitment_at_height_request(height)?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let commitment_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let data = commitment_raw.to_vec(); // TODO: haha this version of alloy calls

		// Decode the result
		let commitment = MCRStorage::Commitment::abi_decode(data.as_slice(), true)
			.map_err(|e| McrClientError::Internal(e.into()))?;

		if commitment.height == U256::ZERO {
			return Ok(None);
		}

		Ok(Some(Commitment::new(
			commitment.height.try_into().map_err(
				|e: alloy::primitives::ruint::FromUintError<u64>| {
					McrClientError::Internal(e.into())
				},
			)?,
			Id::new(commitment.id.0),
			Vote::new(commitment.vote.0),
		)))
	}

	async fn get_max_tolerable_commitment_height(&self) -> Result<u64, McrClientError> {
		let transaction_request = self.core_client.get_max_tolerable_commitment_height_request()?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let max_tolerable_commitment_height_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let data = max_tolerable_commitment_height_raw.to_vec(); // TODO: haha this version of alloy calls

		// Decode the result
		let max_tolerable_commitment_height = U256::try_from_le_slice(&data).ok_or(
			McrClientError::Internal("Failed to decode max tolerable commitment height".into()),
		)?;

		let height = max_tolerable_commitment_height.try_into().map_err(
			|e: alloy::primitives::ruint::FromUintError<u64>| McrClientError::Internal(e.into()),
		)?;

		Ok(height)
	}

	async fn get_validator_commitment_at_height(
		&self,
		height: u64,
		attester: String,
	) -> Result<Option<Commitment>, McrClientError> {
		let transaction_request =
			self.core_client.get_validator_commitment_at_height_request(height, attester)?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let commitment_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		let data = commitment_raw.to_vec(); // TODO: haha this version of alloy calls

		// Decode the result
		let commitment = MCRStorage::Commitment::abi_decode(data.as_slice(), true)
			.map_err(|e| McrClientError::Internal(e.into()))?;

		if commitment.height == U256::ZERO {
			return Ok(None);
		}

		Ok(Some(Commitment::new(
			commitment.height.try_into().map_err(
				|e: alloy::primitives::ruint::FromUintError<u64>| {
					McrClientError::Internal(e.into())
				},
			)?,
			Id::new(commitment.id.0),
			Vote::new(commitment.vote.0),
		)))
	}

	async fn get_balance(&self, address: String) -> Result<u64, McrClientError> {
		let transaction_request = self.core_client.get_balance_request(address)?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// MCommitment::newake the call
		let balance_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;

		let balance = U256::try_from_le_slice(&balance_raw)
			.ok_or(McrClientError::Internal("Failed to decode balance".into()))?;

		Ok(balance.try_into().map_err(|e: alloy::primitives::ruint::FromUintError<u64>| {
			McrClientError::Internal(e.into())
		})?)
	}

	async fn get_last_accepted_block_height(&self) -> Result<u64, McrClientError> {
		let transaction_request = self.core_client.get_last_accepted_commitment_height_request()?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let last_accepted_block_height_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;

		let last_accepted_block_height = U256::try_from_le_slice(&last_accepted_block_height_raw)
			.ok_or(McrClientError::Internal(
			"Failed to decode last accepted block height".into(),
		))?;

		Ok(last_accepted_block_height.try_into().map_err(
			|e: alloy::primitives::ruint::FromUintError<u64>| McrClientError::Internal(e.into()),
		)?)
	}

	async fn get_leading_commitment_tolerance(&self) -> Result<u64, McrClientError> {
		let transaction_request = self.core_client.get_leading_commitment_tolerance_request()?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let leading_commitment_tolerance_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;

		let leading_commitment_tolerance = U256::try_from_le_slice(
			&leading_commitment_tolerance_raw,
		)
		.ok_or(McrClientError::Internal("Failed to decode leading commitment tolerance".into()))?;

		Ok(leading_commitment_tolerance.try_into().map_err(
			|e: alloy::primitives::ruint::FromUintError<u64>| McrClientError::Internal(e.into()),
		)?)
	}

	async fn grant_trusted_attester(&self, attester: String) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.grant_trusted_attester_request(attester)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn stake(&self, amount: U256) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.stake_request(amount)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn get_stake(&self, custodian: String, address: String) -> Result<u64, McrClientError> {
		let transaction_request = self.core_client.get_stake_request(custodian, address)?;
		let transaction_request =
			AlloyCompatibility::transaction_request_backwards(transaction_request)?;

		// Make the call
		let stake_raw = self
			.eth_client
			.call(&transaction_request, self.block_finality)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;

		let stake = U256::try_from_le_slice(&stake_raw)
			.ok_or(McrClientError::Internal("Failed to decode stake".into()))?;

		Ok(stake.try_into().map_err(|e: alloy::primitives::ruint::FromUintError<u64>| {
			McrClientError::Internal(e.into())
		})?)
	}

	async fn unstake(&self, amount: U256) -> Result<(), McrClientError> {
		let transaction_request = self.core_client.unstake_request(amount)?;
		let signed_transaction = transaction_request
			.build(&self.signer)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		self.eth_client
			.send_raw_transaction(signed_transaction.encoded_2718().as_slice())
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(())
	}

	async fn stream_commitments(&self) -> Result<CommitmentStream, McrClientError> {
		// get the filter
		let filter = self.core_client.stream_commitments_filter()?;
		let filter = AlloyCompatibility::filter_backwards(filter)?;
		let filter_id = self
			.eth_client
			.new_filter(&filter)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;

		// clone the client and the interval for lifetime purposes
		let eth_client = self.eth_client.clone();
		let interval = self.polling_parameters.interval.clone();

		let stream = async_stream::try_stream! {

			loop {
				let changes = eth_client.get_filter_changes(filter_id).await.map_err(|e| McrClientError::Internal(e.into()))?;

				match changes {
					helios::ethereum::alloy::rpc::types::FilterChanges::Logs(logs) => {
						for log in logs {

							let commitment = MCR::CommitmentAccepted::decode_raw_log(log.topics().iter().copied(), &log.data().data.to_vec(), false)
								.map_err(|e| McrClientError::Internal(e.into()))?;

							yield Commitment::new(
								commitment.height.try_into().map_err(
									|e: alloy::primitives::ruint::FromUintError<u64>| {
										McrClientError::Internal(e.into())
									},
								)?,
								// TODO: these need to be changed to `id` and `vote` in the contracts.
								Id::new(commitment.blockHash.0),
								Vote::new(commitment.stateCommitment.0),
							)
						}
					},
					_ => {}
				}

				// sleep for a bit
				tokio::time::sleep(interval).await;
			}

		};

		Ok(Box::pin(stream) as CommitmentStream)
	}
}

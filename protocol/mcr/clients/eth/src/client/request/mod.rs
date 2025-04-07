use super::{Client, MCRStorage, MCR};
use alloy::providers::Provider;
use alloy_network::{Ethereum, Network};
use mcr_protocol_client_core_util::{McrClientError, U256};
use mcr_types::commitment::Commitment;
use std::array::TryFromSliceError;

impl<R, W> Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	pub fn submit_commitment_request(
		&self,
		commitment: Commitment,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		// block commitment that is made ready for eth
		let eth_commitment = MCRStorage::Commitment {
			// Currently, to simplify the API, we'll say 0 is uncommitted all other numbers are legitimate heights
			height: U256::from(commitment.height()),
			vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
			id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
		};

		Ok(contract
			.submitCommitment(eth_commitment)
			.from(self.signer_address)
			.into_transaction_request())
	}

	pub async fn submit_commitment_batch_request(
		&self,
		commitments: Vec<Commitment>,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_commitment_batch: Vec<_> = commitments
			.into_iter()
			.map(|commitment| {
				Ok(MCRStorage::Commitment {
					height: U256::from(commitment.height()),
					vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
					id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
				})
			})
			.collect::<Result<Vec<_>, TryFromSliceError>>()
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(contract
			.submitBatchCommitment(eth_commitment_batch)
			.from(self.signer_address)
			.into_transaction_request())
	}

	pub async fn force_commitment_request(
		&self,
		commitment: Commitment,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		let eth_commitment = MCRStorage::Commitment {
			height: U256::from(commitment.height()),
			vote: alloy_primitives::FixedBytes(commitment.vote().as_bytes().clone()),
			id: alloy_primitives::FixedBytes(commitment.id().as_bytes().clone()),
		};

		Ok(contract
			.forceLatestCommitment(eth_commitment)
			.from(self.signer_address)
			.into_transaction_request())
	}

	pub async fn grant_trusted_attester_request(
		&self,
		attester: String,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(contract
			.grantTrustedAttester(attester_addr)
			.from(self.signer_address)
			.into_transaction_request())
	}

	pub async fn stake_request(
		&self,
		amount: U256,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		Ok(contract.stake(amount).from(self.signer_address).into_transaction_request())
	}

	pub async fn unstake_request(
		&self,
		amount: U256,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		Ok(contract.unstake(amount).from(self.signer_address).into_transaction_request())
	}
}

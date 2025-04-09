use super::{Client, MCRStorage, MOVEToken, MCR};
use alloy::providers::Provider;
use alloy_network::{Ethereum, Network};
use alloy_primitives::Address;
use alloy_rpc_types::Filter;
use mcr_protocol_client_core_util::{McrClientError, U256};
use mcr_types::commitment::Commitment;
use std::array::TryFromSliceError;

impl<R, W> Client<R, W>
where
	R: Provider + Clone,
	W: Provider + Clone,
{
	/// Form the request to submit a commitment to the MCR contract.
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

	/// Form the request to submit a batch of commitments to the MCR contract.
	pub fn submit_commitment_batch_request(
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

	/// Form the request to force a commitment to the MCR contract.
	pub fn force_commitment_request(
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

	/// Form the request to grant a trusted attester to the MCR contract.
	pub fn grant_trusted_attester_request(
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

	/// Form the request to stake an amount to the MCR contract.
	pub fn stake_request(
		&self,
		amount: U256,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		Ok(contract.stake(amount).from(self.signer_address).into_transaction_request())
	}

	/// Form the request to unstake an amount from the MCR contract.
	pub fn unstake_request(
		&self,
		amount: U256,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.rpc_provider);

		Ok(contract.unstake(amount).from(self.signer_address).into_transaction_request())
	}

	/// For the filter for streaming commitments.
	pub fn stream_commitments_filter(&self) -> Result<Filter, McrClientError> {
		// Register to contract CommitmentSubmitted event

		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let event = contract.CommitmentAccepted_filter();

		Ok(event.filter)
	}

	/// Form the request to get an accepted commitment at a given height.
	pub fn get_accepted_commitment_at_height_request(
		&self,
		height: u64,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		Ok(contract
			.getAcceptedCommitmentAtCommitmentHeight(U256::from(height))
			.into_transaction_request())
	}

	/// Form the request to get a posted commitment at a given height.
	pub fn get_posted_commitment_at_height_request(
		&self,
		height: u64,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		Ok(contract
			.getValidatorCommitmentAtHeight(U256::from(height), self.signer_address)
			.into_transaction_request())
	}

	/// Get max tolerable commitment height.
	pub fn get_max_tolerable_commitment_height_request(
		&self,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		Ok(contract.getMaxTolerableCommitmentHeight().into_transaction_request())
	}

	/// Get validator commitment at height.
	pub fn get_validator_commitment_at_height_request(
		&self,
		height: u64,
		attester: String,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(contract
			.getValidatorCommitmentAtHeight(U256::from(height), attester_addr)
			.into_transaction_request())
	}

	/// Get balance of MOVE-octas token.
	pub fn get_balance_request(
		&self,
		address: String,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let token = MOVEToken::new(self.move_token_address, &self.rpc_provider);
		let addr = address.parse::<Address>().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(token.balanceOf(addr).into_transaction_request())
	}

	/// Get last accepted commitment height.
	pub fn get_last_accepted_commitment_height_request(
		&self,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		Ok(contract.lastAcceptedCommitmentHeight().into_transaction_request())
	}

	/// Get leading commitment tolerance.
	pub fn get_leading_commitment_tolerance_request(
		&self,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		Ok(contract.leadingCommitmentTolerance().into_transaction_request())
	}

	/// Get current epoch stake for an attester.
	pub fn get_stake_request(
		&self,
		custodian: String,
		attester: String,
	) -> Result<<Ethereum as Network>::TransactionRequest, McrClientError> {
		let contract = MCR::new(self.contract_address, &self.ws_provider);
		let custodian_addr =
			custodian.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;
		let attester_addr = attester.parse().map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(contract
			.getCurrentEpochStake(custodian_addr, attester_addr)
			.into_transaction_request())
	}
}

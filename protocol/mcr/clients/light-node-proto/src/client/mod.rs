use alloy_primitives::U256;
use mcr_light_node_proto::mcr_light_node_service_client::McrLightNodeServiceClient;
use mcr_light_node_proto::v1beta2::*;
use mcr_light_node_proto::McrLightNodeProtoError;
use mcr_protocol_client_core_util::{CommitmentStream, McrClientError, McrClientOperations};
use mcr_types::commitment::Commitment;
use tonic::transport::Channel;
use tonic::transport::Endpoint;
#[derive(Debug, Clone)]
pub struct Client {
	pub client: McrLightNodeServiceClient<Channel>,
}

impl Client {
	/// Creates a new client connected to the specified endpoint.
	pub async fn connect<D>(dst: D) -> Result<Self, McrClientError>
	where
		D: TryInto<tonic::transport::Endpoint>,
		D::Error: Into<tonic::transport::Error> + Send + Sync + 'static,
		<D as TryInto<Endpoint>>::Error: std::error::Error + Send + Sync,
	{
		let client = McrLightNodeServiceClient::connect(dst)
			.await
			.map_err(|e| McrClientError::Internal(e.into()))?;
		Ok(Self { client })
	}
}

impl McrClientOperations for Client {
	async fn post_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let request = PostCommitmentRequest::try_from(commitment)
			.map_err(|e| McrClientError::PostCommitment(Box::new(e)))?;

		self.client
			.clone()
			.post_commitment(request)
			.await
			.map_err(|e| McrClientError::PostCommitment(Box::new(e)))?;

		Ok(())
	}

	async fn post_commitment_batch(
		&self,
		commitments: Vec<Commitment>,
	) -> Result<(), McrClientError> {
		let request = PostCommitmentBatchRequest::try_from(commitments)
			.map_err(|e| McrClientError::PostCommitment(Box::new(e)))?;

		self.client
			.clone()
			.post_commitment_batch(request)
			.await
			.map_err(|e| McrClientError::PostCommitment(Box::new(e)))?;

		Ok(())
	}

	async fn force_commitment(&self, commitment: Commitment) -> Result<(), McrClientError> {
		let request = ForceCommitmentRequest::try_from(commitment)
			.map_err(|e| McrClientError::AdminFunction(Box::new(e)))?;

		self.client
			.clone()
			.force_commitment(request)
			.await
			.map_err(|e| McrClientError::AdminFunction(Box::new(e)))?;

		Ok(())
	}

	async fn stream_commitments(&self) -> Result<CommitmentStream, McrClientError> {
		let mut stream = self
			.client
			.clone()
			.stream_commitments(StreamCommitmentsRequest {})
			.await
			.map_err(|e| McrClientError::StreamCommitments(Box::new(e)))?
			.into_inner();

		let stream = async_stream::stream! {
			while let Some(response) = stream.message().await.map_err(|e| McrClientError::StreamCommitments(Box::new(e)))? {
				yield response.try_into().map_err(|e: McrLightNodeProtoError| McrClientError::StreamCommitments(Box::new(e)));
			}
		};

		Ok(Box::pin(stream))
	}

	async fn get_accepted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		let request = GetAcceptedCommitmentAtHeightRequest { height };
		let response = self
			.client
			.clone()
			.get_accepted_commitment_at_height(request)
			.await
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))
	}

	async fn get_posted_commitment_at_height(
		&self,
		height: u64,
	) -> Result<Option<Commitment>, McrClientError> {
		let request = GetPostedCommitmentAtHeightRequest { height };
		let response = self
			.client
			.clone()
			.get_posted_commitment_at_height(request)
			.await
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))
	}

	async fn get_max_tolerable_commitment_height(&self) -> Result<u64, McrClientError> {
		let response = self
			.client
			.clone()
			.get_max_tolerable_commitment_height(GetMaxTolerableCommitmentHeightRequest {})
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::Internal(Box::new(e)))
	}

	async fn get_validator_commitment_at_height(
		&self,
		height: u64,
		attester: String,
	) -> Result<Option<Commitment>, McrClientError> {
		let request = GetValidatorCommitmentAtHeightRequest { height, attester };
		let response = self
			.client
			.clone()
			.get_validator_commitment_at_height(request)
			.await
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::GetCommitment(Box::new(e)))
	}

	async fn get_balance(&self, address: String) -> Result<u64, McrClientError> {
		let request = GetBalanceRequest::try_from(address)
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let response = self
			.client
			.clone()
			.get_balance(request)
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::Internal(Box::new(e)))
	}

	async fn get_last_accepted_block_height(&self) -> Result<u64, McrClientError> {
		let response = self
			.client
			.clone()
			.get_last_accepted_block_height(GetLastAcceptedBlockHeightRequest {})
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::Internal(Box::new(e)))
	}

	async fn get_leading_commitment_tolerance(&self) -> Result<u64, McrClientError> {
		let response = self
			.client
			.clone()
			.get_leading_commitment_tolerance(GetLeadingCommitmentToleranceRequest {})
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::Internal(Box::new(e)))
	}

	async fn grant_trusted_attester(&self, attester: String) -> Result<(), McrClientError> {
		let request = GrantTrustedAttesterRequest::try_from(attester)
			.map_err(|e| McrClientError::AdminFunction(Box::new(e)))?;

		self.client
			.clone()
			.grant_trusted_attester(request)
			.await
			.map_err(|e| McrClientError::AdminFunction(Box::new(e)))?;

		Ok(())
	}

	async fn stake(&self, amount: U256) -> Result<(), McrClientError> {
		let request =
			StakeRequest::try_from(amount).map_err(|e| McrClientError::Internal(Box::new(e)))?;

		self.client
			.clone()
			.stake(request)
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(())
	}

	async fn get_stake(&self, custodian: String, attester: String) -> Result<u64, McrClientError> {
		let request = GetStakeRequest::try_from((custodian, attester))
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		let response = self
			.client
			.clone()
			.get_stake(request)
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		response
			.into_inner()
			.try_into()
			.map_err(|e| McrClientError::Internal(Box::new(e)))
	}

	async fn unstake(&self, amount: U256) -> Result<(), McrClientError> {
		let request =
			UnstakeRequest::try_from(amount).map_err(|e| McrClientError::Internal(Box::new(e)))?;

		self.client
			.clone()
			.unstake(request)
			.await
			.map_err(|e| McrClientError::Internal(Box::new(e)))?;

		Ok(())
	}
}

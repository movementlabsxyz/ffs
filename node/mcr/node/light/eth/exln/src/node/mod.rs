use futures::StreamExt;
use mcr_light_node_proto::mcr_light_node_service_server::{
	McrLightNodeService, McrLightNodeServiceServer,
};
use mcr_light_node_proto::{
	ForceCommitmentRequest, ForceCommitmentResponse, GetAcceptedCommitmentAtHeightRequest,
	GetAcceptedCommitmentAtHeightResponse, GetBalanceRequest, GetBalanceResponse,
	GetLastAcceptedBlockHeightRequest, GetLastAcceptedBlockHeightResponse,
	GetLeadingCommitmentToleranceRequest, GetLeadingCommitmentToleranceResponse,
	GetMaxTolerableCommitmentHeightRequest, GetMaxTolerableCommitmentHeightResponse,
	GetPostedCommitmentAtHeightRequest, GetPostedCommitmentAtHeightResponse, GetStakeRequest,
	GetStakeResponse, GetValidatorCommitmentAtHeightRequest,
	GetValidatorCommitmentAtHeightResponse, GrantTrustedAttesterRequest,
	GrantTrustedAttesterResponse, McrLightNodeProtoError, PostCommitmentBatchRequest,
	PostCommitmentBatchResponse, PostCommitmentRequest, PostCommitmentResponse, StakeRequest,
	StakeResponse, StreamCommitmentsRequest, StreamCommitmentsResponse, UnstakeRequest,
	UnstakeResponse,
};
use mcr_protocol_client_core_util::{McrClientOperations, U256};
use mcr_protocol_client_eth_core::config::StandardClient;
use mcr_types::commitment::Commitment;
use std::pin::Pin;
use tokio_stream::Stream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub struct Node {
	pub(crate) client: StandardClient,
	pub(crate) address: String,
}

#[tonic::async_trait]
impl McrLightNodeService for Node {
	async fn post_commitment(
		&self,
		request: Request<PostCommitmentRequest>,
	) -> Result<Response<PostCommitmentResponse>, Status> {
		let commitment: Commitment = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client
			.post_commitment(commitment)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(PostCommitmentResponse {}))
	}

	async fn post_commitment_batch(
		&self,
		request: Request<PostCommitmentBatchRequest>,
	) -> Result<Response<PostCommitmentBatchResponse>, Status> {
		let commitments: Vec<Commitment> = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client
			.post_commitment_batch(commitments)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(PostCommitmentBatchResponse {}))
	}

	async fn force_commitment(
		&self,
		request: Request<ForceCommitmentRequest>,
	) -> Result<Response<ForceCommitmentResponse>, Status> {
		let commitment: Commitment = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client
			.force_commitment(commitment)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(ForceCommitmentResponse {}))
	}

	type StreamCommitmentsStream =
		Pin<Box<dyn Stream<Item = Result<StreamCommitmentsResponse, Status>> + Send>>;

	async fn stream_commitments(
		&self,
		_request: Request<StreamCommitmentsRequest>,
	) -> Result<Response<Self::StreamCommitmentsStream>, Status> {
		let stream = self
			.client
			.stream_commitments()
			.await
			.map_err(|e| Status::internal(e.to_string()))?;

		let output_stream =
			stream.map(|result| {
				result.map_err(|e| Status::internal(e.to_string())).and_then(|commitment| {
					Ok(StreamCommitmentsResponse {
						commitment: Some(commitment.try_into().map_err(
							|e: McrLightNodeProtoError| Status::internal(e.to_string()),
						)?),
					})
				})
			});

		Ok(Response::new(Box::pin(output_stream)))
	}

	async fn get_accepted_commitment_at_height(
		&self,
		request: Request<GetAcceptedCommitmentAtHeightRequest>,
	) -> Result<Response<GetAcceptedCommitmentAtHeightResponse>, Status> {
		let height = request.into_inner().height;
		let commitment: Option<Commitment> = self
			.client
			.get_accepted_commitment_at_height(height)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetAcceptedCommitmentAtHeightResponse = commitment
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_posted_commitment_at_height(
		&self,
		request: Request<GetPostedCommitmentAtHeightRequest>,
	) -> Result<Response<GetPostedCommitmentAtHeightResponse>, Status> {
		let height = request.into_inner().height;
		let commitment: Option<Commitment> = self
			.client
			.get_posted_commitment_at_height(height)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetPostedCommitmentAtHeightResponse = commitment
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_max_tolerable_commitment_height(
		&self,
		_request: Request<GetMaxTolerableCommitmentHeightRequest>,
	) -> Result<Response<GetMaxTolerableCommitmentHeightResponse>, Status> {
		let height: u64 = self
			.client
			.get_max_tolerable_commitment_height()
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetMaxTolerableCommitmentHeightResponse = height
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_validator_commitment_at_height(
		&self,
		request: Request<GetValidatorCommitmentAtHeightRequest>,
	) -> Result<Response<GetValidatorCommitmentAtHeightResponse>, Status> {
		let req = request.into_inner();
		let commitment: Option<Commitment> = self
			.client
			.get_validator_commitment_at_height(req.height, req.attester)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetValidatorCommitmentAtHeightResponse = commitment
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_balance(
		&self,
		request: Request<GetBalanceRequest>,
	) -> Result<Response<GetBalanceResponse>, Status> {
		let address: String = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		let balance: u64 = self
			.client
			.get_balance(address)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetBalanceResponse = balance
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_last_accepted_block_height(
		&self,
		_request: Request<GetLastAcceptedBlockHeightRequest>,
	) -> Result<Response<GetLastAcceptedBlockHeightResponse>, Status> {
		let height: u64 = self
			.client
			.get_last_accepted_block_height()
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetLastAcceptedBlockHeightResponse = height
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn get_leading_commitment_tolerance(
		&self,
		_request: Request<GetLeadingCommitmentToleranceRequest>,
	) -> Result<Response<GetLeadingCommitmentToleranceResponse>, Status> {
		let height: u64 = self
			.client
			.get_leading_commitment_tolerance()
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetLeadingCommitmentToleranceResponse = height
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn grant_trusted_attester(
		&self,
		request: Request<GrantTrustedAttesterRequest>,
	) -> Result<Response<GrantTrustedAttesterResponse>, Status> {
		let attester: String = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client
			.grant_trusted_attester(attester)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(GrantTrustedAttesterResponse {}))
	}

	async fn stake(
		&self,
		request: Request<StakeRequest>,
	) -> Result<Response<StakeResponse>, Status> {
		let amount: U256 = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client.stake(amount).await.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(StakeResponse {}))
	}

	async fn get_stake(
		&self,
		request: Request<GetStakeRequest>,
	) -> Result<Response<GetStakeResponse>, Status> {
		let (custodian, attester): (String, String) = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		let stake: u64 = self
			.client
			.get_stake(custodian, attester)
			.await
			.map_err(|e| Status::internal(e.to_string()))?;
		let response: GetStakeResponse = stake
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		Ok(Response::new(response))
	}

	async fn unstake(
		&self,
		request: Request<UnstakeRequest>,
	) -> Result<Response<UnstakeResponse>, Status> {
		let amount: U256 = request
			.into_inner()
			.try_into()
			.map_err(|e: McrLightNodeProtoError| Status::internal(e.to_string()))?;
		self.client.unstake(amount).await.map_err(|e| Status::internal(e.to_string()))?;
		Ok(Response::new(UnstakeResponse {}))
	}
}

impl Node {
	pub async fn run(self) -> Result<(), anyhow::Error> {
		let reflection = tonic_reflection::server::Builder::configure()
			.register_encoded_file_descriptor_set(mcr_light_node_proto::FILE_DESCRIPTOR_SET)
			.build_v1()?;
		let address = self.address.clone();

		Server::builder()
			.max_frame_size(1024 * 1024 * 16 - 1)
			.accept_http1(true)
			.add_service(McrLightNodeServiceServer::new(self))
			.add_service(reflection)
			.serve(address.parse()?)
			.await?;

		Ok(())
	}
}

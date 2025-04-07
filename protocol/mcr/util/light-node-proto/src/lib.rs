pub mod v1beta2 {
	tonic::include_proto!("movementlabsxyz.ffs.mcr.light_node.v1beta1"); // The string specified here
	pub const FILE_DESCRIPTOR_SET: &[u8] =
		tonic::include_file_descriptor_set!("mcr-light-node-proto-descriptor");
}

// Re-export the latest version at the crate root
pub use v1beta2::*;

/// Errors thrown by the McrClient.
#[derive(Debug, thiserror::Error)]
pub enum McrLightNodeProtoError {
	#[error("failed to convert from proto to domain type: {0}")]
	ConvertToDomain(#[source] Box<dyn std::error::Error + Send + Sync>),
	#[error("failed to convert from domain to proto type: {0}")]
	ConvertToProto(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl TryFrom<Commitment> for mcr_types::commitment::Commitment {
	type Error = McrLightNodeProtoError;
	fn try_from(value: Commitment) -> Result<Self, Self::Error> {
		// Get the commitment value.
		let vote = mcr_types::commitment::Vote::new(value.vote.try_into().map_err(|e| {
			McrLightNodeProtoError::ConvertToDomain(
				format!("failed to convert from proto to domain type: {:?}", e).into(),
			)
		})?);

		// Get the commitment id.
		let id = mcr_types::commitment::Id::new(value.id.try_into().map_err(|e| {
			McrLightNodeProtoError::ConvertToDomain(
				format!("failed to convert from proto to domain type: {:?}", e).into(),
			)
		})?);

		// Get the commitment height.
		let height = value.height;

		Ok(mcr_types::commitment::Commitment::new(height, id, vote))
	}
}

impl TryFrom<mcr_types::commitment::Commitment> for Commitment {
	type Error = McrLightNodeProtoError;
	fn try_from(value: mcr_types::commitment::Commitment) -> Result<Self, Self::Error> {
		Ok(Commitment {
			height: value.height().into(),
			id: (*value.id()).into(),
			vote: (*value.vote()).into(),
		})
	}
}

// Convert PostCommitmentRequest
impl TryFrom<PostCommitmentRequest> for mcr_types::commitment::Commitment {
	type Error = McrLightNodeProtoError;
	fn try_from(value: PostCommitmentRequest) -> Result<Self, Self::Error> {
		value
			.commitment
			.ok_or_else(|| {
				McrLightNodeProtoError::ConvertToDomain(
					"missing commitment in PostCommitmentRequest".into(),
				)
			})?
			.try_into()
	}
}

impl TryFrom<mcr_types::commitment::Commitment> for PostCommitmentRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: mcr_types::commitment::Commitment) -> Result<Self, Self::Error> {
		Ok(PostCommitmentRequest { commitment: Some(value.try_into()?) })
	}
}

// Convert PostCommitmentBatchRequest
impl TryFrom<PostCommitmentBatchRequest> for Vec<mcr_types::commitment::Commitment> {
	type Error = McrLightNodeProtoError;
	fn try_from(value: PostCommitmentBatchRequest) -> Result<Self, Self::Error> {
		value.commitments.into_iter().map(|c| c.try_into()).collect()
	}
}

impl TryFrom<Vec<mcr_types::commitment::Commitment>> for PostCommitmentBatchRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: Vec<mcr_types::commitment::Commitment>) -> Result<Self, Self::Error> {
		Ok(PostCommitmentBatchRequest {
			commitments: value.into_iter().map(|c| c.try_into()).collect::<Result<Vec<_>, _>>()?,
		})
	}
}

// Convert GetCommitmentResponse
impl TryFrom<GetCommitmentResponse> for Option<mcr_types::commitment::Commitment> {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetCommitmentResponse) -> Result<Self, Self::Error> {
		Ok(value.commitment.map(|c| c.try_into()).transpose()?)
	}
}

impl TryFrom<Option<mcr_types::commitment::Commitment>> for GetCommitmentResponse {
	type Error = McrLightNodeProtoError;
	fn try_from(value: Option<mcr_types::commitment::Commitment>) -> Result<Self, Self::Error> {
		Ok(GetCommitmentResponse { commitment: value.map(|c| c.try_into()).transpose()? })
	}
}

// Convert CommitmentStreamResponse
impl TryFrom<CommitmentStreamResponse> for mcr_types::commitment::Commitment {
	type Error = McrLightNodeProtoError;
	fn try_from(value: CommitmentStreamResponse) -> Result<Self, Self::Error> {
		value
			.commitment
			.ok_or_else(|| {
				McrLightNodeProtoError::ConvertToDomain(
					"missing commitment in CommitmentStreamResponse".into(),
				)
			})?
			.try_into()
	}
}

impl TryFrom<mcr_types::commitment::Commitment> for CommitmentStreamResponse {
	type Error = McrLightNodeProtoError;
	fn try_from(value: mcr_types::commitment::Commitment) -> Result<Self, Self::Error> {
		Ok(CommitmentStreamResponse { commitment: Some(value.try_into()?) })
	}
}

// Convert StakeRequest/UnstakeRequest
impl TryFrom<StakeRequest> for alloy_primitives::U256 {
	type Error = McrLightNodeProtoError;
	fn try_from(value: StakeRequest) -> Result<Self, Self::Error> {
		alloy_primitives::U256::from_str_radix(&value.amount, 10)
			.map_err(|e| McrLightNodeProtoError::ConvertToDomain(Box::new(e)))
	}
}

impl TryFrom<alloy_primitives::U256> for StakeRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: alloy_primitives::U256) -> Result<Self, Self::Error> {
		Ok(StakeRequest { amount: value.to_string() })
	}
}

impl TryFrom<UnstakeRequest> for alloy_primitives::U256 {
	type Error = McrLightNodeProtoError;
	fn try_from(value: UnstakeRequest) -> Result<Self, Self::Error> {
		alloy_primitives::U256::from_str_radix(&value.amount, 10)
			.map_err(|e| McrLightNodeProtoError::ConvertToDomain(Box::new(e)))
	}
}

impl TryFrom<alloy_primitives::U256> for UnstakeRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: alloy_primitives::U256) -> Result<Self, Self::Error> {
		Ok(UnstakeRequest { amount: value.to_string() })
	}
}

// Convert GetStakeRequest/Response
impl TryFrom<GetStakeRequest> for (String, String) {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetStakeRequest) -> Result<Self, Self::Error> {
		Ok((value.custodian, value.attester))
	}
}

impl TryFrom<(String, String)> for GetStakeRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: (String, String)) -> Result<Self, Self::Error> {
		Ok(GetStakeRequest { custodian: value.0, attester: value.1 })
	}
}

impl TryFrom<GetStakeResponse> for u64 {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetStakeResponse) -> Result<Self, Self::Error> {
		Ok(value.stake)
	}
}

impl TryFrom<u64> for GetStakeResponse {
	type Error = McrLightNodeProtoError;
	fn try_from(value: u64) -> Result<Self, Self::Error> {
		Ok(GetStakeResponse { stake: value })
	}
}

// Convert GetHeightResponse
impl TryFrom<GetHeightResponse> for u64 {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetHeightResponse) -> Result<Self, Self::Error> {
		Ok(value.height)
	}
}

impl TryFrom<u64> for GetHeightResponse {
	type Error = McrLightNodeProtoError;
	fn try_from(value: u64) -> Result<Self, Self::Error> {
		Ok(GetHeightResponse { height: value })
	}
}

// Convert GetValidatorCommitmentRequest
impl TryFrom<GetValidatorCommitmentRequest> for (u64, String) {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetValidatorCommitmentRequest) -> Result<Self, Self::Error> {
		Ok((value.height, value.attester))
	}
}

impl TryFrom<(u64, String)> for GetValidatorCommitmentRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: (u64, String)) -> Result<Self, Self::Error> {
		Ok(GetValidatorCommitmentRequest { height: value.0, attester: value.1 })
	}
}

// Convert GetBalanceRequest/Response
impl TryFrom<GetBalanceRequest> for String {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetBalanceRequest) -> Result<Self, Self::Error> {
		Ok(value.address)
	}
}

impl TryFrom<String> for GetBalanceRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Ok(GetBalanceRequest { address: value })
	}
}

impl TryFrom<GetBalanceResponse> for u64 {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GetBalanceResponse) -> Result<Self, Self::Error> {
		Ok(value.balance)
	}
}

impl TryFrom<u64> for GetBalanceResponse {
	type Error = McrLightNodeProtoError;
	fn try_from(value: u64) -> Result<Self, Self::Error> {
		Ok(GetBalanceResponse { balance: value })
	}
}

// Convert GrantTrustedAttesterRequest
impl TryFrom<GrantTrustedAttesterRequest> for String {
	type Error = McrLightNodeProtoError;
	fn try_from(value: GrantTrustedAttesterRequest) -> Result<Self, Self::Error> {
		Ok(value.attester)
	}
}

impl TryFrom<String> for GrantTrustedAttesterRequest {
	type Error = McrLightNodeProtoError;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		Ok(GrantTrustedAttesterRequest { attester: value })
	}
}

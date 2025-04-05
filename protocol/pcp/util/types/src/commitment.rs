use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
	Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct CommitmentId([u8; 32]);

impl CommitmentId {
	pub fn new(data: [u8; 32]) -> Self {
		Self(data)
	}

	pub fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}

	pub fn test() -> Self {
		Self([0; 32])
	}

	pub fn to_vec(&self) -> Vec<u8> {
		self.0.into()
	}

	pub fn genesis_block() -> Self {
		Self([0; 32])
	}
}

impl AsRef<[u8]> for CommitmentId {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl fmt::Display for CommitmentId {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for byte in &self.0 {
			write!(f, "{:02x}", byte)?;
		}
		Ok(())
	}
}

#[derive(
	Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct CommitmentValue([u8; 32]);

impl CommitmentValue {
	pub fn new(data: [u8; 32]) -> Self {
		Self(data)
	}

	pub fn test() -> Self {
		Self([0; 32])
	}

	pub fn as_bytes(&self) -> &[u8; 32] {
		&self.0
	}
}

impl fmt::Display for CommitmentValue {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for byte in &self.0 {
			write!(f, "{:02x}", byte)?;
		}
		Ok(())
	}
}

impl From<CommitmentValue> for [u8; 32] {
	fn from(commitment: CommitmentValue) -> [u8; 32] {
		commitment.0
	}
}

impl From<CommitmentValue> for Vec<u8> {
	fn from(commitment: CommitmentValue) -> Vec<u8> {
		commitment.0.into()
	}
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Commitment {
	height: u64,
	commitment_id: CommitmentId,
	commitment_value: CommitmentValue,
}

impl Commitment {
	pub fn new(height: u64, commitment_id: CommitmentId, commitment_value: CommitmentValue) -> Self {
		Self { height, commitment_id, commitment_value }
	}

	pub fn height(&self) -> u64 {
		self.height
	}

	pub fn commitment_id(&self) -> &CommitmentId {
		&self.commitment_id
	}

	pub fn commitment_value(&self) -> CommitmentValue {
		self.commitment_value
	}

	pub fn test() -> Self {
		Self::new(0, CommitmentId::test(), CommitmentValue::test())
	}
}

impl fmt::Display for Commitment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Commitment {{ height: {}, commitment_id: {}, commitment_value: {} }}",
			self.height, self.commitment_id, self.commitment_value
		)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommitmentRejectionReason {
	InvalidCommitmentId,
	InvalidCommitmentValue,
	InvalidHeight,
	ContractError,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommitmentEvent {
	Accepted(Commitment),
	Rejected { height: u64, reason: CommitmentRejectionReason },
}

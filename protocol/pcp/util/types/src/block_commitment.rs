use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
	Serialize, Deserialize, Clone, Copy, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct Id([u8; 32]);

impl Id {
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

impl AsRef<[u8]> for Id {
	fn as_ref(&self) -> &[u8] {
		&self.0
	}
}

impl fmt::Display for Id {
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
pub struct Commitment([u8; 32]);

impl Commitment {
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

impl fmt::Display for Commitment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for byte in &self.0 {
			write!(f, "{:02x}", byte)?;
		}
		Ok(())
	}
}

impl From<Commitment> for [u8; 32] {
	fn from(commitment: Commitment) -> [u8; 32] {
		commitment.0
	}
}

impl From<Commitment> for Vec<u8> {
	fn from(commitment: Commitment) -> Vec<u8> {
		commitment.0.into()
	}
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SuperBlockCommitment {
	height: u64,
	block_id: Id,
	commitment: Commitment,
}

impl SuperBlockCommitment {
	pub fn new(height: u64, block_id: Id, commitment: Commitment) -> Self {
		Self { height, block_id, commitment }
	}

	pub fn height(&self) -> u64 {
		self.height
	}

	pub fn block_id(&self) -> &Id {
		&self.block_id
	}

	pub fn commitment(&self) -> Commitment {
		self.commitment
	}

	pub fn test() -> Self {
		Self::new(0, Id::test(), Commitment::test())
	}
}

impl fmt::Display for SuperBlockCommitment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"SuperBlockCommitment {{ height: {}, block_id: {}, commitment: {} }}",
			self.height, self.block_id, self.commitment
		)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuperBlockCommitmentRejectionReason {
	InvalidBlockId,
	InvalidCommitment,
	InvalidHeight,
	ContractError,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SuperBlockCommitmentEvent {
	Accepted(SuperBlockCommitment),
	Rejected { height: u64, reason: SuperBlockCommitmentRejectionReason },
}

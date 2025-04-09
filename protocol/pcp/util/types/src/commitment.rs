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
pub struct Vote([u8; 32]);

impl Vote {
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

impl fmt::Display for Vote {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		for byte in &self.0 {
			write!(f, "{:02x}", byte)?;
		}
		Ok(())
	}
}

impl From<Vote> for [u8; 32] {
	fn from(commitment: Vote) -> [u8; 32] {
		commitment.0
	}
}

impl From<Vote> for Vec<u8> {
	fn from(commitment: Vote) -> Vec<u8> {
		commitment.0.into()
	}
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Commitment {
	height: u64,
	id: Id,
	vote: Vote,
}

impl Commitment {
	pub fn new(height: u64, id: Id, vote: Vote) -> Self {
		Self { height, id, vote }
	}

	pub fn height(&self) -> u64 {
		self.height
	}

	pub fn id(&self) -> &Id {
		&self.id
	}

	pub fn vote(&self) -> Vote {
		self.vote
	}

	pub fn test() -> Self {
		Self::new(0, Id::test(), Vote::test())
	}
}

impl fmt::Display for Commitment {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"Commitment {{ height: {}, id: {}, vote: {} }}",
			self.height, self.id, self.vote
		)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommitmentRejectionReason {
	InvalidId,
	InvalidVote,
	InvalidHeight,
	ContractError,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CommitmentEvent {
	Accepted(Commitment),
	Rejected { height: u64, reason: CommitmentRejectionReason },
}

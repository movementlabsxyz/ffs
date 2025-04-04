use clap::Parser;
use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum McrRewardContract {
	None,
	Aro,
	Address(String),
}

impl FromStr for McrRewardContract {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "aro" => Ok(Self::Aro),
            addr if addr.starts_with("0x") => Ok(Self::Address(addr.to_string())),
            _ => Err(format!("Invalid reward contract: {}. Must be 'none', 'aro', or an address starting with '0x'", s)),
        }
	}
}

impl McrRewardContract {
	pub fn reward_option(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::Aro => 1,
			Self::Address(_addr) => 2,
		}
	}

	pub fn reward_contract(&self) -> Option<String> {
		match self {
			Self::Address(addr) => Some(addr.clone()),
			_ => None,
		}
	}
}

impl Default for McrRewardContract {
	fn default() -> Self {
		Self::None
	}
}

/// Configuration for MCR deployment, matching the Solidity struct
#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
pub struct Arguments {
	/// Admin address for deployed contracts
	#[arg(long)]
	pub contract_admin: String,

	/// The token name
	#[arg(long, default_value = "Move Token")]
	pub token_name: String,

	/// The token symbol
	#[arg(long, default_value = "MOVE")]
	pub token_symbol: String,

	/// The initial token mint
	#[arg(long, default_value = "1000000000000000000000000")]
	pub initial_token_mint: String,

	/// The custodians
	/// By default this should be an empty vector
	#[arg(long)]
	pub custodians: Option<Vec<String>>,

	/// The initial block height
	#[arg(long, default_value = "1")]
	pub initial_commitment_height: String,

	/// The leading block tolerance
	#[arg(long, default_value = "10")]
	pub leading_commitment_tolerance: String,

	/// The epoch duration
	#[arg(long, default_value = "1000000")]
	pub epoch_duration: String,

	/// The reward contract
	#[arg(long)]
	pub reward_contract: Option<McrRewardContract>,
}

impl Arguments {
	/// Returns a new instance of [Arguments] which is designed to be filled in some fields.
	pub fn to_be_filled() -> Self {
		Self {
			contract_admin: "".to_string(),
			token_name: "Move Token".to_string(),
			token_symbol: "MOVE".to_string(),
			initial_token_mint: "1000000000000000000000000".to_string(),
			custodians: None,
			initial_commitment_height: "1".to_string(),
			leading_commitment_tolerance: "10".to_string(),
			epoch_duration: "1000000".to_string(),
			reward_contract: None,
		}
	}
}

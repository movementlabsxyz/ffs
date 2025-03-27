use clap::Parser;
use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
pub struct Artifacts {
	pub proxy_admin: String,
	pub move_token_proxy: String,
	pub staking_proxy: String,
	pub mcr_proxy: String,
	pub reward_proxy: String,
}

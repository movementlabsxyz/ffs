use clap::Parser;
use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(help_expected = true)]
pub struct Artifacts {
	/// The proxy admin
	#[arg(long)]
	pub proxy_admin: String,
	/// The move token proxy
	#[arg(long)]
	pub token_proxy: String,
	/// The staking proxy
	#[arg(long)]
	pub staking_proxy: String,
	/// The mcr proxy
	#[arg(long)]
	pub mcr_proxy: String,
	/// The reward proxy
	#[arg(long)]
	pub reward_proxy: String,
}

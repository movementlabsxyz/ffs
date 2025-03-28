use clap::Parser;
use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
#[clap(rename_all = "kebab-case")]
#[clap(help_expected = true)]
pub struct Artifacts {
	#[arg(long)]
	pub proxy_admin: String,
	#[arg(long)]
	pub move_token_proxy: String,
	#[arg(long)]
	pub staking_proxy: String,
	#[arg(long)]
	pub mcr_proxy: String,
	#[arg(long)]
	pub reward_proxy: String,
}

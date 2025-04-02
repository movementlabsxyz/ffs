use super::output::Artifacts as OutputArtifacts;
use clap::Parser;
use jsonlvar::Jsonl;
use lifecycle::LifecycleError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
pub struct Artifacts {
	/// The existing proxy admin
	#[arg(long)]
	pub proxy_admin: Option<String>,

	/// The existing move token proxy
	#[arg(long)]
	pub token_proxy: Option<String>,

	/// The existing staking proxy
	#[arg(long)]
	pub staking_proxy: Option<String>,

	/// The existing MCR proxy
	#[arg(long)]
	pub mcr_proxy: Option<String>,

	/// The existing ARO proxy
	#[arg(long)]
	pub reward_proxy: Option<String>,
}

impl Default for Artifacts {
	fn default() -> Self {
		Self {
			proxy_admin: None,
			token_proxy: None,
			staking_proxy: None,
			mcr_proxy: None,
			reward_proxy: None,
		}
	}
}

impl TryFrom<OutputArtifacts> for Artifacts {
	type Error = LifecycleError;

	fn try_from(artifacts: OutputArtifacts) -> Result<Self, Self::Error> {
		Ok(Self {
			proxy_admin: Some(artifacts.proxy_admin),
			token_proxy: Some(artifacts.token_proxy),
			staking_proxy: Some(artifacts.staking_proxy),
			mcr_proxy: Some(artifacts.mcr_proxy),
			reward_proxy: Some(artifacts.reward_proxy),
		})
	}
}

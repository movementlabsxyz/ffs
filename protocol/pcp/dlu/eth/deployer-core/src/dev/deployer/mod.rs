use crate::contracts::ContractWorkspace;
use crate::dev::artifacts::Artifacts;
use clap::Parser;
use jsonlvar::Jsonl;
use serde::{Deserialize, Serialize};
use serde_json;
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PcpRewardContract {
	None,
	Aro,
	Address(String),
}

impl FromStr for PcpRewardContract {
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

impl PcpRewardContract {
	pub fn reward_option(&self) -> u8 {
		match self {
			Self::None => 0,
			Self::Aro => 1,
			Self::Address(_addr) => 2,
		}
	}

	pub fn existing_reward_contract(&self) -> Option<String> {
		match self {
			Self::Address(addr) => Some(addr.clone()),
			_ => None,
		}
	}

	pub fn reward_option_forge_arg(&self) -> String {
		format!("{}", self.reward_option())
	}

	pub fn existing_reward_contract_forge_arg(&self) -> String {
		match self.existing_reward_contract() {
			Some(addr) => format!("\"{}\"", addr),
			None => "0x0".to_string(),
		}
	}
}

impl Default for PcpRewardContract {
	fn default() -> Self {
		Self::None
	}
}

/// Configuration for PCP deployment, matching the Solidity struct
#[derive(Debug, Clone, Serialize, Deserialize, Jsonl, Parser)]
pub struct DeployConfig {
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
	pub initial_block_height: String,

	/// The leading block tolerance
	#[arg(long, default_value = "10")]
	pub leading_block_tolerance: String,

	/// The epoch duration
	#[arg(long, default_value = "1000000")]
	pub epoch_duration: String,

	/// The reward contract
	#[arg(long)]
	pub reward_contract: Option<PcpRewardContract>,

	/// The existing proxy admin
	#[arg(long)]
	pub existing_proxy_admin: Option<String>,

	/// The existing move token proxy
	#[arg(long)]
	pub existing_token_proxy: Option<String>,

	/// The existing staking proxy
	#[arg(long)]
	pub existing_staking_proxy: Option<String>,

	/// The existing PCP proxy
	#[arg(long)]
	pub existing_pcp_proxy: Option<String>,

	/// The existing ARO proxy
	#[arg(long)]
	pub existing_reward_proxy: Option<String>,

	/// Whether to destroy the contracts
	#[arg(long, default_value = "false")]
	pub destroy_mode: bool,
}

impl DeployConfig {
	pub fn to_forge_arg_string(&self) -> String {
		let reward_contract = self.reward_contract.clone().unwrap_or_default();
		let zero_address = "0x0000000000000000000000000000000000000000";

		// Create a JSON object that matches the Solidity struct's expected format
		let json = serde_json::json!({
			"contractAdmin": self.contract_admin,
			"tokenName": self.token_name,
			"tokenSymbol": self.token_symbol,
			"initialTokenMint": self.initial_token_mint,
			"custodians": self.custodians.as_ref().map_or(Vec::new(), |v| v.clone()),
			"initialBlockHeight": self.initial_block_height,
			"leadingBlockTolerance": self.leading_block_tolerance,
			"epochDuration": self.epoch_duration,
			"rewardOption": reward_contract.reward_option(),
			"existingRewardContract": reward_contract.existing_reward_contract().unwrap_or_else(|| zero_address.to_string()),
			"existingProxyAdmin": self.existing_proxy_admin.as_deref().unwrap_or(zero_address),
			"existingMoveTokenProxy": self.existing_token_proxy.as_deref().unwrap_or(zero_address),
			"existingStakingProxy": self.existing_staking_proxy.as_deref().unwrap_or(zero_address),
			"existingPcpProxy": self.existing_pcp_proxy.as_deref().unwrap_or(zero_address),
			"existingAroProxy": self.existing_reward_proxy.as_deref().unwrap_or(zero_address),
			"destroyMode": self.destroy_mode
		});

		// Convert to a compact JSON string
		let json_str = json.to_string();
		json_str
	}
}

/// The deployer of PCP dev contracts.
#[derive(Debug)]
pub struct Deployer {
	/// The contract workspace in which the deployment command will run.
	pub workspace: ContractWorkspace,
	/// The raw private key used for deployment.
	pub raw_private_key: String,
	/// The fork url used for deployment.
	pub fork_url: String,
	/// The deployment configuration.
	pub config: DeployConfig,
	/// The jsonl prefix to give to the output from the deployer.
	pub jsonl_prefix: Option<String>,
}

impl Deployer {
	/// The constant path to the deployment script from the workspace.
	pub const DEPLOYMENT_SCRIPT_PATH: &'static str = "./script/DeployPCPDev.s.sol";

	/// Deploys the PCP dev contracts.
	pub async fn deploy(&self) -> Result<Artifacts, anyhow::Error> {
		// prepare the workspace directory
		self.workspace.prepare_directory()?;

		// Build command arguments
		let mut args = vec![
			"script",
			Self::DEPLOYMENT_SCRIPT_PATH,
			"--tc",
			"DeployPCPDev",
			"--sig",
			"run(string)",
		];

		// Build the JSON config string
		let config_str = self.config.to_forge_arg_string();
		args.push(&config_str);

		// Add common parameters
		args.extend_from_slice(&[
			"--private-key",
			&self.raw_private_key,
			"--fork-url",
			&self.fork_url,
			"--broadcast",
		]);

		// run the command for deployment
		let log_output = self.workspace.run_command("forge", args).await?;

		// Parse the output
		Ok(Artifacts::try_from_jsonl(&log_output, self.jsonl_prefix.as_deref())?)
	}
}

use super::{
	super::{artifacts, ForgeDeployer},
	Arguments,
};
use clap::Parser;
use jsonlvar::Jsonl;
use lifecycle::{ApplyOperations, LifecycleError};
use serde::{Deserialize, Serialize};

/// The arguments to be passed to the Forge Apply script.
#[derive(Debug, Clone, Serialize, Deserialize, Parser)]
pub struct ScriptArgs {
	#[clap(flatten)]
	pub args: Arguments,
	#[clap(flatten)]
	pub artifacts: artifacts::input::Artifacts,
}

impl ScriptArgs {
	pub fn to_forge_arg_string(&self) -> String {
		let reward_contract = self.args.reward_contract.clone().unwrap_or_default();
		let zero_address = "0x0000000000000000000000000000000000000000";

		// Create a JSON object that matches the Solidity struct's expected format
		let json = serde_json::json!({
			"contractAdmin": self.args.contract_admin,
			"tokenName": self.args.token_name,
			"tokenSymbol": self.args.token_symbol,
			"initialTokenMint": self.args.initial_token_mint,
			"custodians": self.args.custodians.as_ref().map_or(Vec::new(), |v| v.clone()),
			"initialBlockHeight": self.args.initial_block_height,
			"leadingCommitmentTolerance": self.args.leading_block_tolerance,
			"epochDuration": self.args.epoch_duration,
			"rewardOption": reward_contract.reward_option(),
			"existingRewardContract": reward_contract.reward_contract().unwrap_or_else(|| zero_address.to_string()),
			"existingProxyAdmin": self.artifacts.proxy_admin.as_deref().unwrap_or(zero_address),
			"existingMoveTokenProxy": self.artifacts.token_proxy.as_deref().unwrap_or(zero_address),
			"existingStakingProxy": self.artifacts.staking_proxy.as_deref().unwrap_or(zero_address),
			"existingMcrProxy": self.artifacts.mcr_proxy.as_deref().unwrap_or(zero_address),
			"existingAroProxy": self.artifacts.reward_proxy.as_deref().unwrap_or(zero_address),
		});

		// Convert to a compact JSON string
		let json_str = json.to_string();
		json_str
	}

	/// Returns a new instance of [ScriptArgs] which is designed to be filled in some fields.
	pub fn to_be_filled() -> Self {
		Self { args: Arguments::to_be_filled(), artifacts: artifacts::input::Artifacts::default() }
	}
}

/// The Applier struct.
#[derive(Debug, Clone)]
pub struct Applier {
	/// The arguments to be passed to the Forge Apply script.
	pub(crate) script_args: ScriptArgs,
	/// The [ForgeDeployer] instance.
	pub(crate) forge_deployer: ForgeDeployer,
}

impl Applier {
	/// The constant path to the deployment script from the workspace.
	pub const DEPLOYMENT_SCRIPT_PATH: &'static str = "./script/DeployMCRDev.s.sol";
}

impl ApplyOperations for Applier {
	type ApplyArguments = Arguments;
	type InputArtifacts = artifacts::input::Artifacts;
	type OutputArtifacts = artifacts::output::Artifacts;

	async fn apply(&self) -> Result<Self::OutputArtifacts, LifecycleError> {
		// prepare the workspace directory
		self.forge_deployer
			.workspace
			.prepare_directory()
			.map_err(|e| LifecycleError::Apply(e.into()))?;

		// Build command arguments
		let mut args = vec![
			"script",
			Self::DEPLOYMENT_SCRIPT_PATH,
			"--tc",
			"DeployMCRDev",
			"--sig",
			"run(string)",
		];

		// Build the JSON config string
		let config_str = self.script_args.to_forge_arg_string();
		args.push(&config_str);

		// Add common parameters
		args.extend_from_slice(&[
			"--private-key",
			&self.forge_deployer.raw_private_key,
			"--fork-url",
			&self.forge_deployer.fork_url,
			"--broadcast",
		]);

		// run the command for deployment
		let log_output = self
			.forge_deployer
			.workspace
			.run_command("forge", args)
			.await
			.map_err(|e| LifecycleError::Apply(e.into()))?;

		// Parse the output
		Ok(Self::OutputArtifacts::try_from_jsonl(
			&log_output,
			self.forge_deployer.jsonl_prefix.as_deref(),
		)
		.map_err(|e| LifecycleError::Apply(e.into()))?)
	}
}

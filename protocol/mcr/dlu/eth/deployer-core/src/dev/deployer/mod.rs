use crate::contracts::ContractWorkspace;

/// The deployer of MCR dev contracts.
#[derive(Debug)]
pub struct Deployer {
	/// The contract workspace in which the deployment command will run.
	pub workspace: ContractWorkspace,
	/// The raw private key used for deployment.
	pub raw_private_key: String,
	/// The fork url used for deployment.
	pub fork_url: String,
	/// The contractAdmin address used in deployment.
	pub contract_admin: String,
}

impl Deployer {
	/// The constant path to the deployment script from the workspace.
	pub const DEPLOYMENT_SCRIPT_PATH: &'static str = "./script/DeployMCRDev.s.sol";

	/// Deploys the MCR dev contracts.
	pub async fn deploy(&self) -> Result<(), anyhow::Error> {
		// prepare the workspace directory
		self.workspace.prepare_directory()?;

		// run the command for deployment
		// todo: we would like to migrate this to use an embedded version of forge
		self.workspace
			.run_command(
				"forge",
				vec![
					"script",
					Self::DEPLOYMENT_SCRIPT_PATH,
					"--sig",
					"run(address)",
					&self.contract_admin,
					"--private-key",
					&self.raw_private_key,
					"--fork-url",
					&self.fork_url,
					"--broadcast",
				],
			)
			.await?;

		// todo: parse the output

		Ok(())
	}
}

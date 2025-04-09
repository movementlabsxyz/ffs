use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, U256};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct StakeArgs {
	/// Amount to stake
	#[clap(long)]
	amount: U256,
}

pub struct StakeHelper<C: McrConfigOperations> {
	config: C,
	args: StakeArgs,
}

impl<C: McrConfigOperations + Clone> StakeHelper<C> {
	pub fn new(config: C, args: StakeArgs) -> Self {
		Self { config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;
		client.stake(self.args.amount).await?;
		println!("Successfully staked {} tokens", self.args.amount);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_stake {
	($config:ty) => {
		use crate::cli::standard::stake::{StakeArgs, StakeHelper};
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Stake tokens for the MCR domain
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct Stake {
			/// Path to the configuration file
			#[orfile(config)]
			/// The config to use (this requires a specific account).
			#[clap(flatten)]
			pub config: $config,
			/// The arguments for staking
			#[clap(flatten)]
			args: StakeArgs,
		}

		impl Stake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = StakeHelper::new(self.config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::Stake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

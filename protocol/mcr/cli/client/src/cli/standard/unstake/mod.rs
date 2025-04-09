use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, U256};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct UnstakeArgs {
	/// Amount to unstake
	#[clap(long)]
	amount: U256,
}

pub struct UnstakeHelper<C: McrConfigOperations> {
	config: C,
	args: UnstakeArgs,
}

impl<C: McrConfigOperations + Clone> UnstakeHelper<C> {
	pub fn new(config: C, args: UnstakeArgs) -> Self {
		Self { config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;
		client.unstake(self.args.amount).await?;
		println!("Successfully unstaked {} tokens", self.args.amount);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_unstake {
	($config:ty) => {
		use crate::cli::standard::unstake::{UnstakeArgs, UnstakeHelper};
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Unstake tokens from the MCR domain
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct Unstake {
			/// Path to the configuration file
			#[orfile(config)]
			/// The config to use (this requires a specific account).
			#[clap(flatten)]
			pub config: $config,
			/// The arguments for unstaking
			#[clap(flatten)]
			args: UnstakeArgs,
		}

		impl Unstake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = UnstakeHelper::new(self.config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::Unstake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

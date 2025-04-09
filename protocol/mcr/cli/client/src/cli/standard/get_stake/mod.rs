use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GetStakeArgs {
	/// The custodian address
	#[clap(long)]
	custodian: String,

	/// The attester address
	#[clap(long)]
	attester: String,
}

pub struct GetStakeHelper<V: McrViewOperations> {
	view_config: V,
	args: GetStakeArgs,
}

impl<V: McrViewOperations + Clone> GetStakeHelper<V> {
	pub fn new(view_config: V, args: GetStakeArgs) -> Self {
		Self { view_config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.view_config.clone().try_into_config()?;
		let client = config.build().await?;
		let stake = client
			.get_stake(self.args.custodian.clone(), self.args.attester.clone())
			.await?;
		println!("Current stake for {}: {}", self.args.attester, stake);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_get_stake {
	($view_config:ty) => {
		use crate::cli::standard::get_stake::{GetStakeArgs, GetStakeHelper};
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get the current epoch stake for an attester
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetStake {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $view_config,
			/// The arguments for getting stake
			#[clap(flatten)]
			args: GetStakeArgs,
		}

		impl GetStake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = GetStakeHelper::new(self.view_config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::GetStake {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

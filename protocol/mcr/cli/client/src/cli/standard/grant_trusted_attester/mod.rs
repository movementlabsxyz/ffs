use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct GrantTrustedAttesterArgs {
	/// The attester address to grant the role to
	#[clap(long)]
	attester: String,
}

pub struct GrantTrustedAttesterHelper<C: McrConfigOperations> {
	config: C,
	args: GrantTrustedAttesterArgs,
}

impl<C: McrConfigOperations + Clone> GrantTrustedAttesterHelper<C> {
	pub fn new(config: C, args: GrantTrustedAttesterArgs) -> Self {
		Self { config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;
		client.grant_trusted_attester(self.args.attester.clone()).await?;
		println!("Successfully granted TRUSTED_ATTESTER role to {}", self.args.attester);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_grant_trusted_attester {
	($config:ty) => {
		use crate::cli::standard::grant_trusted_attester::{
			GrantTrustedAttesterArgs, GrantTrustedAttesterHelper,
		};
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Grant TRUSTED_ATTESTER role to an attester
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GrantTrustedAttester {
			/// Path to the configuration file
			#[orfile(config)]
			/// The config to use (this requires admin privileges).
			#[clap(flatten)]
			pub config: $config,
			/// The arguments for granting the role
			#[clap(flatten)]
			args: GrantTrustedAttesterArgs,
		}

		impl GrantTrustedAttester {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper =
					GrantTrustedAttesterHelper::new(self.config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::GrantTrustedAttester {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

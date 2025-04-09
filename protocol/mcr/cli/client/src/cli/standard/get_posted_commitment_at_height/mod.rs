#[macro_export]
macro_rules! mcr_get_posted_commitment_at_height {
	($config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Get the posted commitment at a given height
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct GetPostedCommitmentAtHeight {
			#[clap(flatten)]
			config: $config,

			/// The height to get the commitment for
			#[clap(long)]
			height: u64,
		}

		impl GetPostedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.config.clone();
				let client = config.build().await?;

				match client.get_posted_commitment_at_height(self.height).await? {
					Some(commitment) => {
						println!("Posted commitment at height {}: {:?}", self.height, commitment)
					}
					None => println!("No posted commitment found at height {}", self.height),
				}

				Ok(())
			}
		}

		impl or_file::GetPostedCommitmentAtHeight {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}

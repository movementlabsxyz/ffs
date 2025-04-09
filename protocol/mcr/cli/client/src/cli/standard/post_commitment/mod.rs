#[macro_export]
macro_rules! mcr_post_commitment {
	($config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
		use mcr_types::commitment::{Commitment, Id, Vote};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Post a commitment
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct PostCommitment {
			#[clap(flatten)]
			config: $config,

			/// The commitment height at which to commit
			#[clap(long)]
			height: Option<u64>,
			/// The commitment id to commit
			#[clap(long)]
			id: Id,
			/// The commitment value to commit
			#[clap(long)]
			vote: Vote,
		}

		impl PostCommitment {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.config.clone();
				let client = config.build().await?;

				let height = match self.height {
					Some(height) => height,
					None => client.get_max_tolerable_commitment_height().await?,
				};

				let commitment = Commitment::new(height, self.id, self.vote);

				client.force_commitment(commitment).await?;

				Ok(())
			}
		}

		impl or_file::PostCommitment {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}

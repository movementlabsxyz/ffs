use clap::Parser;
use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
use mcr_types::commitment::{Commitment, Id, Vote};
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct PostCommitmentBatchArgs {
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

pub struct PostCommitmentBatchHelper<C: McrConfigOperations> {
	config: C,
	args: PostCommitmentBatchArgs,
}

impl<C: McrConfigOperations + Clone> PostCommitmentBatchHelper<C> {
	pub fn new(config: C, args: PostCommitmentBatchArgs) -> Self {
		Self { config, args }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let client = self.config.clone().build().await?;

		let height = match self.args.height {
			Some(height) => height,
			None => client.get_max_tolerable_commitment_height().await?,
		};

		let commitment = Commitment::new(height, self.args.id, self.args.vote);
		client.post_commitment_batch(vec![commitment]).await?;
		println!("Successfully posted commitment at height {}", height);
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_post_commitment_batch {
	($config:ty) => {
		use crate::cli::standard::post_commitment_batch::{
			PostCommitmentBatchArgs, PostCommitmentBatchHelper,
		};
		use clap::Parser;
		use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations};
		use mcr_types::commitment::{Commitment, Id, Vote};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Post a batch of commitments
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct PostCommitmentBatch {
			/// Path to the configuration file
			#[orfile(config)]
			/// The config to use (this requires a specific account).
			#[clap(flatten)]
			pub config: $config,
			/// The arguments for posting the batch of commitments
			#[clap(flatten)]
			args: PostCommitmentBatchArgs,
		}

		impl PostCommitmentBatch {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = PostCommitmentBatchHelper::new(self.config.clone(), self.args.clone());
				helper.execute().await
			}
		}

		impl or_file::PostCommitmentBatch {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

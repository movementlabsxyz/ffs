pub mod get_commitment_at_height;
pub mod get_max_tolerable_block_height;
pub mod get_posted_commitment_at_height;
pub mod post_admin_commitment;
pub mod post_commitment_batch;
pub mod stake;
pub mod stream_commitments;
pub mod unstake;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Eth {
	/// Force a block commitment (admin only)
	PostAdminCommitment(post_admin_commitment::PostAdminCommitment),
	/// Post a batch of block commitments
	PostCommitmentBatch(post_commitment_batch::PostCommitmentBatch),
	/// Stream block commitments
	StreamCommitments(stream_commitments::StreamCommitments),
	/// Get commitment at a specific height
	GetCommitmentAtHeight(get_commitment_at_height::GetCommitmentAtHeight),
	/// Get posted commitment at a specific height
	GetPostedCommitmentAtHeight(get_posted_commitment_at_height::GetPostedCommitmentAtHeight),
	/// Get max tolerable block height
	GetMaxTolerableBlockHeight(get_max_tolerable_block_height::GetMaxTolerableBlockHeight),
	/// Stake tokens for the MCR domain
	Stake(stake::Stake),
	/// Unstake tokens from the MCR domain
	Unstake(unstake::Unstake),
}

impl Eth {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Eth::PostAdminCommitment(cmd) => cmd.execute().await?,
			Eth::PostCommitmentBatch(cmd) => cmd.execute().await?,
			Eth::StreamCommitments(cmd) => cmd.execute().await?,
			Eth::GetCommitmentAtHeight(cmd) => cmd.execute().await?,
			Eth::GetPostedCommitmentAtHeight(cmd) => cmd.execute().await?,
			Eth::GetMaxTolerableBlockHeight(cmd) => cmd.execute().await?,
			Eth::Stake(cmd) => cmd.execute().await?,
			Eth::Unstake(cmd) => cmd.execute().await?,
		}
		Ok(())
	}
}

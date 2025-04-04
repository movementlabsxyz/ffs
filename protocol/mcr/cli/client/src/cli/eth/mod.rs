pub mod get_block_commitment;
pub mod get_accepted_commitment_at_height;
pub mod get_max_tolerable_block_height;
pub mod get_posted_commitment_at_height;
pub mod post_admin_commitment;
pub mod post_commitment;
pub mod post_commitment_batch;
pub mod stake;
pub mod get_stake;
pub mod stream_commitments;
pub mod unstake;
pub mod grant_trusted_attester;
use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
pub enum Eth {
	/// Force a block commitment (admin only)
	PostAdminCommitment(post_admin_commitment::PostAdminCommitment),
	/// Post a single commitment
	PostCommitment(post_commitment::PostCommitment),
	/// Post a batch of commitments
	PostCommitmentBatch(post_commitment_batch::PostCommitmentBatch),
	/// Stream commitments
	StreamCommitments(stream_commitments::StreamCommitments),
	/// Get a block commitment for a given height and attester
	GetCommitment(get_block_commitment::GetCommitment),
	/// Get accepted commitment at a specific height
	GetAcceptedCommitmentAtHeight(get_accepted_commitment_at_height::GetAcceptedCommitmentAtHeight),
	/// Get posted commitment at a specific height
	GetPostedCommitmentAtHeight(get_posted_commitment_at_height::GetPostedCommitmentAtHeight),
	/// Get max tolerable superBlock height
	GetMaxTolerableBlockHeight(get_max_tolerable_block_height::GetMaxTolerableBlockHeight),
	/// Stake tokens for the MCR domain
	Stake(stake::Stake),
	/// Get the current epoch stake for an attester
	GetStake(get_stake::GetStake),
	/// Unstake tokens from the MCR domain
	Unstake(unstake::Unstake),
	/// Grant TRUSTED_ATTESTER role to an attester
	GrantTrustedAttester(grant_trusted_attester::GrantTrustedAttester),
}

impl Eth {
	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		match self {
			Eth::GetCommitment(cmd) => cmd.execute().await?,
			Eth::PostAdminCommitment(cmd) => cmd.execute().await?,
			Eth::PostCommitment(cmd) => cmd.execute().await?,
			Eth::PostCommitmentBatch(cmd) => cmd.execute().await?,
			Eth::StreamCommitments(cmd) => cmd.execute().await?,
			Eth::GetAcceptedCommitmentAtHeight(cmd) => cmd.execute().await?,
			Eth::GetPostedCommitmentAtHeight(cmd) => cmd.execute().await?,
			Eth::GetMaxTolerableBlockHeight(cmd) => cmd.execute().await?,
			Eth::Stake(cmd) => cmd.execute().await?,
			Eth::GetStake(cmd) => cmd.execute().await?,
			Eth::Unstake(cmd) => cmd.execute().await?,
			Eth::GrantTrustedAttester(cmd) => cmd.execute().await?,
		}
		Ok(())
	}
}

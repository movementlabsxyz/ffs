pub mod get_accepted_commitment_at_height;
pub mod get_commitment;
pub mod get_max_tolerable_commitment_height;
pub mod get_posted_commitment_at_height;
pub mod get_stake;
pub mod grant_trusted_attester;
pub mod post_admin_commitment;
pub mod post_commitment;
pub mod post_commitment_batch;
pub mod stake;
pub mod stream_commitments;
pub mod unstake;

#[macro_export]
macro_rules! mcr_standard_commands_relative {
	($enum_name:ident) => {
		pub mod get_accepted_commitment_at_height;
		pub mod get_commitment;
		pub mod get_max_tolerable_commitment_height;
		pub mod get_posted_commitment_at_height;
		pub mod get_stake;
		pub mod grant_trusted_attester;
		pub mod post_admin_commitment;
		pub mod post_commitment;
		pub mod post_commitment_batch;
		pub mod stake;
		pub mod stream_commitments;
		pub mod unstake;
		use clap::Subcommand;

		#[derive(Subcommand)]
		#[clap(rename_all = "kebab-case")]
		#[clap(after_help = concat!("KEEP THIS UNTIL PRODUCTION-READY : Defined in: ", file!()))]
		pub enum $enum_name {
			/// Force a commitment (admin only)
			#[clap(subcommand)]
			PostAdminCommitment(post_admin_commitment::or_file::PostAdminCommitment),
			/// Post a single commitment
			#[clap(subcommand)]
			PostCommitment(post_commitment::or_file::PostCommitment),
			/// Post a batch of commitments
			#[clap(subcommand)]
			PostCommitmentBatch(post_commitment_batch::or_file::PostCommitmentBatch),
			/// Stream commitments
			#[clap(subcommand)]
			StreamCommitments(stream_commitments::or_file::StreamCommitments),
			/// Get a commitment for a given height and attester
			#[clap(subcommand)]
			GetCommitment(get_commitment::or_file::GetCommitment),
			/// Get accepted commitment at a specific height
			#[clap(subcommand)]
			GetAcceptedCommitmentAtHeight(
				get_accepted_commitment_at_height::or_file::GetAcceptedCommitmentAtHeight,
			),
			/// Get posted commitment at a specific height
			#[clap(subcommand)]
			GetPostedCommitmentAtHeight(
				get_posted_commitment_at_height::or_file::GetPostedCommitmentAtHeight,
			),
			/// Get max tolerable commitment height
			#[clap(subcommand)]
			GetMaxTolerableCommitmentHeight(
				get_max_tolerable_commitment_height::or_file::GetMaxTolerableCommitmentHeight,
			),
			/// Stake tokens for the MCR domain
			#[clap(subcommand)]
			Stake(stake::or_file::Stake),
			/// Get the current epoch stake for an attester
			#[clap(subcommand)]
			GetStake(get_stake::or_file::GetStake),
			/// Unstake tokens from the MCR domain
			#[clap(subcommand)]
			Unstake(unstake::or_file::Unstake),
			/// Grant TRUSTED_ATTESTER role to an attester
			#[clap(subcommand)]
			GrantTrustedAttester(grant_trusted_attester::or_file::GrantTrustedAttester),
		}

		impl $enum_name {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				match self {
					$enum_name::GetCommitment(cmd) => cmd.execute().await?,
					$enum_name::PostAdminCommitment(cmd) => cmd.execute().await?,
					$enum_name::PostCommitment(cmd) => cmd.execute().await?,
					$enum_name::PostCommitmentBatch(cmd) => cmd.execute().await?,
					$enum_name::StreamCommitments(cmd) => cmd.execute().await?,
					$enum_name::GetAcceptedCommitmentAtHeight(cmd) => cmd.execute().await?,
					$enum_name::GetPostedCommitmentAtHeight(cmd) => cmd.execute().await?,
					$enum_name::GetMaxTolerableCommitmentHeight(cmd) => cmd.execute().await?,
					$enum_name::Stake(cmd) => cmd.execute().await?,
					$enum_name::GetStake(cmd) => cmd.execute().await?,
					$enum_name::Unstake(cmd) => cmd.execute().await?,
					$enum_name::GrantTrustedAttester(cmd) => cmd.execute().await?,
				}
				Ok(())
			}
		}
	};
}

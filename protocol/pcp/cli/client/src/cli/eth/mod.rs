pub mod post_admin_commitment;

use clap::Subcommand;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Eth {
    PostAdminCommitment(post_admin_commitment::PostAdminCommitment),
}

impl Eth {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Eth::PostAdminCommitment(cmd) => cmd.execute().await?,
        }
        Ok(())
    }
} 
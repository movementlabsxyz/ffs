use anyhow::Context;
use clap::Parser;
use pcp_protocol_client_core_eth::config::Config;
use pcp_protocol_client_core_util::PcpClientOperations;
use pcp_types::block_commitment::SuperBlockCommitment;
use serde::{Deserialize, Serialize};

#[derive(Parser, Serialize, Deserialize, Debug, Clone)]
pub struct PostAdminCommitment {
    #[clap(flatten)]
    pub config: Option<Config>,
}

impl PostAdminCommitment {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        let config = self.config.clone().context("no config provided")?;
        let client = config.build().await?;
        client.force_block_commitment(SuperBlockCommitment::test()).await?;

        Ok(())
    }
} 
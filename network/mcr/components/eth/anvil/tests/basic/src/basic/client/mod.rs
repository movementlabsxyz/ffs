use mcr_protocol_client_core_util::McrClientOperations;
use mcr_protocol_client_eth_core::config::StandardClient;
use mcr_types::block_commitment::Commitment;

#[derive(Debug, Clone)]
pub enum Act {
	PostCommitment(Commitment),
}

pub struct Client {
	pub mcr_protocol_client: StandardClient,
}

impl Client {
	pub fn new(mcr_protocol_client: StandardClient) -> Self {
		Self { mcr_protocol_client }
	}

	pub async fn act(&self, act: Act) -> Result<(), anyhow::Error> {
		println!("acting on {:?}", act);

		match act {
			Act::PostCommitment(block_commitment) => {
				self.mcr_protocol_client.post_block_commitment(block_commitment).await?;
				Ok(())
			}
		}
	}
}

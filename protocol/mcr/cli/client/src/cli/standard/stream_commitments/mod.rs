#[macro_export]
macro_rules! mcr_stream_commitments {
	($view_config:ty) => {
		use clap::Parser;
		use mcr_protocol_client_core_util::{
			McrClientOperations, McrConfigOperations, McrViewOperations,
		};
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};
		use tokio_stream::StreamExt;

		/// Stream commitments
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct StreamCommitments {
			#[clap(flatten)]
			view_config: $view_config,
		}

		impl StreamCommitments {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let config = self.view_config.clone().try_into_config()?;
				let client = config.build().await?;

				let mut stream = client.stream_commitments().await?;

				while let Some(result) = stream.next().await {
					match result {
						Ok(commitment) => println!("Received commitment: {:?}", commitment),
						Err(e) => eprintln!("Error receiving commitment: {:?}", e),
					}
				}

				Ok(())
			}
		}

		impl or_file::StreamCommitments {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let resolved = self.clone().resolve().await?;
				resolved.execute().await
			}
		}
	};
}

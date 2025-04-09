use mcr_protocol_client_core_util::{McrClientOperations, McrConfigOperations, McrViewOperations};
use tokio_stream::StreamExt;

pub struct StreamCommitmentsHelper<V: McrViewOperations> {
	view_config: V,
}

impl<V: McrViewOperations + Clone> StreamCommitmentsHelper<V> {
	pub fn new(view_config: V) -> Self {
		Self { view_config }
	}

	pub async fn execute(&self) -> Result<(), anyhow::Error> {
		let config = self.view_config.clone().try_into_config()?;
		let client = config.build().await?;

		let mut stream = client.stream_commitments().await?;
		while let Some(commitment) = stream.next().await {
			println!("Received commitment: {:?}", commitment);
		}
		Ok(())
	}
}

#[macro_export]
macro_rules! mcr_stream_commitments {
	($view_config:ty) => {
		use crate::cli::standard::stream_commitments::StreamCommitmentsHelper;
		use clap::Parser;
		use orfile::Orfile;
		use serde::{Deserialize, Serialize};

		/// Stream commitments
		#[derive(Parser, Serialize, Deserialize, Debug, Clone, Orfile)]
		#[clap(help_expected = true)]
		pub struct StreamCommitments {
			/// Path to the configuration file
			#[orfile(config)]
			/// The view config to use (this is a view method after all).
			#[clap(flatten)]
			pub view_config: $view_config,
		}

		impl StreamCommitments {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let helper = StreamCommitmentsHelper::new(self.view_config.clone());
				helper.execute().await
			}
		}

		impl or_file::StreamCommitments {
			pub async fn execute(&self) -> Result<(), anyhow::Error> {
				let inner = self.clone().resolve().await?;
				inner.execute().await
			}
		}
	};
}

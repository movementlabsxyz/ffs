use clap::Parser;
use std::path::PathBuf;
use anyhow::{Context, Result};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Generate {
	/// Override the default docs location
	#[clap(long)]
	pub file: Option<String>,
}

impl Generate {
	pub async fn execute<C>(&self, cli_name: &str) -> Result<(), anyhow::Error>
	where
		C: Parser,
	{
		let mut markdown = clap_markdown::help_markdown::<C>();
		
		markdown = markdown.replace("Command-line", &format!("{} CLI", cli_name));

		// Default path: sdk/cli/{cli_name}/docs/README.md
		let file = self.file.clone().unwrap_or_else(|| {
			format!("sdk/cli/{}/docs/README.md", cli_name)
		});

		std::fs::write(&file, markdown)?;
		Ok(())
	}
}

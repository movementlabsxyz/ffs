use anyhow::Result;
use clap::{CommandFactory, Parser};

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Generate {
	/// Override the default docs location
	#[clap(long)]
	pub file: Option<String>,
}

impl Generate {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: CommandFactory,
	{
		// get the cli name from the command factory
		let cli_name = C::command().get_name().to_string();

		let mut markdown = clap_markdown::help_markdown::<C>();

		markdown = markdown.replace("Command-line", &format!("{} CLI", cli_name));

		// Default path: sdk/cli/{cli_name}/docs/README.md
		let file = self
			.file
			.clone()
			.unwrap_or_else(|| format!("sdk/cli/{}/docs/README.md", cli_name));

		std::fs::write(&file, markdown)?;
		Ok(())
	}
}

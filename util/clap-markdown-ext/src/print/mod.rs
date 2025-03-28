use clap::{CommandFactory, Parser};

/// Print the CLI documentation to the console.
#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Print {}

impl Default for Print {
	fn default() -> Self {
		Self {}
	}
}

impl Print {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: CommandFactory,
	{
		let markdown = clap_markdown::help_markdown::<C>();
		println!("{}", markdown);
		Ok(())
	}
}

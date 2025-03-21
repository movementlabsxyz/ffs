use clap::Parser;

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Print {}

impl Print {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: Parser,
	{
		let markdown = clap_markdown::help_markdown::<C>();
		println!("{}", markdown);
		Ok(())
	}
}

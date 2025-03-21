use clap::Parser;

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct File {
	/// the file to write out to
	#[clap(long)]
	pub file: String,
}

impl File {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: Parser,
	{
		let markdown = clap_markdown::help_markdown::<C>();
		std::fs::write(&self.file, markdown)?;
		Ok(())
	}
}

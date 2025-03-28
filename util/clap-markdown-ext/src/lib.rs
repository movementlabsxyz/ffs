pub mod generate;
pub mod file;
pub mod print;
pub mod workspace;

use clap::{Parser, Subcommand};

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Markdown {
	/// Generate and update the documentation
	Generate(generate::Generate),
	/// Print the documentation to a file (providing the file path)
	File(file::File),
	/// Print the documentation in the shell
	Print(print::Print),
	/// ???
	Workspace(workspace::Workspace),
}

impl Markdown {
	pub async fn execute<C>(&self, cli_name: &str) -> Result<(), anyhow::Error>
	where
		C: Parser,
	{
		match self {
			Markdown::Generate(generate) => generate.execute::<C>(cli_name).await?,
			Markdown::File(file) => file.execute::<C>().await?,
			Markdown::Print(print) => print.execute::<C>().await?,
			Markdown::Workspace(workspace) => workspace.execute::<C>().await?,
		}
		Ok(())
	}
}

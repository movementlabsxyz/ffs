pub mod file;
pub mod generate;
pub mod print;
pub mod workspace;

use clap::{CommandFactory, Subcommand};

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Markdown {
	/// Generate and update the documentation
	Generate(generate::Generate),
	/// Print the documentation to a file (providing the file path)
	File(file::File),
	/// Print the documentation in the shell
	Print(print::Print),
	/// Generate the documentation for the workspace
	Workspace(workspace::Workspace),
}

impl Markdown {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: CommandFactory,
	{
		match self {
			Markdown::Generate(generate) => generate.execute::<C>().await?,
			Markdown::File(file) => file.execute::<C>().await?,
			Markdown::Print(print) => print.execute::<C>().await?,
			Markdown::Workspace(workspace) => workspace.execute::<C>().await?,
		}
		Ok(())
	}
}

impl Default for Markdown {
	fn default() -> Self {
		Self::Print(print::Print::default())
	}
}

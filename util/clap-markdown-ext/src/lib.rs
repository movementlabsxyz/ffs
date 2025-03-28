pub mod file;
pub mod print;
pub mod workspace;

use clap::{CommandFactory, Subcommand};

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Markdown {
	File(file::File),
	Print(print::Print),
	Workspace(workspace::Workspace),
}

impl Markdown {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: CommandFactory,
	{
		match self {
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

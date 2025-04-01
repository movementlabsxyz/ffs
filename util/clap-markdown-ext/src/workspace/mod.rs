use cargo_metadata::MetadataCommand;
use clap::{CommandFactory, Parser};
use std::path::Path;

/// Write the CLI documentation to a file in the workspace.
#[derive(Parser)]
#[clap(help_expected = true)]
pub struct Workspace {
	/// The file to write out to, relative to the crate root
	#[clap(long)]
	pub relative_path: String,
}

impl Workspace {
	pub async fn execute<C>(&self) -> Result<(), anyhow::Error>
	where
		C: CommandFactory,
	{
		// Get the currently executing binary's name
		let binary_path = std::env::args().next().unwrap_or_default();
		let binary_name = Path::new(&binary_path)
			.file_stem()
			.ok_or_else(|| anyhow::anyhow!("Could not determine binary name"))?
			.to_string_lossy()
			.to_string();

		// Load metadata
		let metadata = MetadataCommand::new().exec()?;

		// Find the package that defines this binary
		let package = metadata
			.packages
			.iter()
			.find(|pkg| pkg.targets.iter().any(|target| target.name == binary_name))
			.ok_or_else(|| {
				anyhow::anyhow!("could not find package for binary `{}`", binary_name)
			})?;

		let crate_root = package.manifest_path.parent().unwrap();

		let output_path = crate_root.join(&self.relative_path);

		// Generate markdown
		let markdown = clap_markdown::help_markdown::<C>();

		// Ensure directory exists
		if let Some(parent) = output_path.parent() {
			std::fs::create_dir_all(parent)?;
		}

		std::fs::write(&output_path, markdown)?;

		println!("Wrote help markdown to {}", output_path);

		Ok(())
	}
}

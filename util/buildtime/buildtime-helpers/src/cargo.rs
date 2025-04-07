use cargo_metadata;
use std::path::PathBuf;

/// Gets the current cargo workspace root using `cargo metadata`
pub fn cargo_workspace() -> Result<PathBuf, anyhow::Error> {
	let metadata = cargo_metadata::MetadataCommand::new().no_deps().exec()?;

	Ok(metadata.workspace_root.into())
}

#[cfg(test)]
pub mod test {

	use super::*;
	use std::fs;

	#[test]
	fn test_cargo_workspace() -> Result<(), anyhow::Error> {
		// Get the cargo workspace
		let workspace = cargo_workspace()?;

		// Check that a Cargo.toml file exists in the workspace
		assert_eq!(workspace.join("Cargo.toml").exists(), true);

		// Parse the toml and check that workspace.package.authors is ["Movement Labs"]
		let toml = fs::read_to_string(workspace.join("Cargo.toml"))?;
		let toml: toml::Value = toml::from_str(&toml)?;
		let authors = toml["workspace"]["package"]["authors"].as_array();
		assert_eq!(authors, Some(&vec![toml::Value::String("Movement Labs".to_string())]));

		Ok(())
	}
}

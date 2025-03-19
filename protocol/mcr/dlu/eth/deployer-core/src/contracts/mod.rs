use std::ffi::OsStr;
use std::fs::File;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use zip::read::ZipArchive;

pub const CONTRACTS_ZIP: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/contracts.zip"));

#[derive(Debug)]
pub enum WorkspacePath {
	PathBuf(PathBuf),
	TempDir(TempDir),
}

impl WorkspacePath {
	pub fn get_path(&self) -> &Path {
		match self {
			WorkspacePath::PathBuf(path) => path.as_path(),
			WorkspacePath::TempDir(temp_dir) => temp_dir.path(),
		}
	}
}

#[derive(Debug)]
pub struct ContractWorkspace {
	pub contracts_zip: &'static [u8],
	pub workspace_path: WorkspacePath,
}

/// Used to manage a contract workspace
impl ContractWorkspace {
	/// Creates a new contract workspace.
	pub fn new(contracts_zip: &'static [u8], workspace_path: WorkspacePath) -> Self {
		ContractWorkspace { contracts_zip, workspace_path }
	}

	/// Creates a new temporary contract workspace.
	pub fn try_temp(contracts_zip: &'static [u8]) -> Result<Self, std::io::Error> {
		let temp_dir = TempDir::new()?;
		Ok(ContractWorkspace { contracts_zip, workspace_path: WorkspacePath::TempDir(temp_dir) })
	}

	/// Creates a new contract workspace using the `CONTRACTS_ZIP` constant.
	pub fn try_temp_tip() -> Result<Self, std::io::Error> {
		Self::try_temp(CONTRACTS_ZIP)
	}

	/// Gets the workspace path
	pub fn get_workspace_path(&self) -> &Path {
		self.workspace_path.get_path()
	}

	/// Unzips the contracts zip file to the provided path.
	pub fn prepare_directory(&self) -> Result<(), std::io::Error> {
		// Determine the output directory
		let output_dir = match &self.workspace_path {
			WorkspacePath::PathBuf(path) => path.clone(),
			WorkspacePath::TempDir(temp_dir) => temp_dir.path().to_path_buf(),
		};

		// Read the embedded ZIP archive
		let cursor = Cursor::new(self.contracts_zip);
		let mut archive = ZipArchive::new(cursor)?;

		// Extract each file in the ZIP archive
		for i in 0..archive.len() {
			let mut file = archive.by_index(i)?;
			let outpath = output_dir.join(file.name());

			if file.is_dir() {
				std::fs::create_dir_all(&outpath)?;
			} else {
				if let Some(parent) = outpath.parent() {
					std::fs::create_dir_all(parent)?;
				}
				let mut outfile = File::create(&outpath)?;
				std::io::copy(&mut file, &mut outfile)?;
			}
		}

		Ok(())
	}

	pub async fn run_command<C, I, S>(&self, command: C, args: I) -> Result<(), anyhow::Error>
	where
		C: AsRef<OsStr>,
		I: IntoIterator<Item = S>,
		S: AsRef<OsStr>,
	{
		// Implementation of the run_command function
		let path = self.get_workspace_path();
		commander::run_command(command, args, Some(path)).await?;

		Ok(())
	}
}

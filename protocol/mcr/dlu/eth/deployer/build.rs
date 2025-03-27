use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};

fn main() -> std::io::Result<()> {
	// Define the source directory (relative to the crate)
	let source_dir = Path::new("../contracts");
	if !source_dir.exists() {
		panic!("Source directory {:?} does not exist!", source_dir);
	}

	// Get the output directory where build artifacts are stored
	let out_dir = env::var("OUT_DIR").unwrap();
	let zip_path = Path::new(&out_dir).join("contracts.zip");

	// Create the zip file
	let zip_file = File::create(&zip_path)?;
	let mut zip = ZipWriter::new(BufWriter::new(zip_file));
	let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);

	// Walk through the source directory recursively
	for entry in WalkDir::new(source_dir).into_iter().filter_map(Result::ok) {
		let path = entry.path();
		let name = path.strip_prefix(source_dir).unwrap().to_str().unwrap();

		if path.is_file() {
			let mut file = File::open(path)?;
			zip.start_file(name, options)?;
			std::io::copy(&mut file, &mut zip)?;
		} else if path.is_dir() {
			zip.add_directory(name, options)?;
		}
	}

	zip.finish()?;
	println!("cargo:rerun-if-changed=../contracts");

	Ok(())
}

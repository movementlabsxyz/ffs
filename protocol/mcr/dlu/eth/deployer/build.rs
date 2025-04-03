use include_dir::{Buildtime, BuildtimeError};
use std::path::Path;

fn main() -> Result<(), BuildtimeError> {
	let builder = Buildtime::new(Path::new("../contracts").into(), "contracts".to_string());

	builder.build()?;

	Ok(())
}

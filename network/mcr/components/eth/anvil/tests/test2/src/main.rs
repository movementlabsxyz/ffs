use test_test2_mcr_network_anvil_component_core::{config::Config, Test};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
	let config = Config::to_be_filled()?;
	let test = Test::new(config);
	test.run().await?;
	Ok(())
}

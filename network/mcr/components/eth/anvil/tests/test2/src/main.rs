use test_test2_mcr_network_anvil_component_core::{config::Config, Basic};

#[tokio::main]
pub async fn main() -> Result<(), anyhow::Error> {
	let config = Config::to_be_filled()?;
	let basic = Basic::new(config);
	basic.run().await?;
	Ok(())
}

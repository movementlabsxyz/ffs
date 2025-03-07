use ffs_environment::{backend::config_file::ConfigFile, ffs_environment};
use post_confirmations_config::Config;
use post_confirmations_settlement_setup::Setup;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
	use tracing_subscriber::EnvFilter;

	tracing_subscriber::fmt()
		.with_env_filter(
			EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
		)
		.init();

	// get the config file
	let dot_movement = dot_movement::DotMovement::try_from_env()?;
	let config_file = dot_movement.try_get_or_create_config_file().await?;

	// get a matching ffs_environment object
	let ffs_environment: ffs_environment<Config, ConfigFile> = ffs_environment::new(
		ConfigFile::new(config_file),
		vec!["post_confirmations_settlement".to_string()],
	);

	// Apply all of the setup steps
	let anvil_join_handle = ffs_environment
		.try_transaction_with_result(|config| async move {
			tracing::info!("Config: {:?}", config);
			let config = config.unwrap_or_default();
			tracing::info!("Config: {:?}", config);

			let (config, anvil_join_handle) = Setup::default().setup(&dot_movement, config).await?;
			Ok((Some(config), anvil_join_handle))
		})
		.await?;

	// wait for anvil to finish
	let _ = anvil_join_handle.await?;

	Ok(())
}

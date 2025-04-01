use mcr_network_anvil_component_core::dev::lifecycle::up;

#[derive(Debug, Clone)]
pub struct Config {
	pub up: up::Config,
}

impl Config {
	pub fn to_be_filled() -> Result<Self, anyhow::Error> {
		Ok(Self { up: up::Config::to_be_filled()? })
	}
}

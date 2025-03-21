use kestrel::{fulfill::custom::CustomProcessor, fulfill::FulfillError};
use tokio::sync::mpsc::Receiver;

/// The data output from starting Anvil
#[derive(Debug, Clone)]
pub struct AnvilData {
	pub signers: Vec<String>,
	pub private_keys: Vec<String>,
	pub mnemonic: String,
	pub derivation_path: String,
	pub chain_id: u64,
	pub base_fee: u64,
	pub gas_limit: u64,
	pub genesis_timestamp: u64,
}

/// A struct used to parse the output of Anvil
pub struct ParseAnvilData;

impl CustomProcessor<AnvilData> for ParseAnvilData {
	async fn process_receiver(
		&self,
		receiver: &mut Receiver<String>,
	) -> Result<Option<AnvilData>, FulfillError> {
		let mut signers = Vec::new();
		let mut private_keys = Vec::new();
		let mut mnemonic = None;
		let mut derivation_path = None;
		let mut chain_id = None;
		let mut base_fee = None;
		let mut gas_limit = None;
		let mut genesis_timestamp = None;

		#[derive(Debug)]
		enum Section {
			None,
			Accounts,
			Keys,
			Wallet,
			ChainId,
			BaseFee,
			GasLimit,
			Timestamp,
		}

		let mut current_section = Section::None;

		while let Some(line) = receiver.recv().await {
			let stripped = strip_ansi_escapes::strip(&line);
			let trimmed = std::str::from_utf8(&stripped)
				.map_err(|e| FulfillError::Internal(format!("invalid UTF-8: {e}").into()))?
				.trim();

			match trimmed {
				"Available Accounts" => {
					current_section = Section::Accounts;
				}
				"Private Keys" => {
					current_section = Section::Keys;
				}
				"Wallet" => {
					current_section = Section::Wallet;
				}
				"Chain ID" => {
					current_section = Section::ChainId;
				}
				"Base Fee" => {
					current_section = Section::BaseFee;
				}
				"Gas Limit" => {
					current_section = Section::GasLimit;
				}
				"Genesis Timestamp" => {
					current_section = Section::Timestamp;
				}
				"" | "==================" => {
					// skip separators and blank lines
				}
				_ => match current_section {
					Section::Accounts => {
						if let Some(start) = trimmed.find(')') {
							let address = trimmed[start + 1..]
								.split_whitespace()
								.next()
								.ok_or_else(|| {
									FulfillError::Internal("Invalid address line".into())
								})?
								.to_string();
							signers.push(address);
						}
					}
					Section::Keys => {
						if let Some(start) = trimmed.find(')') {
							private_keys.push(trimmed[start + 1..].trim().to_string());
						}
					}
					Section::Wallet => {
						if let Some((key, value)) = trimmed.split_once(':') {
							match key.trim() {
								"Mnemonic" => {
									mnemonic = Some(value.trim().to_string());
								}
								"Derivation path" => {
									derivation_path = Some(value.trim().to_string());
								}
								_ => {}
							}
						}
					}
					Section::ChainId => {
						if let Ok(id) = trimmed.parse::<u64>() {
							chain_id = Some(id);
						}
					}
					Section::BaseFee => {
						if let Ok(fee) = trimmed.parse::<u64>() {
							base_fee = Some(fee);
						}
					}
					Section::GasLimit => {
						if let Ok(limit) = trimmed.parse::<u64>() {
							gas_limit = Some(limit);
						}
					}
					Section::Timestamp => {
						if let Ok(ts) = trimmed.parse::<u64>() {
							genesis_timestamp = Some(ts); // End of relevant data
						}
						break;
					}
					Section::None => {}
				},
			}
		}

		Ok(Some(AnvilData {
			signers,
			private_keys,
			mnemonic: mnemonic.unwrap_or_default(),
			derivation_path: derivation_path.unwrap_or_default(),
			chain_id: chain_id.unwrap_or_default(),
			base_fee: base_fee.unwrap_or_default(),
			gas_limit: gas_limit.unwrap_or_default(),
			genesis_timestamp: genesis_timestamp.unwrap_or_default(),
		}))
	}
}

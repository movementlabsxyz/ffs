use kestrel::{
	fulfill::{custom::Custom, Fulfill},
	process::{command::Command, Pipe, ProcessOperations},
	State,
};

use crate::util::parser::{AnvilData, ParseAnvilData};

pub struct Deploy {
	anvil_data: State<AnvilData>,
}

impl Deploy {
	pub fn new() -> Self {
		Deploy { anvil_data: State::new() }
	}

	pub fn anvil_data(&self) -> &State<AnvilData> {
		&self.anvil_data
	}

	pub async fn run(self) -> Result<(), anyhow::Error> {
		let anvil_data = self.anvil_data.clone();
		let anvil = kestrel::task(async move {
			// construct the keys fulfiller
			let keys_fulfiller = Custom::new(anvil_data.write(), ParseAnvilData);

			// construct the anvil command
			let mut anvil_command =
				Command::line("anvil", Vec::<String>::new(), None, false, vec![], vec![]);

			// pipe the anvil command output to the keys fulfiller
			anvil_command.pipe(Pipe::STDOUT, keys_fulfiller.sender()?)?;

			// start the keys fulfiller
			let keys_task = keys_fulfiller.spawn()?;

			// run the anvil command
			let anvil_task = anvil_command.spawn()?;

			// wait for both tasks to finish
			keys_task.await??;
			anvil_task.await??;

			Ok::<_, anyhow::Error>(())
		});

		let signers = self.anvil_data.read().wait_for().await;

		println!("Signers: {:?}", signers);

		anvil.await??;

		Ok(())
	}
}

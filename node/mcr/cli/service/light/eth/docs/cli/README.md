# Command-Line Help for `mcr-light-node-eth`

This document contains the help content for the `mcr-light-node-eth` command-line program.

**Command Overview:**

* [`mcr-light-node-eth`↴](#mcr-light-node-eth)
* [`mcr-light-node-eth markdown`↴](#mcr-light-node-eth-markdown)
* [`mcr-light-node-eth markdown generate`↴](#mcr-light-node-eth-markdown-generate)
* [`mcr-light-node-eth markdown file`↴](#mcr-light-node-eth-markdown-file)
* [`mcr-light-node-eth markdown print`↴](#mcr-light-node-eth-markdown-print)
* [`mcr-light-node-eth markdown workspace`↴](#mcr-light-node-eth-markdown-workspace)
* [`mcr-light-node-eth emln`↴](#mcr-light-node-eth-emln)
* [`mcr-light-node-eth emln where`↴](#mcr-light-node-eth-emln-where)
* [`mcr-light-node-eth emln using`↴](#mcr-light-node-eth-emln-using)
* [`mcr-light-node-eth exln`↴](#mcr-light-node-eth-exln)
* [`mcr-light-node-eth exln where`↴](#mcr-light-node-eth-exln-where)
* [`mcr-light-node-eth exln using`↴](#mcr-light-node-eth-exln-using)

## `mcr-light-node-eth`

The `mcr-light-node-eth` CLI

**Usage:** `mcr-light-node-eth [COMMAND]`

###### **Subcommands:**

* `markdown` — Generate markdown for the CLI
* `emln` — Runs the embedded Ethereum MCR light node
* `exln` — Runs the externalized Ethereum MCR light node



## `mcr-light-node-eth markdown`

Generate markdown for the CLI

**Usage:** `mcr-light-node-eth markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-light-node-eth markdown generate`

Generate and update the documentation

**Usage:** `mcr-light-node-eth markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-light-node-eth markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-light-node-eth markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-light-node-eth markdown print`

Print the documentation in the shell

**Usage:** `mcr-light-node-eth markdown print`



## `mcr-light-node-eth markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-light-node-eth markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-light-node-eth emln`

Runs the embedded Ethereum MCR light node

**Usage:** `mcr-light-node-eth emln <COMMAND>`

###### **Subcommands:**

* `where` — Run emln with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run emln with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-light-node-eth emln where`

Run emln with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-light-node-eth emln where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node
* `--address <ADDRESS>` — The address to listen on

  Default value: `0.0.0.0:44513`



## `mcr-light-node-eth emln using`

Run emln with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-light-node-eth emln using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--core-path <CORE_PATH>`



## `mcr-light-node-eth exln`

Runs the externalized Ethereum MCR light node

**Usage:** `mcr-light-node-eth exln <COMMAND>`

###### **Subcommands:**

* `where` — Run exln with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run exln with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-light-node-eth exln where`

Run exln with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-light-node-eth exln where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--address <ADDRESS>` — The address to listen on

  Default value: `0.0.0.0:44513`



## `mcr-light-node-eth exln using`

Run exln with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-light-node-eth exln using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--core-path <CORE_PATH>`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

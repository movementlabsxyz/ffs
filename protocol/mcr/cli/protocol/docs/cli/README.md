# Command-Line Help for `mcr-protocol`

This document contains the help content for the `mcr-protocol` command-line program.

**Command Overview:**

* [`mcr-protocol`↴](#mcr-protocol)
* [`mcr-protocol markdown`↴](#mcr-protocol-markdown)
* [`mcr-protocol markdown generate`↴](#mcr-protocol-markdown-generate)
* [`mcr-protocol markdown file`↴](#mcr-protocol-markdown-file)
* [`mcr-protocol markdown print`↴](#mcr-protocol-markdown-print)
* [`mcr-protocol markdown workspace`↴](#mcr-protocol-markdown-workspace)
* [`mcr-protocol client`↴](#mcr-protocol-client)
* [`mcr-protocol client markdown`↴](#mcr-protocol-client-markdown)
* [`mcr-protocol client markdown generate`↴](#mcr-protocol-client-markdown-generate)
* [`mcr-protocol client markdown file`↴](#mcr-protocol-client-markdown-file)
* [`mcr-protocol client markdown print`↴](#mcr-protocol-client-markdown-print)
* [`mcr-protocol client markdown workspace`↴](#mcr-protocol-client-markdown-workspace)
* [`mcr-protocol client eth`↴](#mcr-protocol-client-eth)
* [`mcr-protocol client eth exln`↴](#mcr-protocol-client-eth-exln)
* [`mcr-protocol client eth exln post-admin-commitment`↴](#mcr-protocol-client-eth-exln-post-admin-commitment)
* [`mcr-protocol client eth exln post-admin-commitment where`↴](#mcr-protocol-client-eth-exln-post-admin-commitment-where)
* [`mcr-protocol client eth exln post-admin-commitment using`↴](#mcr-protocol-client-eth-exln-post-admin-commitment-using)
* [`mcr-protocol client eth exln post-commitment`↴](#mcr-protocol-client-eth-exln-post-commitment)
* [`mcr-protocol client eth exln post-commitment where`↴](#mcr-protocol-client-eth-exln-post-commitment-where)
* [`mcr-protocol client eth exln post-commitment using`↴](#mcr-protocol-client-eth-exln-post-commitment-using)
* [`mcr-protocol client eth exln post-commitment-batch`↴](#mcr-protocol-client-eth-exln-post-commitment-batch)
* [`mcr-protocol client eth exln post-commitment-batch where`↴](#mcr-protocol-client-eth-exln-post-commitment-batch-where)
* [`mcr-protocol client eth exln post-commitment-batch using`↴](#mcr-protocol-client-eth-exln-post-commitment-batch-using)
* [`mcr-protocol client eth exln stream-commitments`↴](#mcr-protocol-client-eth-exln-stream-commitments)
* [`mcr-protocol client eth exln stream-commitments where`↴](#mcr-protocol-client-eth-exln-stream-commitments-where)
* [`mcr-protocol client eth exln stream-commitments using`↴](#mcr-protocol-client-eth-exln-stream-commitments-using)
* [`mcr-protocol client eth exln get-commitment`↴](#mcr-protocol-client-eth-exln-get-commitment)
* [`mcr-protocol client eth exln get-commitment where`↴](#mcr-protocol-client-eth-exln-get-commitment-where)
* [`mcr-protocol client eth exln get-commitment using`↴](#mcr-protocol-client-eth-exln-get-commitment-using)
* [`mcr-protocol client eth exln get-accepted-commitment-at-height`↴](#mcr-protocol-client-eth-exln-get-accepted-commitment-at-height)
* [`mcr-protocol client eth exln get-accepted-commitment-at-height where`↴](#mcr-protocol-client-eth-exln-get-accepted-commitment-at-height-where)
* [`mcr-protocol client eth exln get-accepted-commitment-at-height using`↴](#mcr-protocol-client-eth-exln-get-accepted-commitment-at-height-using)
* [`mcr-protocol client eth exln get-posted-commitment-at-height`↴](#mcr-protocol-client-eth-exln-get-posted-commitment-at-height)
* [`mcr-protocol client eth exln get-posted-commitment-at-height where`↴](#mcr-protocol-client-eth-exln-get-posted-commitment-at-height-where)
* [`mcr-protocol client eth exln get-posted-commitment-at-height using`↴](#mcr-protocol-client-eth-exln-get-posted-commitment-at-height-using)
* [`mcr-protocol client eth exln get-max-tolerable-commitment-height`↴](#mcr-protocol-client-eth-exln-get-max-tolerable-commitment-height)
* [`mcr-protocol client eth exln get-max-tolerable-commitment-height where`↴](#mcr-protocol-client-eth-exln-get-max-tolerable-commitment-height-where)
* [`mcr-protocol client eth exln get-max-tolerable-commitment-height using`↴](#mcr-protocol-client-eth-exln-get-max-tolerable-commitment-height-using)
* [`mcr-protocol client eth exln stake`↴](#mcr-protocol-client-eth-exln-stake)
* [`mcr-protocol client eth exln stake where`↴](#mcr-protocol-client-eth-exln-stake-where)
* [`mcr-protocol client eth exln stake using`↴](#mcr-protocol-client-eth-exln-stake-using)
* [`mcr-protocol client eth exln get-stake`↴](#mcr-protocol-client-eth-exln-get-stake)
* [`mcr-protocol client eth exln get-stake where`↴](#mcr-protocol-client-eth-exln-get-stake-where)
* [`mcr-protocol client eth exln get-stake using`↴](#mcr-protocol-client-eth-exln-get-stake-using)
* [`mcr-protocol client eth exln unstake`↴](#mcr-protocol-client-eth-exln-unstake)
* [`mcr-protocol client eth exln unstake where`↴](#mcr-protocol-client-eth-exln-unstake-where)
* [`mcr-protocol client eth exln unstake using`↴](#mcr-protocol-client-eth-exln-unstake-using)
* [`mcr-protocol client eth exln grant-trusted-attester`↴](#mcr-protocol-client-eth-exln-grant-trusted-attester)
* [`mcr-protocol client eth exln grant-trusted-attester where`↴](#mcr-protocol-client-eth-exln-grant-trusted-attester-where)
* [`mcr-protocol client eth exln grant-trusted-attester using`↴](#mcr-protocol-client-eth-exln-grant-trusted-attester-using)
* [`mcr-protocol client eth emln`↴](#mcr-protocol-client-eth-emln)
* [`mcr-protocol client eth emln post-admin-commitment`↴](#mcr-protocol-client-eth-emln-post-admin-commitment)
* [`mcr-protocol client eth emln post-admin-commitment where`↴](#mcr-protocol-client-eth-emln-post-admin-commitment-where)
* [`mcr-protocol client eth emln post-admin-commitment using`↴](#mcr-protocol-client-eth-emln-post-admin-commitment-using)
* [`mcr-protocol client eth emln post-commitment`↴](#mcr-protocol-client-eth-emln-post-commitment)
* [`mcr-protocol client eth emln post-commitment where`↴](#mcr-protocol-client-eth-emln-post-commitment-where)
* [`mcr-protocol client eth emln post-commitment using`↴](#mcr-protocol-client-eth-emln-post-commitment-using)
* [`mcr-protocol client eth emln post-commitment-batch`↴](#mcr-protocol-client-eth-emln-post-commitment-batch)
* [`mcr-protocol client eth emln post-commitment-batch where`↴](#mcr-protocol-client-eth-emln-post-commitment-batch-where)
* [`mcr-protocol client eth emln post-commitment-batch using`↴](#mcr-protocol-client-eth-emln-post-commitment-batch-using)
* [`mcr-protocol client eth emln stream-commitments`↴](#mcr-protocol-client-eth-emln-stream-commitments)
* [`mcr-protocol client eth emln stream-commitments where`↴](#mcr-protocol-client-eth-emln-stream-commitments-where)
* [`mcr-protocol client eth emln stream-commitments using`↴](#mcr-protocol-client-eth-emln-stream-commitments-using)
* [`mcr-protocol client eth emln get-commitment`↴](#mcr-protocol-client-eth-emln-get-commitment)
* [`mcr-protocol client eth emln get-commitment where`↴](#mcr-protocol-client-eth-emln-get-commitment-where)
* [`mcr-protocol client eth emln get-commitment using`↴](#mcr-protocol-client-eth-emln-get-commitment-using)
* [`mcr-protocol client eth emln get-accepted-commitment-at-height`↴](#mcr-protocol-client-eth-emln-get-accepted-commitment-at-height)
* [`mcr-protocol client eth emln get-accepted-commitment-at-height where`↴](#mcr-protocol-client-eth-emln-get-accepted-commitment-at-height-where)
* [`mcr-protocol client eth emln get-accepted-commitment-at-height using`↴](#mcr-protocol-client-eth-emln-get-accepted-commitment-at-height-using)
* [`mcr-protocol client eth emln get-posted-commitment-at-height`↴](#mcr-protocol-client-eth-emln-get-posted-commitment-at-height)
* [`mcr-protocol client eth emln get-posted-commitment-at-height where`↴](#mcr-protocol-client-eth-emln-get-posted-commitment-at-height-where)
* [`mcr-protocol client eth emln get-posted-commitment-at-height using`↴](#mcr-protocol-client-eth-emln-get-posted-commitment-at-height-using)
* [`mcr-protocol client eth emln get-max-tolerable-commitment-height`↴](#mcr-protocol-client-eth-emln-get-max-tolerable-commitment-height)
* [`mcr-protocol client eth emln get-max-tolerable-commitment-height where`↴](#mcr-protocol-client-eth-emln-get-max-tolerable-commitment-height-where)
* [`mcr-protocol client eth emln get-max-tolerable-commitment-height using`↴](#mcr-protocol-client-eth-emln-get-max-tolerable-commitment-height-using)
* [`mcr-protocol client eth emln stake`↴](#mcr-protocol-client-eth-emln-stake)
* [`mcr-protocol client eth emln stake where`↴](#mcr-protocol-client-eth-emln-stake-where)
* [`mcr-protocol client eth emln stake using`↴](#mcr-protocol-client-eth-emln-stake-using)
* [`mcr-protocol client eth emln get-stake`↴](#mcr-protocol-client-eth-emln-get-stake)
* [`mcr-protocol client eth emln get-stake where`↴](#mcr-protocol-client-eth-emln-get-stake-where)
* [`mcr-protocol client eth emln get-stake using`↴](#mcr-protocol-client-eth-emln-get-stake-using)
* [`mcr-protocol client eth emln unstake`↴](#mcr-protocol-client-eth-emln-unstake)
* [`mcr-protocol client eth emln unstake where`↴](#mcr-protocol-client-eth-emln-unstake-where)
* [`mcr-protocol client eth emln unstake using`↴](#mcr-protocol-client-eth-emln-unstake-using)
* [`mcr-protocol client eth emln grant-trusted-attester`↴](#mcr-protocol-client-eth-emln-grant-trusted-attester)
* [`mcr-protocol client eth emln grant-trusted-attester where`↴](#mcr-protocol-client-eth-emln-grant-trusted-attester-where)
* [`mcr-protocol client eth emln grant-trusted-attester using`↴](#mcr-protocol-client-eth-emln-grant-trusted-attester-using)
* [`mcr-protocol client light-node-proto`↴](#mcr-protocol-client-light-node-proto)
* [`mcr-protocol client light-node-proto post-admin-commitment`↴](#mcr-protocol-client-light-node-proto-post-admin-commitment)
* [`mcr-protocol client light-node-proto post-admin-commitment where`↴](#mcr-protocol-client-light-node-proto-post-admin-commitment-where)
* [`mcr-protocol client light-node-proto post-admin-commitment using`↴](#mcr-protocol-client-light-node-proto-post-admin-commitment-using)
* [`mcr-protocol client light-node-proto post-commitment`↴](#mcr-protocol-client-light-node-proto-post-commitment)
* [`mcr-protocol client light-node-proto post-commitment where`↴](#mcr-protocol-client-light-node-proto-post-commitment-where)
* [`mcr-protocol client light-node-proto post-commitment using`↴](#mcr-protocol-client-light-node-proto-post-commitment-using)
* [`mcr-protocol client light-node-proto post-commitment-batch`↴](#mcr-protocol-client-light-node-proto-post-commitment-batch)
* [`mcr-protocol client light-node-proto post-commitment-batch where`↴](#mcr-protocol-client-light-node-proto-post-commitment-batch-where)
* [`mcr-protocol client light-node-proto post-commitment-batch using`↴](#mcr-protocol-client-light-node-proto-post-commitment-batch-using)
* [`mcr-protocol client light-node-proto stream-commitments`↴](#mcr-protocol-client-light-node-proto-stream-commitments)
* [`mcr-protocol client light-node-proto stream-commitments where`↴](#mcr-protocol-client-light-node-proto-stream-commitments-where)
* [`mcr-protocol client light-node-proto stream-commitments using`↴](#mcr-protocol-client-light-node-proto-stream-commitments-using)
* [`mcr-protocol client light-node-proto get-commitment`↴](#mcr-protocol-client-light-node-proto-get-commitment)
* [`mcr-protocol client light-node-proto get-commitment where`↴](#mcr-protocol-client-light-node-proto-get-commitment-where)
* [`mcr-protocol client light-node-proto get-commitment using`↴](#mcr-protocol-client-light-node-proto-get-commitment-using)
* [`mcr-protocol client light-node-proto get-accepted-commitment-at-height`↴](#mcr-protocol-client-light-node-proto-get-accepted-commitment-at-height)
* [`mcr-protocol client light-node-proto get-accepted-commitment-at-height where`↴](#mcr-protocol-client-light-node-proto-get-accepted-commitment-at-height-where)
* [`mcr-protocol client light-node-proto get-accepted-commitment-at-height using`↴](#mcr-protocol-client-light-node-proto-get-accepted-commitment-at-height-using)
* [`mcr-protocol client light-node-proto get-posted-commitment-at-height`↴](#mcr-protocol-client-light-node-proto-get-posted-commitment-at-height)
* [`mcr-protocol client light-node-proto get-posted-commitment-at-height where`↴](#mcr-protocol-client-light-node-proto-get-posted-commitment-at-height-where)
* [`mcr-protocol client light-node-proto get-posted-commitment-at-height using`↴](#mcr-protocol-client-light-node-proto-get-posted-commitment-at-height-using)
* [`mcr-protocol client light-node-proto get-max-tolerable-commitment-height`↴](#mcr-protocol-client-light-node-proto-get-max-tolerable-commitment-height)
* [`mcr-protocol client light-node-proto get-max-tolerable-commitment-height where`↴](#mcr-protocol-client-light-node-proto-get-max-tolerable-commitment-height-where)
* [`mcr-protocol client light-node-proto get-max-tolerable-commitment-height using`↴](#mcr-protocol-client-light-node-proto-get-max-tolerable-commitment-height-using)
* [`mcr-protocol client light-node-proto stake`↴](#mcr-protocol-client-light-node-proto-stake)
* [`mcr-protocol client light-node-proto stake where`↴](#mcr-protocol-client-light-node-proto-stake-where)
* [`mcr-protocol client light-node-proto stake using`↴](#mcr-protocol-client-light-node-proto-stake-using)
* [`mcr-protocol client light-node-proto get-stake`↴](#mcr-protocol-client-light-node-proto-get-stake)
* [`mcr-protocol client light-node-proto get-stake where`↴](#mcr-protocol-client-light-node-proto-get-stake-where)
* [`mcr-protocol client light-node-proto get-stake using`↴](#mcr-protocol-client-light-node-proto-get-stake-using)
* [`mcr-protocol client light-node-proto unstake`↴](#mcr-protocol-client-light-node-proto-unstake)
* [`mcr-protocol client light-node-proto unstake where`↴](#mcr-protocol-client-light-node-proto-unstake-where)
* [`mcr-protocol client light-node-proto unstake using`↴](#mcr-protocol-client-light-node-proto-unstake-using)
* [`mcr-protocol client light-node-proto grant-trusted-attester`↴](#mcr-protocol-client-light-node-proto-grant-trusted-attester)
* [`mcr-protocol client light-node-proto grant-trusted-attester where`↴](#mcr-protocol-client-light-node-proto-grant-trusted-attester-where)
* [`mcr-protocol client light-node-proto grant-trusted-attester using`↴](#mcr-protocol-client-light-node-proto-grant-trusted-attester-using)
* [`mcr-protocol deployer`↴](#mcr-protocol-deployer)
* [`mcr-protocol deployer markdown`↴](#mcr-protocol-deployer-markdown)
* [`mcr-protocol deployer markdown generate`↴](#mcr-protocol-deployer-markdown-generate)
* [`mcr-protocol deployer markdown file`↴](#mcr-protocol-deployer-markdown-file)
* [`mcr-protocol deployer markdown print`↴](#mcr-protocol-deployer-markdown-print)
* [`mcr-protocol deployer markdown workspace`↴](#mcr-protocol-deployer-markdown-workspace)
* [`mcr-protocol deployer eth`↴](#mcr-protocol-deployer-eth)
* [`mcr-protocol deployer eth apply`↴](#mcr-protocol-deployer-eth-apply)
* [`mcr-protocol deployer eth apply where`↴](#mcr-protocol-deployer-eth-apply-where)
* [`mcr-protocol deployer eth apply using`↴](#mcr-protocol-deployer-eth-apply-using)
* [`mcr-protocol deployer eth destroy`↴](#mcr-protocol-deployer-eth-destroy)

## `mcr-protocol`

The `mcr-protocol` CLI

**Usage:** `mcr-protocol [COMMAND]`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `client` — The client-specific commands of the MCR protocol
* `deployer` — The deployer-specific commands of the MCR protocol



## `mcr-protocol markdown`

Generates markdown for the CLI

**Usage:** `mcr-protocol markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-protocol markdown generate`

Generate and update the documentation

**Usage:** `mcr-protocol markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-protocol markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-protocol markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-protocol markdown print`

Print the documentation in the shell

**Usage:** `mcr-protocol markdown print`



## `mcr-protocol markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-protocol markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-protocol client`

The client-specific commands of the MCR protocol

**Usage:** `mcr-protocol client <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `eth` — Ethereum-specific commands of the protocol, such as staking and committing
* `light-node-proto` — Light node protocol commands, such as staking and committing



## `mcr-protocol client markdown`

Generates markdown for the CLI

**Usage:** `mcr-protocol client markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-protocol client markdown generate`

Generate and update the documentation

**Usage:** `mcr-protocol client markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-protocol client markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-protocol client markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-protocol client markdown print`

Print the documentation in the shell

**Usage:** `mcr-protocol client markdown print`



## `mcr-protocol client markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-protocol client markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-protocol client eth`

Ethereum-specific commands of the protocol, such as staking and committing

**Usage:** `mcr-protocol client eth <COMMAND>`

###### **Subcommands:**

* `exln` — Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be externalized
* `emln` — Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be internalized



## `mcr-protocol client eth exln`

Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be externalized

**Usage:** `mcr-protocol client eth exln <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/eth/exln/mod.rs

###### **Subcommands:**

* `post-admin-commitment` — Force a commitment (admin only)
* `post-commitment` — Post a single commitment
* `post-commitment-batch` — Post a batch of commitments
* `stream-commitments` — Stream commitments
* `get-commitment` — Get a commitment for a given height and attester
* `get-accepted-commitment-at-height` — Get accepted commitment at a specific height
* `get-posted-commitment-at-height` — Get posted commitment at a specific height
* `get-max-tolerable-commitment-height` — Get max tolerable commitment height
* `stake` — Stake tokens for the MCR domain
* `get-stake` — Get the current epoch stake for an attester
* `unstake` — Unstake tokens from the MCR domain
* `grant-trusted-attester` — Grant TRUSTED_ATTESTER role to an attester



## `mcr-protocol client eth exln post-admin-commitment`

Force a commitment (admin only)

**Usage:** `mcr-protocol client eth exln post-admin-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln post-admin-commitment where`

Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-admin-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln post-admin-commitment using`

Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-admin-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln post-commitment`

Post a single commitment

**Usage:** `mcr-protocol client eth exln post-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln post-commitment where`

Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln post-commitment using`

Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-commitment using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln post-commitment-batch`

Post a batch of commitments

**Usage:** `mcr-protocol client eth exln post-commitment-batch <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln post-commitment-batch where`

Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-commitment-batch where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln post-commitment-batch using`

Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln post-commitment-batch using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth exln stream-commitments`

Stream commitments

**Usage:** `mcr-protocol client eth exln stream-commitments <COMMAND>`

###### **Subcommands:**

* `where` — Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln stream-commitments where`

Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln stream-commitments where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `mcr-protocol client eth exln stream-commitments using`

Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln stream-commitments using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client eth exln get-commitment`

Get a commitment for a given height and attester

**Usage:** `mcr-protocol client eth exln get-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln get-commitment where`

Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT> --attester <ATTESTER>`

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
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth exln get-commitment using`

Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-commitment using [OPTIONS] --height <HEIGHT> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth exln get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `mcr-protocol client eth exln get-accepted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln get-accepted-commitment-at-height where`

Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-accepted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT>`

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
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth exln get-accepted-commitment-at-height using`

Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-accepted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth exln get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `mcr-protocol client eth exln get-posted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln get-posted-commitment-at-height where`

Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-posted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --height <HEIGHT>`

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
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth exln get-posted-commitment-at-height using`

Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-posted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth exln get-max-tolerable-commitment-height`

Get max tolerable commitment height

**Usage:** `mcr-protocol client eth exln get-max-tolerable-commitment-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln get-max-tolerable-commitment-height where`

Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-max-tolerable-commitment-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `mcr-protocol client eth exln get-max-tolerable-commitment-height using`

Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-max-tolerable-commitment-height using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client eth exln stake`

Stake tokens for the MCR domain

**Usage:** `mcr-protocol client eth exln stake <COMMAND>`

###### **Subcommands:**

* `where` — Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln stake where`

Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client eth exln stake using`

Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln stake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client eth exln get-stake`

Get the current epoch stake for an attester

**Usage:** `mcr-protocol client eth exln get-stake <COMMAND>`

###### **Subcommands:**

* `where` — Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln get-stake where`

Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --custodian <CUSTODIAN> --attester <ATTESTER>`

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
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth exln get-stake using`

Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln get-stake using [OPTIONS] --custodian <CUSTODIAN> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth exln unstake`

Unstake tokens from the MCR domain

**Usage:** `mcr-protocol client eth exln unstake <COMMAND>`

###### **Subcommands:**

* `where` — Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln unstake where`

Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln unstake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client eth exln unstake using`

Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln unstake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client eth exln grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `mcr-protocol client eth exln grant-trusted-attester <COMMAND>`

###### **Subcommands:**

* `where` — Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth exln grant-trusted-attester where`

Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln grant-trusted-attester where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --attester <ATTESTER>`

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
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol client eth exln grant-trusted-attester using`

Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth exln grant-trusted-attester using [OPTIONS] --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol client eth emln`

Ethereum-specific commands of the protocol wherein light node assumptions are assumed to be internalized

**Usage:** `mcr-protocol client eth emln <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/eth/emln/mod.rs

###### **Subcommands:**

* `post-admin-commitment` — Force a commitment (admin only)
* `post-commitment` — Post a single commitment
* `post-commitment-batch` — Post a batch of commitments
* `stream-commitments` — Stream commitments
* `get-commitment` — Get a commitment for a given height and attester
* `get-accepted-commitment-at-height` — Get accepted commitment at a specific height
* `get-posted-commitment-at-height` — Get posted commitment at a specific height
* `get-max-tolerable-commitment-height` — Get max tolerable commitment height
* `stake` — Stake tokens for the MCR domain
* `get-stake` — Get the current epoch stake for an attester
* `unstake` — Unstake tokens from the MCR domain
* `grant-trusted-attester` — Grant TRUSTED_ATTESTER role to an attester



## `mcr-protocol client eth emln post-admin-commitment`

Force a commitment (admin only)

**Usage:** `mcr-protocol client eth emln post-admin-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln post-admin-commitment where`

Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-admin-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln post-admin-commitment using`

Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-admin-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln post-commitment`

Post a single commitment

**Usage:** `mcr-protocol client eth emln post-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln post-commitment where`

Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln post-commitment using`

Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-commitment using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln post-commitment-batch`

Post a batch of commitments

**Usage:** `mcr-protocol client eth emln post-commitment-batch <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln post-commitment-batch where`

Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-commitment-batch where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --id <ID> --vote <VOTE>`

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
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln post-commitment-batch using`

Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln post-commitment-batch using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client eth emln stream-commitments`

Stream commitments

**Usage:** `mcr-protocol client eth emln stream-commitments <COMMAND>`

###### **Subcommands:**

* `where` — Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln stream-commitments where`

Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln stream-commitments where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY>`

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
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node



## `mcr-protocol client eth emln stream-commitments using`

Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln stream-commitments using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client eth emln get-commitment`

Get a commitment for a given height and attester

**Usage:** `mcr-protocol client eth emln get-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln get-commitment where`

Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --height <HEIGHT> --attester <ATTESTER>`

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
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth emln get-commitment using`

Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-commitment using [OPTIONS] --height <HEIGHT> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth emln get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `mcr-protocol client eth emln get-accepted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln get-accepted-commitment-at-height where`

Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-accepted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --height <HEIGHT>`

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
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth emln get-accepted-commitment-at-height using`

Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-accepted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth emln get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `mcr-protocol client eth emln get-posted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln get-posted-commitment-at-height where`

Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-posted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --height <HEIGHT>`

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
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth emln get-posted-commitment-at-height using`

Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-posted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth emln get-max-tolerable-commitment-height`

Get max tolerable commitment height

**Usage:** `mcr-protocol client eth emln get-max-tolerable-commitment-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln get-max-tolerable-commitment-height where`

Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-max-tolerable-commitment-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY>`

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
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node



## `mcr-protocol client eth emln get-max-tolerable-commitment-height using`

Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-max-tolerable-commitment-height using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client eth emln stake`

Stake tokens for the MCR domain

**Usage:** `mcr-protocol client eth emln stake <COMMAND>`

###### **Subcommands:**

* `where` — Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln stake where`

Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --amount <AMOUNT>`

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
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client eth emln stake using`

Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln stake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client eth emln get-stake`

Get the current epoch stake for an attester

**Usage:** `mcr-protocol client eth emln get-stake <COMMAND>`

###### **Subcommands:**

* `where` — Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln get-stake where`

Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --custodian <CUSTODIAN> --attester <ATTESTER>`

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
* `--light-node-data-dir <LIGHT_NODE_DATA_DIR>` — The directory to store the light node data
* `--light-node-network <LIGHT_NODE_NETWORK>` — The Ethereum network type to use for light node consensus parameterizaton

  Default value: `mainnet`
* `--consensus-rpc-url <CONSENSUS_RPC_URL>` — The consensus RPC for the light node to use
* `--finality <FINALITY>` — The finality configuration for the light node
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth emln get-stake using`

Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln get-stake using [OPTIONS] --custodian <CUSTODIAN> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth emln unstake`

Unstake tokens from the MCR domain

**Usage:** `mcr-protocol client eth emln unstake <COMMAND>`

###### **Subcommands:**

* `where` — Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln unstake where`

Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln unstake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --amount <AMOUNT>`

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
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client eth emln unstake using`

Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln unstake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client eth emln grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `mcr-protocol client eth emln grant-trusted-attester <COMMAND>`

###### **Subcommands:**

* `where` — Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth emln grant-trusted-attester where`

Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln grant-trusted-attester where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --light-node-data-dir <LIGHT_NODE_DATA_DIR> --consensus-rpc-url <CONSENSUS_RPC_URL> --finality <FINALITY> --attester <ATTESTER>`

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
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol client eth emln grant-trusted-attester using`

Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth emln grant-trusted-attester using [OPTIONS] --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol client light-node-proto`

Light node protocol commands, such as staking and committing

**Usage:** `mcr-protocol client light-node-proto <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/light_node_proto/mod.rs

###### **Subcommands:**

* `post-admin-commitment` — Force a commitment (admin only)
* `post-commitment` — Post a single commitment
* `post-commitment-batch` — Post a batch of commitments
* `stream-commitments` — Stream commitments
* `get-commitment` — Get a commitment for a given height and attester
* `get-accepted-commitment-at-height` — Get accepted commitment at a specific height
* `get-posted-commitment-at-height` — Get posted commitment at a specific height
* `get-max-tolerable-commitment-height` — Get max tolerable commitment height
* `stake` — Stake tokens for the MCR domain
* `get-stake` — Get the current epoch stake for an attester
* `unstake` — Unstake tokens from the MCR domain
* `grant-trusted-attester` — Grant TRUSTED_ATTESTER role to an attester



## `mcr-protocol client light-node-proto post-admin-commitment`

Force a commitment (admin only)

**Usage:** `mcr-protocol client light-node-proto post-admin-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto post-admin-commitment where`

Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-admin-commitment where [OPTIONS] --endpoint <ENDPOINT> --id <ID> --vote <VOTE>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto post-admin-commitment using`

Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-admin-commitment using [OPTIONS] --endpoint <ENDPOINT> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto post-commitment`

Post a single commitment

**Usage:** `mcr-protocol client light-node-proto post-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto post-commitment where`

Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-commitment where [OPTIONS] --endpoint <ENDPOINT> --id <ID> --vote <VOTE>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto post-commitment using`

Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-commitment using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto post-commitment-batch`

Post a batch of commitments

**Usage:** `mcr-protocol client light-node-proto post-commitment-batch <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto post-commitment-batch where`

Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-commitment-batch where [OPTIONS] --endpoint <ENDPOINT> --id <ID> --vote <VOTE>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto post-commitment-batch using`

Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto post-commitment-batch using [OPTIONS] --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--height <HEIGHT>` — The commitment height at which to commit
* `--id <ID>` — The commitment id to commit
* `--vote <VOTE>` — The commitment value to commit



## `mcr-protocol client light-node-proto stream-commitments`

Stream commitments

**Usage:** `mcr-protocol client light-node-proto stream-commitments <COMMAND>`

###### **Subcommands:**

* `where` — Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto stream-commitments where`

Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto stream-commitments where [OPTIONS] --endpoint <ENDPOINT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`



## `mcr-protocol client light-node-proto stream-commitments using`

Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto stream-commitments using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client light-node-proto get-commitment`

Get a commitment for a given height and attester

**Usage:** `mcr-protocol client light-node-proto get-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto get-commitment where`

Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-commitment where [OPTIONS] --endpoint <ENDPOINT> --height <HEIGHT> --attester <ATTESTER>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client light-node-proto get-commitment using`

Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-commitment using [OPTIONS] --height <HEIGHT> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client light-node-proto get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `mcr-protocol client light-node-proto get-accepted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto get-accepted-commitment-at-height where`

Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-accepted-commitment-at-height where [OPTIONS] --endpoint <ENDPOINT> --height <HEIGHT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client light-node-proto get-accepted-commitment-at-height using`

Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-accepted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client light-node-proto get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `mcr-protocol client light-node-proto get-posted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto get-posted-commitment-at-height where`

Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-posted-commitment-at-height where [OPTIONS] --endpoint <ENDPOINT> --height <HEIGHT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client light-node-proto get-posted-commitment-at-height using`

Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-posted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client light-node-proto get-max-tolerable-commitment-height`

Get max tolerable commitment height

**Usage:** `mcr-protocol client light-node-proto get-max-tolerable-commitment-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto get-max-tolerable-commitment-height where`

Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-max-tolerable-commitment-height where [OPTIONS] --endpoint <ENDPOINT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`



## `mcr-protocol client light-node-proto get-max-tolerable-commitment-height using`

Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-max-tolerable-commitment-height using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`



## `mcr-protocol client light-node-proto stake`

Stake tokens for the MCR domain

**Usage:** `mcr-protocol client light-node-proto stake <COMMAND>`

###### **Subcommands:**

* `where` — Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto stake where`

Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto stake where [OPTIONS] --endpoint <ENDPOINT> --amount <AMOUNT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client light-node-proto stake using`

Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto stake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client light-node-proto get-stake`

Get the current epoch stake for an attester

**Usage:** `mcr-protocol client light-node-proto get-stake <COMMAND>`

###### **Subcommands:**

* `where` — Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto get-stake where`

Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-stake where [OPTIONS] --endpoint <ENDPOINT> --custodian <CUSTODIAN> --attester <ATTESTER>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client light-node-proto get-stake using`

Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto get-stake using [OPTIONS] --custodian <CUSTODIAN> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client light-node-proto unstake`

Unstake tokens from the MCR domain

**Usage:** `mcr-protocol client light-node-proto unstake <COMMAND>`

###### **Subcommands:**

* `where` — Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto unstake where`

Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto unstake where [OPTIONS] --endpoint <ENDPOINT> --amount <AMOUNT>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client light-node-proto unstake using`

Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto unstake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client light-node-proto grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `mcr-protocol client light-node-proto grant-trusted-attester <COMMAND>`

###### **Subcommands:**

* `where` — Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client light-node-proto grant-trusted-attester where`

Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto grant-trusted-attester where [OPTIONS] --endpoint <ENDPOINT> --attester <ATTESTER>`

###### **Options:**

* `--endpoint <ENDPOINT>` — The gRPC endpoint URL for the light node service
* `--timeout-ms <TIMEOUT_MS>` — The timeout for gRPC requests in milliseconds

  Default value: `5000`
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol client light-node-proto grant-trusted-attester using`

Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client light-node-proto grant-trusted-attester using [OPTIONS] --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--attester <ATTESTER>` — The attester address to grant the role to



## `mcr-protocol deployer`

The deployer-specific commands of the MCR protocol

**Usage:** `mcr-protocol deployer <COMMAND>`

###### **Subcommands:**

* `markdown` — 
* `eth` — 



## `mcr-protocol deployer markdown`

**Usage:** `mcr-protocol deployer markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-protocol deployer markdown generate`

Generate and update the documentation

**Usage:** `mcr-protocol deployer markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-protocol deployer markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-protocol deployer markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-protocol deployer markdown print`

Print the documentation in the shell

**Usage:** `mcr-protocol deployer markdown print`



## `mcr-protocol deployer markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-protocol deployer markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-protocol deployer eth`

**Usage:** `mcr-protocol deployer eth <COMMAND>`

###### **Subcommands:**

* `apply` — 
* `destroy` — 



## `mcr-protocol deployer eth apply`

**Usage:** `mcr-protocol deployer eth apply <COMMAND>`

###### **Subcommands:**

* `where` — Run config with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run config with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol deployer eth apply where`

Run config with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol deployer eth apply where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

###### **Options:**

* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--contract-admin <CONTRACT_ADMIN>` — Admin address for deployed contracts
* `--token-name <TOKEN_NAME>` — The token name

  Default value: `Move Token`
* `--token-symbol <TOKEN_SYMBOL>` — The token symbol

  Default value: `MOVE`
* `--initial-token-mint <INITIAL_TOKEN_MINT>` — The initial token mint

  Default value: `1000000000000000000000000`
* `--custodians <CUSTODIANS>` — The custodians By default this should be an empty vector
* `--initial-commitment-height <INITIAL_COMMITMENT_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-commitment-tolerance <LEADING_COMMITMENT_TOLERANCE>` — The leading block tolerance

  Default value: `10`
* `--epoch-duration <EPOCH_DURATION>` — The epoch duration

  Default value: `1000000`
* `--reward-contract <REWARD_CONTRACT>` — The reward contract
* `--proxy-admin <PROXY_ADMIN>` — The existing proxy admin
* `--token-proxy <TOKEN_PROXY>` — The existing move token proxy
* `--staking-proxy <STAKING_PROXY>` — The existing staking proxy
* `--mcr-proxy <MCR_PROXY>` — The existing MCR proxy
* `--reward-proxy <REWARD_PROXY>` — The existing ARO proxy
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `mcr-protocol deployer eth apply using`

Run config with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol deployer eth apply using [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--script-args-path <SCRIPT_ARGS_PATH>`
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `mcr-protocol deployer eth destroy`

**Usage:** `mcr-protocol deployer eth destroy [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --proxy-admin <PROXY_ADMIN> --token-proxy <TOKEN_PROXY> --staking-proxy <STAKING_PROXY> --mcr-proxy <MCR_PROXY> --reward-proxy <REWARD_PROXY>`

###### **Options:**

* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--proxy-admin <PROXY_ADMIN>` — The proxy admin
* `--token-proxy <TOKEN_PROXY>` — The move token proxy
* `--staking-proxy <STAKING_PROXY>` — The staking proxy
* `--mcr-proxy <MCR_PROXY>` — The mcr proxy
* `--reward-proxy <REWARD_PROXY>` — The reward proxy
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

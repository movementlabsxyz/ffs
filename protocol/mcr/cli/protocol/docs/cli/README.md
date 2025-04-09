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
* [`mcr-protocol client eth post-admin-commitment`↴](#mcr-protocol-client-eth-post-admin-commitment)
* [`mcr-protocol client eth post-admin-commitment where`↴](#mcr-protocol-client-eth-post-admin-commitment-where)
* [`mcr-protocol client eth post-admin-commitment using`↴](#mcr-protocol-client-eth-post-admin-commitment-using)
* [`mcr-protocol client eth post-commitment`↴](#mcr-protocol-client-eth-post-commitment)
* [`mcr-protocol client eth post-commitment where`↴](#mcr-protocol-client-eth-post-commitment-where)
* [`mcr-protocol client eth post-commitment using`↴](#mcr-protocol-client-eth-post-commitment-using)
* [`mcr-protocol client eth post-commitment-batch`↴](#mcr-protocol-client-eth-post-commitment-batch)
* [`mcr-protocol client eth post-commitment-batch where`↴](#mcr-protocol-client-eth-post-commitment-batch-where)
* [`mcr-protocol client eth post-commitment-batch using`↴](#mcr-protocol-client-eth-post-commitment-batch-using)
* [`mcr-protocol client eth stream-commitments`↴](#mcr-protocol-client-eth-stream-commitments)
* [`mcr-protocol client eth stream-commitments where`↴](#mcr-protocol-client-eth-stream-commitments-where)
* [`mcr-protocol client eth stream-commitments using`↴](#mcr-protocol-client-eth-stream-commitments-using)
* [`mcr-protocol client eth get-commitment`↴](#mcr-protocol-client-eth-get-commitment)
* [`mcr-protocol client eth get-commitment where`↴](#mcr-protocol-client-eth-get-commitment-where)
* [`mcr-protocol client eth get-commitment using`↴](#mcr-protocol-client-eth-get-commitment-using)
* [`mcr-protocol client eth get-accepted-commitment-at-height`↴](#mcr-protocol-client-eth-get-accepted-commitment-at-height)
* [`mcr-protocol client eth get-accepted-commitment-at-height where`↴](#mcr-protocol-client-eth-get-accepted-commitment-at-height-where)
* [`mcr-protocol client eth get-accepted-commitment-at-height using`↴](#mcr-protocol-client-eth-get-accepted-commitment-at-height-using)
* [`mcr-protocol client eth get-posted-commitment-at-height`↴](#mcr-protocol-client-eth-get-posted-commitment-at-height)
* [`mcr-protocol client eth get-posted-commitment-at-height where`↴](#mcr-protocol-client-eth-get-posted-commitment-at-height-where)
* [`mcr-protocol client eth get-posted-commitment-at-height using`↴](#mcr-protocol-client-eth-get-posted-commitment-at-height-using)
* [`mcr-protocol client eth get-max-tolerable-commitment-height`↴](#mcr-protocol-client-eth-get-max-tolerable-commitment-height)
* [`mcr-protocol client eth get-max-tolerable-commitment-height where`↴](#mcr-protocol-client-eth-get-max-tolerable-commitment-height-where)
* [`mcr-protocol client eth get-max-tolerable-commitment-height using`↴](#mcr-protocol-client-eth-get-max-tolerable-commitment-height-using)
* [`mcr-protocol client eth stake`↴](#mcr-protocol-client-eth-stake)
* [`mcr-protocol client eth stake where`↴](#mcr-protocol-client-eth-stake-where)
* [`mcr-protocol client eth stake using`↴](#mcr-protocol-client-eth-stake-using)
* [`mcr-protocol client eth get-stake`↴](#mcr-protocol-client-eth-get-stake)
* [`mcr-protocol client eth get-stake where`↴](#mcr-protocol-client-eth-get-stake-where)
* [`mcr-protocol client eth get-stake using`↴](#mcr-protocol-client-eth-get-stake-using)
* [`mcr-protocol client eth unstake`↴](#mcr-protocol-client-eth-unstake)
* [`mcr-protocol client eth unstake where`↴](#mcr-protocol-client-eth-unstake-where)
* [`mcr-protocol client eth unstake using`↴](#mcr-protocol-client-eth-unstake-using)
* [`mcr-protocol client eth grant-trusted-attester`↴](#mcr-protocol-client-eth-grant-trusted-attester)
* [`mcr-protocol client eth grant-trusted-attester where`↴](#mcr-protocol-client-eth-grant-trusted-attester-where)
* [`mcr-protocol client eth grant-trusted-attester using`↴](#mcr-protocol-client-eth-grant-trusted-attester-using)
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

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/eth/mod.rs

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



## `mcr-protocol client eth post-admin-commitment`

Force a commitment (admin only)

**Usage:** `mcr-protocol client eth post-admin-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth post-admin-commitment where`

Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-admin-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `mcr-protocol client eth post-admin-commitment using`

Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-admin-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `mcr-protocol client eth post-commitment`

Post a single commitment

**Usage:** `mcr-protocol client eth post-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth post-commitment where`

Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `mcr-protocol client eth post-commitment using`

Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `mcr-protocol client eth post-commitment-batch`

Post a batch of commitments

**Usage:** `mcr-protocol client eth post-commitment-batch <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth post-commitment-batch where`

Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-commitment-batch where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `mcr-protocol client eth post-commitment-batch using`

Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth post-commitment-batch using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `mcr-protocol client eth stream-commitments`

Stream commitments

**Usage:** `mcr-protocol client eth stream-commitments <COMMAND>`

###### **Subcommands:**

* `where` — Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth stream-commitments where`

Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth stream-commitments where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `mcr-protocol client eth stream-commitments using`

Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth stream-commitments using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> [EXTRA_ARGS]...`

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



## `mcr-protocol client eth get-commitment`

Get a commitment for a given height and attester

**Usage:** `mcr-protocol client eth get-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth get-commitment where`

Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT> --attester <ATTESTER>`

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
* `--height <HEIGHT>` — Block height to check commitment for
* `--attester <ATTESTER>` — Attester address to check commitment for



## `mcr-protocol client eth get-commitment using`

Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-commitment using [OPTIONS] --height <HEIGHT> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — Block height to check commitment for
* `--attester <ATTESTER>` — Attester address to check commitment for



## `mcr-protocol client eth get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `mcr-protocol client eth get-accepted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth get-accepted-commitment-at-height where`

Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-accepted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT>`

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



## `mcr-protocol client eth get-accepted-commitment-at-height using`

Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-accepted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `mcr-protocol client eth get-posted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth get-posted-commitment-at-height where`

Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-posted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --height <HEIGHT>`

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



## `mcr-protocol client eth get-posted-commitment-at-height using`

Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-posted-commitment-at-height using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --height <HEIGHT> [EXTRA_ARGS]...`

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
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol client eth get-max-tolerable-commitment-height`

Get max tolerable commitment height

**Usage:** `mcr-protocol client eth get-max-tolerable-commitment-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth get-max-tolerable-commitment-height where`

Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-max-tolerable-commitment-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `mcr-protocol client eth get-max-tolerable-commitment-height using`

Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-max-tolerable-commitment-height using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> [EXTRA_ARGS]...`

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



## `mcr-protocol client eth stake`

Stake tokens for the MCR domain

**Usage:** `mcr-protocol client eth stake <COMMAND>`

###### **Subcommands:**

* `where` — Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth stake where`

Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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



## `mcr-protocol client eth stake using`

Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth stake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol client eth get-stake`

Get the current epoch stake for an attester

**Usage:** `mcr-protocol client eth get-stake <COMMAND>`

###### **Subcommands:**

* `where` — Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth get-stake where`

Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --custodian <CUSTODIAN> --attester <ATTESTER>`

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



## `mcr-protocol client eth get-stake using`

Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth get-stake using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --custodian <CUSTODIAN> --attester <ATTESTER> [EXTRA_ARGS]...`

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
* `--custodian <CUSTODIAN>` — The custodian address
* `--attester <ATTESTER>` — The attester address



## `mcr-protocol client eth unstake`

Unstake tokens from the MCR domain

**Usage:** `mcr-protocol client eth unstake <COMMAND>`

###### **Subcommands:**

* `where` — Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth unstake where`

Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth unstake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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



## `mcr-protocol client eth unstake using`

Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth unstake using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT> [EXTRA_ARGS]...`

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
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol client eth grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `mcr-protocol client eth grant-trusted-attester <COMMAND>`

###### **Subcommands:**

* `where` — Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-protocol client eth grant-trusted-attester where`

Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth grant-trusted-attester where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --attester <ATTESTER>`

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
* `--attester <ATTESTER>` — The address to grant TRUSTED_ATTESTER role to



## `mcr-protocol client eth grant-trusted-attester using`

Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-protocol client eth grant-trusted-attester using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --attester <ATTESTER> [EXTRA_ARGS]...`

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
* `--attester <ATTESTER>` — The address to grant TRUSTED_ATTESTER role to



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

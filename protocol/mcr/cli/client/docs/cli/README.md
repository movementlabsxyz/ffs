# Command-Line Help for `mcr-protocol-client`

This document contains the help content for the `mcr-protocol-client` command-line program.

**Command Overview:**

* [`mcr-protocol-client`↴](#mcr-protocol-client)
* [`mcr-protocol-client markdown`↴](#mcr-protocol-client-markdown)
* [`mcr-protocol-client markdown generate`↴](#mcr-protocol-client-markdown-generate)
* [`mcr-protocol-client markdown file`↴](#mcr-protocol-client-markdown-file)
* [`mcr-protocol-client markdown print`↴](#mcr-protocol-client-markdown-print)
* [`mcr-protocol-client markdown workspace`↴](#mcr-protocol-client-markdown-workspace)
* [`mcr-protocol-client eth`↴](#mcr-protocol-client-eth)
* [`mcr-protocol-client eth post-admin-commitment`↴](#mcr-protocol-client-eth-post-admin-commitment)
* [`mcr-protocol-client eth post-commitment`↴](#mcr-protocol-client-eth-post-commitment)
* [`mcr-protocol-client eth post-commitment-batch`↴](#mcr-protocol-client-eth-post-commitment-batch)
* [`mcr-protocol-client eth stream-commitments`↴](#mcr-protocol-client-eth-stream-commitments)
* [`mcr-protocol-client eth get-block-commitment`↴](#mcr-protocol-client-eth-get-block-commitment)
* [`mcr-protocol-client eth get-accepted-commitment-at-height`↴](#mcr-protocol-client-eth-get-accepted-commitment-at-height)
* [`mcr-protocol-client eth get-posted-commitment-at-height`↴](#mcr-protocol-client-eth-get-posted-commitment-at-height)
* [`mcr-protocol-client eth get-max-tolerable-block-height`↴](#mcr-protocol-client-eth-get-max-tolerable-block-height)
* [`mcr-protocol-client eth stake`↴](#mcr-protocol-client-eth-stake)
* [`mcr-protocol-client eth get-stake`↴](#mcr-protocol-client-eth-get-stake)
* [`mcr-protocol-client eth unstake`↴](#mcr-protocol-client-eth-unstake)
* [`mcr-protocol-client eth grant-trusted-attester`↴](#mcr-protocol-client-eth-grant-trusted-attester)

## `mcr-protocol-client`

The `mcr-protocol-client` CLI

**Usage:** `mcr-protocol-client [COMMAND]`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `eth` — Ethereum-specific commands of the protocol, such as staking and committing



## `mcr-protocol-client markdown`

Generates markdown for the CLI

**Usage:** `mcr-protocol-client markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-protocol-client markdown generate`

Generate and update the documentation

**Usage:** `mcr-protocol-client markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-protocol-client markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-protocol-client markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-protocol-client markdown print`

Print the documentation in the shell

**Usage:** `mcr-protocol-client markdown print`



## `mcr-protocol-client markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-protocol-client markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-protocol-client eth`

Ethereum-specific commands of the protocol, such as staking and committing

**Usage:** `mcr-protocol-client eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/eth/mod.rs

###### **Subcommands:**

* `post-admin-commitment` — Force a block commitment (admin only)
* `post-commitment` — Post a single commitment
* `post-commitment-batch` — Post a batch of commitments
* `stream-commitments` — Stream commitments
* `get-block-commitment` — Get a block commitment for a given height and attester
* `get-accepted-commitment-at-height` — Get accepted commitment at a specific height
* `get-posted-commitment-at-height` — Get posted commitment at a specific height
* `get-max-tolerable-block-height` — Get max tolerable superBlock height
* `stake` — Stake tokens for the MCR domain
* `get-stake` — Get the current epoch stake for an attester
* `unstake` — Unstake tokens from the MCR domain
* `grant-trusted-attester` — Grant TRUSTED_ATTESTER role to an attester



## `mcr-protocol-client eth post-admin-commitment`

Force a block commitment (admin only)

**Usage:** `mcr-protocol-client eth post-admin-commitment [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --id <ID> --commitment <COMMITMENT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--height <HEIGHT>` — The height of the commitment block at which to commit
* `--id <ID>` — The id of the commitment block at which to commit
* `--commitment <COMMITMENT>` — The commitment to commit



## `mcr-protocol-client eth post-commitment`

Post a single commitment

**Usage:** `mcr-protocol-client eth post-commitment [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT> --id <ID> --commitment <COMMITMENT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--height <HEIGHT>` — The height of the commitment block at which to commit
* `--id <ID>` — The id of the commitment block at which to commit
* `--commitment <COMMITMENT>` — The commitment to commit



## `mcr-protocol-client eth post-commitment-batch`

Post a batch of commitments

**Usage:** `mcr-protocol-client eth post-commitment-batch [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT> --id <ID> --commitment <COMMITMENT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--height <HEIGHT>` — The height of the block to commit
* `--id <ID>` — The id of the block to commit
* `--commitment <COMMITMENT>` — The commitment to commit



## `mcr-protocol-client eth stream-commitments`

Stream commitments

**Usage:** `mcr-protocol-client eth stream-commitments [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address



## `mcr-protocol-client eth get-block-commitment`

Get a block commitment for a given height and attester

**Usage:** `mcr-protocol-client eth get-block-commitment [OPTIONS] --height <HEIGHT> --attester <ATTESTER> --mcr-address <MCR_ADDRESS>`

###### **Options:**

* `--height <HEIGHT>` — Block height to check commitment for
* `--attester <ATTESTER>` — Attester address to check commitment for
* `--mcr-address <MCR_ADDRESS>` — MCR contract address
* `--rpc-url <RPC_URL>` — RPC URL (optional, defaults to http://localhost:8545)

  Default value: `http://localhost:8545`
* `--private-key <PRIVATE_KEY>` — Private key for signing transactions (optional)



## `mcr-protocol-client eth get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `mcr-protocol-client eth get-accepted-commitment-at-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol-client eth get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `mcr-protocol-client eth get-posted-commitment-at-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--height <HEIGHT>` — The height to get the commitment for



## `mcr-protocol-client eth get-max-tolerable-block-height`

Get max tolerable superBlock height

**Usage:** `mcr-protocol-client eth get-max-tolerable-block-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address



## `mcr-protocol-client eth stake`

Stake tokens for the MCR domain

**Usage:** `mcr-protocol-client eth stake [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --amount <AMOUNT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--amount <AMOUNT>` — Amount to stake



## `mcr-protocol-client eth get-stake`

Get the current epoch stake for an attester

**Usage:** `mcr-protocol-client eth get-stake [OPTIONS] --attester <ATTESTER> --custodian <CUSTODIAN> --mcr-address <MCR_ADDRESS>`

###### **Options:**

* `--private-key <PRIVATE_KEY>` — Private key for signing transactions (optional)

  Default value: `0x1111111111111111111111111111111111111111111111111111111111111111`
* `--rpc-url <RPC_URL>` — RPC URL (optional, defaults to http://localhost:8545)

  Default value: `http://localhost:8545`
* `--attester <ATTESTER>` — The attester address
* `--custodian <CUSTODIAN>` — The custodian (MOVE token) address
* `--mcr-address <MCR_ADDRESS>` — The MCR contract address



## `mcr-protocol-client eth unstake`

Unstake tokens from the MCR domain

**Usage:** `mcr-protocol-client eth unstake [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --block-lead-tolerance <BLOCK_LEAD_TOLERANCE> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --amount <AMOUNT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--mcr-address <MCR_ADDRESS>` — The MCR address
* `--block-lead-tolerance <BLOCK_LEAD_TOLERANCE>` — The block lead tolerance
* `--move-token-address <MOVE_TOKEN_ADDRESS>` — The move token address
* `--staking-address <STAKING_ADDRESS>` — The staking address
* `--amount <AMOUNT>` — Amount to unstake



## `mcr-protocol-client eth grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `mcr-protocol-client eth grant-trusted-attester --attester <ATTESTER> --mcr-address <MCR_ADDRESS> --private-key <PRIVATE_KEY>`

###### **Options:**

* `--attester <ATTESTER>` — The address to grant TRUSTED_ATTESTER role to
* `--mcr-address <MCR_ADDRESS>` — The MCR contract address
* `--private-key <PRIVATE_KEY>` — The private key to use for signing transactions



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

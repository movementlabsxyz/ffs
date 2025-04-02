# Command-Line Help for `ffs-dev`

This document contains the help content for the `ffs-dev` command-line program.

**Command Overview:**

* [`ffs-dev`↴](#ffs-dev)
* [`ffs-dev markdown`↴](#ffs-dev-markdown)
* [`ffs-dev markdown generate`↴](#ffs-dev-markdown-generate)
* [`ffs-dev markdown file`↴](#ffs-dev-markdown-file)
* [`ffs-dev markdown print`↴](#ffs-dev-markdown-print)
* [`ffs-dev markdown workspace`↴](#ffs-dev-markdown-workspace)
* [`ffs-dev mcr`↴](#ffs-dev-mcr)
* [`ffs-dev mcr network`↴](#ffs-dev-mcr-network)
* [`ffs-dev mcr network run`↴](#ffs-dev-mcr-network-run)
* [`ffs-dev mcr network client`↴](#ffs-dev-mcr-network-client)
* [`ffs-dev mcr network client run`↴](#ffs-dev-mcr-network-client-run)
* [`ffs-dev mcr network coordinator`↴](#ffs-dev-mcr-network-coordinator)
* [`ffs-dev mcr network coordinator run`↴](#ffs-dev-mcr-network-coordinator-run)
* [`ffs-dev mcr network coordinator eth`↴](#ffs-dev-mcr-network-coordinator-eth)
* [`ffs-dev mcr network coordinator eth anvil`↴](#ffs-dev-mcr-network-coordinator-eth-anvil)
* [`ffs-dev mcr network coordinator eth anvil up`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up)
* [`ffs-dev mcr network coordinator eth anvil up where`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up-where)
* [`ffs-dev mcr network coordinator eth anvil up using`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up-using)
* [`ffs-dev mcr network coordinator eth live`↴](#ffs-dev-mcr-network-coordinator-eth-live)
* [`ffs-dev mcr network coordinator eth live up`↴](#ffs-dev-mcr-network-coordinator-eth-live-up)
* [`ffs-dev mcr protocol`↴](#ffs-dev-mcr-protocol)
* [`ffs-dev mcr protocol run`↴](#ffs-dev-mcr-protocol-run)
* [`ffs-dev mcr protocol client`↴](#ffs-dev-mcr-protocol-client)
* [`ffs-dev mcr protocol client run`↴](#ffs-dev-mcr-protocol-client-run)
* [`ffs-dev mcr protocol client eth`↴](#ffs-dev-mcr-protocol-client-eth)
* [`ffs-dev mcr protocol client eth post-admin-commitment`↴](#ffs-dev-mcr-protocol-client-eth-post-admin-commitment)
* [`ffs-dev mcr protocol client eth post-commitment-batch`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-batch)
* [`ffs-dev mcr protocol client eth stream-commitments`↴](#ffs-dev-mcr-protocol-client-eth-stream-commitments)
* [`ffs-dev mcr protocol client eth get-commitment-at-height`↴](#ffs-dev-mcr-protocol-client-eth-get-commitment-at-height)
* [`ffs-dev mcr protocol client eth get-posted-commitment-at-height`↴](#ffs-dev-mcr-protocol-client-eth-get-posted-commitment-at-height)
* [`ffs-dev mcr protocol client eth get-max-tolerable-block-height`↴](#ffs-dev-mcr-protocol-client-eth-get-max-tolerable-block-height)
* [`ffs-dev mcr protocol client eth stake`↴](#ffs-dev-mcr-protocol-client-eth-stake)
* [`ffs-dev mcr protocol client eth unstake`↴](#ffs-dev-mcr-protocol-client-eth-unstake)
* [`ffs-dev mcr protocol client post-commitment`↴](#ffs-dev-mcr-protocol-client-post-commitment)
* [`ffs-dev pcp`↴](#ffs-dev-pcp)
* [`ffs-dev pcp network`↴](#ffs-dev-pcp-network)
* [`ffs-dev pcp network run`↴](#ffs-dev-pcp-network-run)
* [`ffs-dev pcp network client`↴](#ffs-dev-pcp-network-client)
* [`ffs-dev pcp network client run`↴](#ffs-dev-pcp-network-client-run)
* [`ffs-dev pcp network coordinator`↴](#ffs-dev-pcp-network-coordinator)
* [`ffs-dev pcp network coordinator run`↴](#ffs-dev-pcp-network-coordinator-run)
* [`ffs-dev pcp network coordinator eth`↴](#ffs-dev-pcp-network-coordinator-eth)
* [`ffs-dev pcp network coordinator eth anvil`↴](#ffs-dev-pcp-network-coordinator-eth-anvil)
* [`ffs-dev pcp network coordinator eth anvil up`↴](#ffs-dev-pcp-network-coordinator-eth-anvil-up)
* [`ffs-dev pcp network coordinator eth anvil up where`↴](#ffs-dev-pcp-network-coordinator-eth-anvil-up-where)
* [`ffs-dev pcp network coordinator eth anvil up using`↴](#ffs-dev-pcp-network-coordinator-eth-anvil-up-using)
* [`ffs-dev pcp network coordinator eth live`↴](#ffs-dev-pcp-network-coordinator-eth-live)
* [`ffs-dev pcp network coordinator eth live up`↴](#ffs-dev-pcp-network-coordinator-eth-live-up)
* [`ffs-dev pcp protocol`↴](#ffs-dev-pcp-protocol)
* [`ffs-dev pcp protocol run`↴](#ffs-dev-pcp-protocol-run)
* [`ffs-dev pcp protocol client`↴](#ffs-dev-pcp-protocol-client)
* [`ffs-dev pcp protocol client run`↴](#ffs-dev-pcp-protocol-client-run)
* [`ffs-dev pcp protocol client eth`↴](#ffs-dev-pcp-protocol-client-eth)
* [`ffs-dev pcp protocol client eth post-admin-commitment`↴](#ffs-dev-pcp-protocol-client-eth-post-admin-commitment)
* [`ffs-dev pcp protocol client post-commitment`↴](#ffs-dev-pcp-protocol-client-post-commitment)
* [`ffs-dev pcp protocol client deploy`↴](#ffs-dev-pcp-protocol-client-deploy)
* [`ffs-dev pcp protocol client deploy anvil`↴](#ffs-dev-pcp-protocol-client-deploy-anvil)

## `ffs-dev`

**Usage:** `ffs-dev <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: sdk/cli/ffs-dev/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generate CLI documentation
* `mcr` — Manage MCR
* `pcp` — Manage PCP



## `ffs-dev markdown`

Generate CLI documentation

**Usage:** `ffs-dev markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev markdown print`



## `ffs-dev markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr`

Manage MCR

**Usage:** `ffs-dev mcr <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: sdk/cli/ffs-dev/src/cli/mcr/mod.rs

###### **Subcommands:**

* `network` — ???
* `protocol` — The subcommands of the `mcr-protocol` CLI 2



## `ffs-dev mcr network`

???

**Usage:** `ffs-dev mcr network <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/network/src/cli/mod.rs

###### **Subcommands:**

* `run` — ???
* `client` — ???
* `coordinator` — ???



## `ffs-dev mcr network run`

???

**Usage:** `ffs-dev mcr network run`



## `ffs-dev mcr network client`

???

**Usage:** `ffs-dev mcr network client <COMMAND>`

###### **Subcommands:**

* `run` — ???



## `ffs-dev mcr network client run`

???

**Usage:** `ffs-dev mcr network client run`



## `ffs-dev mcr network coordinator`

???

**Usage:** `ffs-dev mcr network coordinator <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/mod.rs

###### **Subcommands:**

* `run` — ???
* `eth` — ???



## `ffs-dev mcr network coordinator run`

???

**Usage:** `ffs-dev mcr network coordinator run`



## `ffs-dev mcr network coordinator eth`

???

**Usage:** `ffs-dev mcr network coordinator eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/mod.rs

###### **Subcommands:**

* `anvil` — ???
* `live` — ???



## `ffs-dev mcr network coordinator eth anvil`

???

**Usage:** `ffs-dev mcr network coordinator eth anvil <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/anvil/mod.rs

###### **Subcommands:**

* `up` — ???



## `ffs-dev mcr network coordinator eth anvil up`

???

**Usage:** `ffs-dev mcr network coordinator eth anvil up <COMMAND>`

###### **Subcommands:**

* `where` — Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr network coordinator eth anvil up where`

Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr network coordinator eth anvil up where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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
* `--initial-block-height <INITIAL_BLOCK_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-block-tolerance <LEADING_BLOCK_TOLERANCE>` — The leading block tolerance

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
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev mcr network coordinator eth anvil up using`

Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr network coordinator eth anvil up using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev mcr network coordinator eth live`

???

**Usage:** `ffs-dev mcr network coordinator eth live <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/live/mod.rs

###### **Subcommands:**

* `up` — ???



## `ffs-dev mcr network coordinator eth live up`

???

**Usage:** `ffs-dev mcr network coordinator eth live up [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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
* `--initial-block-height <INITIAL_BLOCK_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-block-tolerance <LEADING_BLOCK_TOLERANCE>` — The leading block tolerance

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



## `ffs-dev mcr protocol`

The subcommands of the `mcr-protocol` CLI 2

**Usage:** `ffs-dev mcr protocol <COMMAND>`

###### **Subcommands:**

* `run` — ???
* `client` — ???



## `ffs-dev mcr protocol run`

???

**Usage:** `ffs-dev mcr protocol run`



## `ffs-dev mcr protocol client`

???

**Usage:** `ffs-dev mcr protocol client <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/mod.rs

###### **Subcommands:**

* `run` — ???
* `eth` — ???
* `post-commitment` — Post a commitment to an MCR implementation



## `ffs-dev mcr protocol client run`

???

**Usage:** `ffs-dev mcr protocol client run`



## `ffs-dev mcr protocol client eth`

???

**Usage:** `ffs-dev mcr protocol client eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/eth/mod.rs

###### **Subcommands:**

* `post-admin-commitment` — Force a block commitment (admin only)
* `post-commitment-batch` — Post a batch of block commitments
* `stream-commitments` — Stream block commitments
* `get-commitment-at-height` — Get commitment at a specific height
* `get-posted-commitment-at-height` — Get posted commitment at a specific height
* `get-max-tolerable-block-height` — Get max tolerable block height
* `stake` — Stake tokens for the MCR domain
* `unstake` — Unstake tokens from the MCR domain



## `ffs-dev mcr protocol client eth post-admin-commitment`

Force a block commitment (admin only)

**Usage:** `ffs-dev mcr protocol client eth post-admin-commitment [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --id <ID> --commitment <COMMITMENT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--height <HEIGHT>` — The height of the block to commit
* `--id <ID>` — The id of the block to commit
* `--commitment <COMMITMENT>` — The commitment to commit



## `ffs-dev mcr protocol client eth post-commitment-batch`

Post a batch of block commitments

**Usage:** `ffs-dev mcr protocol client eth post-commitment-batch [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --height <HEIGHT> --id <ID> --commitment <COMMITMENT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--height <HEIGHT>` — The height of the block to commit
* `--id <ID>` — The id of the block to commit
* `--commitment <COMMITMENT>` — The commitment to commit



## `ffs-dev mcr protocol client eth stream-commitments`

Stream block commitments

**Usage:** `ffs-dev mcr protocol client eth stream-commitments [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions



## `ffs-dev mcr protocol client eth get-commitment-at-height`

Get commitment at a specific height

**Usage:** `ffs-dev mcr protocol client eth get-commitment-at-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --height <HEIGHT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--height <HEIGHT>` — The height to get the commitment for



## `ffs-dev mcr protocol client eth get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `ffs-dev mcr protocol client eth get-posted-commitment-at-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --height <HEIGHT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--height <HEIGHT>` — The height to get the commitment for



## `ffs-dev mcr protocol client eth get-max-tolerable-block-height`

Get max tolerable block height

**Usage:** `ffs-dev mcr protocol client eth get-max-tolerable-block-height [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions



## `ffs-dev mcr protocol client eth stake`

Stake tokens for the MCR domain

**Usage:** `ffs-dev mcr protocol client eth stake [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --amount <AMOUNT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--amount <AMOUNT>` — Amount to stake



## `ffs-dev mcr protocol client eth unstake`

Unstake tokens from the MCR domain

**Usage:** `ffs-dev mcr protocol client eth unstake [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --amount <AMOUNT>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions
* `--amount <AMOUNT>` — Amount to unstake



## `ffs-dev mcr protocol client post-commitment`

Post a commitment to an MCR implementation

**Usage:** `ffs-dev mcr protocol client post-commitment [OPTIONS]`

###### **Options:**

* `--commitment-hex <COMMITMENT_HEX>` — Hex-encoded commitment
* `--preimage-string <PREIMAGE_STRING>` — String to be hashed into a commitment



## `ffs-dev pcp`

Manage PCP

**Usage:** `ffs-dev pcp <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: sdk/cli/ffs-dev/src/cli/pcp/mod.rs

###### **Subcommands:**

* `network` — ???
* `protocol` — ???



## `ffs-dev pcp network`

???

**Usage:** `ffs-dev pcp network <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/pcp/cli/network/src/cli/mod.rs

###### **Subcommands:**

* `run` — ???
* `client` — ???
* `coordinator` — ???



## `ffs-dev pcp network run`

???

**Usage:** `ffs-dev pcp network run`



## `ffs-dev pcp network client`

???

**Usage:** `ffs-dev pcp network client <COMMAND>`

###### **Subcommands:**

* `run` — ???



## `ffs-dev pcp network client run`

???

**Usage:** `ffs-dev pcp network client run`



## `ffs-dev pcp network coordinator`

???

**Usage:** `ffs-dev pcp network coordinator <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/pcp/cli/coordinator/src/cli/mod.rs

###### **Subcommands:**

* `run` — ???
* `eth` — ???



## `ffs-dev pcp network coordinator run`

???

**Usage:** `ffs-dev pcp network coordinator run`



## `ffs-dev pcp network coordinator eth`

???

**Usage:** `ffs-dev pcp network coordinator eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/pcp/cli/coordinator/src/cli/eth/mod.rs

###### **Subcommands:**

* `anvil` — ???
* `live` — ???



## `ffs-dev pcp network coordinator eth anvil`

???

**Usage:** `ffs-dev pcp network coordinator eth anvil <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/pcp/cli/coordinator/src/cli/eth/anvil/mod.rs

###### **Subcommands:**

* `up` — ???



## `ffs-dev pcp network coordinator eth anvil up`

???

**Usage:** `ffs-dev pcp network coordinator eth anvil up <COMMAND>`

###### **Subcommands:**

* `where` — Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev pcp network coordinator eth anvil up where`

Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev pcp network coordinator eth anvil up where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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
* `--initial-block-height <INITIAL_BLOCK_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-block-tolerance <LEADING_BLOCK_TOLERANCE>` — The leading block tolerance

  Default value: `10`
* `--epoch-duration <EPOCH_DURATION>` — The epoch duration

  Default value: `1000000`
* `--reward-contract <REWARD_CONTRACT>` — The reward contract
* `--existing-proxy-admin <EXISTING_PROXY_ADMIN>` — The existing proxy admin
* `--existing-token-proxy <EXISTING_TOKEN_PROXY>` — The existing move token proxy
* `--existing-staking-proxy <EXISTING_STAKING_PROXY>` — The existing staking proxy
* `--existing-pcp-proxy <EXISTING_PCP_PROXY>` — The existing PCP proxy
* `--existing-reward-proxy <EXISTING_REWARD_PROXY>` — The existing ARO proxy
* `--destroy-mode` — Whether to destroy the contracts

  Default value: `false`
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev pcp network coordinator eth anvil up using`

Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev pcp network coordinator eth anvil up using [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

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
* `--initial-block-height <INITIAL_BLOCK_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-block-tolerance <LEADING_BLOCK_TOLERANCE>` — The leading block tolerance

  Default value: `10`
* `--epoch-duration <EPOCH_DURATION>` — The epoch duration

  Default value: `1000000`
* `--reward-contract <REWARD_CONTRACT>` — The reward contract
* `--existing-proxy-admin <EXISTING_PROXY_ADMIN>` — The existing proxy admin
* `--existing-token-proxy <EXISTING_TOKEN_PROXY>` — The existing move token proxy
* `--existing-staking-proxy <EXISTING_STAKING_PROXY>` — The existing staking proxy
* `--existing-pcp-proxy <EXISTING_PCP_PROXY>` — The existing PCP proxy
* `--existing-reward-proxy <EXISTING_REWARD_PROXY>` — The existing ARO proxy
* `--destroy-mode` — Whether to destroy the contracts

  Default value: `false`
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev pcp network coordinator eth live`

???

**Usage:** `ffs-dev pcp network coordinator eth live <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/pcp/cli/coordinator/src/cli/eth/live/mod.rs

###### **Subcommands:**

* `up` — ???



## `ffs-dev pcp network coordinator eth live up`

???

**Usage:** `ffs-dev pcp network coordinator eth live up [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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
* `--initial-block-height <INITIAL_BLOCK_HEIGHT>` — The initial block height

  Default value: `1`
* `--leading-block-tolerance <LEADING_BLOCK_TOLERANCE>` — The leading block tolerance

  Default value: `10`
* `--epoch-duration <EPOCH_DURATION>` — The epoch duration

  Default value: `1000000`
* `--reward-contract <REWARD_CONTRACT>` — The reward contract
* `--existing-proxy-admin <EXISTING_PROXY_ADMIN>` — The existing proxy admin
* `--existing-token-proxy <EXISTING_TOKEN_PROXY>` — The existing move token proxy
* `--existing-staking-proxy <EXISTING_STAKING_PROXY>` — The existing staking proxy
* `--existing-pcp-proxy <EXISTING_PCP_PROXY>` — The existing PCP proxy
* `--existing-reward-proxy <EXISTING_REWARD_PROXY>` — The existing ARO proxy
* `--destroy-mode` — Whether to destroy the contracts

  Default value: `false`
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `ffs-dev pcp protocol`

???

**Usage:** `ffs-dev pcp protocol <COMMAND>`

###### **Subcommands:**

* `run` — 
* `client` — The subcommands of the `pcp-protocol-client` CLI



## `ffs-dev pcp protocol run`

**Usage:** `ffs-dev pcp protocol run`



## `ffs-dev pcp protocol client`

The subcommands of the `pcp-protocol-client` CLI

**Usage:** `ffs-dev pcp protocol client <COMMAND>`

###### **Subcommands:**

* `run` — 
* `eth` — 
* `post-commitment` — Post a commitment to an PCP implementation
* `deploy` — Deploy PCP contracts using deployer-core



## `ffs-dev pcp protocol client run`

**Usage:** `ffs-dev pcp protocol client run`



## `ffs-dev pcp protocol client eth`

**Usage:** `ffs-dev pcp protocol client eth <COMMAND>`

###### **Subcommands:**

* `post-admin-commitment` — 



## `ffs-dev pcp protocol client eth post-admin-commitment`

**Usage:** `ffs-dev pcp protocol client eth post-admin-commitment [OPTIONS] --pcp-contract-address <PCP_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES>`

###### **Options:**

* `--pcp-contract-address <PCP_CONTRACT_ADDRESS>` — The address of the PCP settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions



## `ffs-dev pcp protocol client post-commitment`

Post a commitment to an PCP implementation

**Usage:** `ffs-dev pcp protocol client post-commitment [OPTIONS]`

###### **Options:**

* `--commitment-hex <COMMITMENT_HEX>` — Hex-encoded commitment
* `--preimage-string <PREIMAGE_STRING>` — String to be hashed into a commitment



## `ffs-dev pcp protocol client deploy`

Deploy PCP contracts using deployer-core

**Usage:** `ffs-dev pcp protocol client deploy <COMMAND>`

###### **Subcommands:**

* `anvil` — Deploy to local Anvil network



## `ffs-dev pcp protocol client deploy anvil`

Deploy to local Anvil network

**Usage:** `ffs-dev pcp protocol client deploy anvil [OPTIONS] --admin <ADMIN> --private-key <PRIVATE_KEY>`

###### **Options:**

* `--admin <ADMIN>` — Admin address for deployed contracts
* `--rpc-url <RPC_URL>` — RPC URL (defaults to http://localhost:8545)

  Default value: `http://localhost:8545`
* `--private-key <PRIVATE_KEY>` — Private key for deployment



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

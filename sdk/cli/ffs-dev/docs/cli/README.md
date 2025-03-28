# Command-Line Help for `ffs-dev`

This document contains the help content for the `ffs-dev` command-line program.

**Command Overview:**

* [`ffs-dev`↴](#ffs-dev)
* [`ffs-dev markdown`↴](#ffs-dev-markdown)
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
* [`ffs-dev mcr protocol client post-commitment`↴](#ffs-dev-mcr-protocol-client-post-commitment)

## `ffs-dev`

**Usage:** `ffs-dev <COMMAND>`

###### **Subcommands:**

* `markdown` — 
* `mcr` — 



## `ffs-dev markdown`

**Usage:** `ffs-dev markdown <COMMAND>`

###### **Subcommands:**

* `file` — Write the CLI documentation to a file
* `print` — Print the CLI documentation to the console
* `workspace` — Write the CLI documentation to a file in the workspace



## `ffs-dev markdown file`

Write the CLI documentation to a file

**Usage:** `ffs-dev markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev markdown print`

Print the CLI documentation to the console

**Usage:** `ffs-dev markdown print`



## `ffs-dev markdown workspace`

Write the CLI documentation to a file in the workspace

**Usage:** `ffs-dev markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr`

**Usage:** `ffs-dev mcr <COMMAND>`

###### **Subcommands:**

* `network` — The subcommands of the `mcr-network` CLI
* `protocol` — The subcommands of the `mcr-protocol` CLI



## `ffs-dev mcr network`

The subcommands of the `mcr-network` CLI

**Usage:** `ffs-dev mcr network <COMMAND>`

###### **Subcommands:**

* `run` — 
* `client` — The subcommands of the `mcr-network-client` CLI
* `coordinator` — The subcommands of the `mcr-network-coordinator` CLI



## `ffs-dev mcr network run`

**Usage:** `ffs-dev mcr network run`



## `ffs-dev mcr network client`

The subcommands of the `mcr-network-client` CLI

**Usage:** `ffs-dev mcr network client <COMMAND>`

###### **Subcommands:**

* `run` — 



## `ffs-dev mcr network client run`

**Usage:** `ffs-dev mcr network client run`



## `ffs-dev mcr network coordinator`

The subcommands of the `mcr-network-coordinator` CLI

**Usage:** `ffs-dev mcr network coordinator <COMMAND>`

###### **Subcommands:**

* `run` — 
* `eth` — 



## `ffs-dev mcr network coordinator run`

**Usage:** `ffs-dev mcr network coordinator run`



## `ffs-dev mcr network coordinator eth`

**Usage:** `ffs-dev mcr network coordinator eth <COMMAND>`

###### **Subcommands:**

* `anvil` — 
* `live` — 



## `ffs-dev mcr network coordinator eth anvil`

**Usage:** `ffs-dev mcr network coordinator eth anvil <COMMAND>`

###### **Subcommands:**

* `up` — 



## `ffs-dev mcr network coordinator eth anvil up`

**Usage:** `ffs-dev mcr network coordinator eth anvil up <COMMAND>`

###### **Subcommands:**

* `where` — Run up with all parameters passed explicitly as CLI flags
* `using` — Run up with parameters from environment variables, config files, and CLI flags



## `ffs-dev mcr network coordinator eth anvil up where`

Run up with all parameters passed explicitly as CLI flags

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
* `--move-token-proxy <MOVE_TOKEN_PROXY>` — The existing move token proxy
* `--staking-proxy <STAKING_PROXY>` — The existing staking proxy
* `--mcr-proxy <MCR_PROXY>` — The existing MCR proxy
* `--reward-proxy <REWARD_PROXY>` — The existing ARO proxy
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev mcr network coordinator eth anvil up using`

Run up with parameters from environment variables, config files, and CLI flags

**Usage:** `ffs-dev mcr network coordinator eth anvil up using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>`

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `ffs-dev mcr network coordinator eth live`

**Usage:** `ffs-dev mcr network coordinator eth live <COMMAND>`

###### **Subcommands:**

* `up` — The arguments to be passed to the Forge Apply script



## `ffs-dev mcr network coordinator eth live up`

The arguments to be passed to the Forge Apply script

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
* `--move-token-proxy <MOVE_TOKEN_PROXY>` — The existing move token proxy
* `--staking-proxy <STAKING_PROXY>` — The existing staking proxy
* `--mcr-proxy <MCR_PROXY>` — The existing MCR proxy
* `--reward-proxy <REWARD_PROXY>` — The existing ARO proxy
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `ffs-dev mcr protocol`

The subcommands of the `mcr-protocol` CLI

**Usage:** `ffs-dev mcr protocol <COMMAND>`

###### **Subcommands:**

* `run` — 
* `client` — The subcommands of the `mcr-protocol-client` CLI



## `ffs-dev mcr protocol run`

**Usage:** `ffs-dev mcr protocol run`



## `ffs-dev mcr protocol client`

The subcommands of the `mcr-protocol-client` CLI

**Usage:** `ffs-dev mcr protocol client <COMMAND>`

###### **Subcommands:**

* `run` — 
* `eth` — 
* `post-commitment` — Post a commitment to an MCR implementation



## `ffs-dev mcr protocol client run`

**Usage:** `ffs-dev mcr protocol client run`



## `ffs-dev mcr protocol client eth`

**Usage:** `ffs-dev mcr protocol client eth <COMMAND>`

###### **Subcommands:**

* `post-admin-commitment` — 



## `ffs-dev mcr protocol client eth post-admin-commitment`

**Usage:** `ffs-dev mcr protocol client eth post-admin-commitment [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --signer-identifier <SIGNER_IDENTIFIER> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES>`

###### **Options:**

* `--mcr-contract-address <MCR_CONTRACT_ADDRESS>` — The address of the MCR settlement contract
* `--rpc-url <RPC_URL>` — The Ethereum RPC connection URL
* `--ws-url <WS_URL>` — The Ethereum WebSocket connection URL
* `--chain-id <CHAIN_ID>` — The Ethereum chain ID
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--run-commitment-admin-mode` — Whether to run in settlement admin mode
* `--gas-limit <GAS_LIMIT>` — The gas limit for transactions
* `--transaction-send-retries <TRANSACTION_SEND_RETRIES>` — The number of retries for sending transactions



## `ffs-dev mcr protocol client post-commitment`

Post a commitment to an MCR implementation

**Usage:** `ffs-dev mcr protocol client post-commitment [OPTIONS]`

###### **Options:**

* `--commitment-hex <COMMITMENT_HEX>` — Hex-encoded commitment
* `--preimage-string <PREIMAGE_STRING>` — String to be hashed into a commitment



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

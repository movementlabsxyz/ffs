# Command-Line Help for `mcr-network-coordinator`

This document contains the help content for the `mcr-network-coordinator` command-line program.

**Command Overview:**

* [`mcr-network-coordinator`↴](#mcr-network-coordinator)
* [`mcr-network-coordinator markdown`↴](#mcr-network-coordinator-markdown)
* [`mcr-network-coordinator markdown generate`↴](#mcr-network-coordinator-markdown-generate)
* [`mcr-network-coordinator markdown file`↴](#mcr-network-coordinator-markdown-file)
* [`mcr-network-coordinator markdown print`↴](#mcr-network-coordinator-markdown-print)
* [`mcr-network-coordinator markdown workspace`↴](#mcr-network-coordinator-markdown-workspace)
* [`mcr-network-coordinator eth`↴](#mcr-network-coordinator-eth)
* [`mcr-network-coordinator eth anvil`↴](#mcr-network-coordinator-eth-anvil)
* [`mcr-network-coordinator eth anvil up`↴](#mcr-network-coordinator-eth-anvil-up)
* [`mcr-network-coordinator eth anvil up where`↴](#mcr-network-coordinator-eth-anvil-up-where)
* [`mcr-network-coordinator eth anvil up using`↴](#mcr-network-coordinator-eth-anvil-up-using)
* [`mcr-network-coordinator eth live`↴](#mcr-network-coordinator-eth-live)
* [`mcr-network-coordinator eth live up`↴](#mcr-network-coordinator-eth-live-up)

## `mcr-network-coordinator`

The `mcr-network-coordinator` CLI

**Usage:** `mcr-network-coordinator [COMMAND]`

###### **Subcommands:**

* `markdown` — Generate markdown for the CLI
* `eth` — Ethereum-specific commands of the network coordinator, i.e., for bringing-up an Ethereum-based MCR network



## `mcr-network-coordinator markdown`

Generate markdown for the CLI

**Usage:** `mcr-network-coordinator markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `mcr-network-coordinator markdown generate`

Generate and update the documentation

**Usage:** `mcr-network-coordinator markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `mcr-network-coordinator markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `mcr-network-coordinator markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-network-coordinator markdown print`

Print the documentation in the shell

**Usage:** `mcr-network-coordinator markdown print`



## `mcr-network-coordinator markdown workspace`

Generate the documentation for the workspace

**Usage:** `mcr-network-coordinator markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-network-coordinator eth`

Ethereum-specific commands of the network coordinator, i.e., for bringing-up an Ethereum-based MCR network

**Usage:** `mcr-network-coordinator eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/mod.rs

###### **Subcommands:**

* `anvil` — Anvil-specific commands of the network coordinator, i.e., for bringing-up an MCR network on Anvil
* `live` — Live-Ethereum-based commands of the network coordinator, i.e., for bringing-up an MCR network on a live Ethereum network



## `mcr-network-coordinator eth anvil`

Anvil-specific commands of the network coordinator, i.e., for bringing-up an MCR network on Anvil

**Usage:** `mcr-network-coordinator eth anvil <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/anvil/mod.rs

###### **Subcommands:**

* `up` — Brings-up an MCR network on Anvil



## `mcr-network-coordinator eth anvil up`

Brings-up an MCR network on Anvil

**Usage:** `mcr-network-coordinator eth anvil up <COMMAND>`

###### **Subcommands:**

* `where` — Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `mcr-network-coordinator eth anvil up where`

Run up with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-network-coordinator eth anvil up where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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



## `mcr-network-coordinator eth anvil up using`

Run up with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `mcr-network-coordinator eth anvil up using [OPTIONS] [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--write-artifacts-path <WRITE_ARTIFACTS_PATH>` — Path to the configuration file
* `--write-anvil-data-path <WRITE_ANVIL_DATA_PATH>` — Path to write the anvil data as json



## `mcr-network-coordinator eth live`

Live-Ethereum-based commands of the network coordinator, i.e., for bringing-up an MCR network on a live Ethereum network

**Usage:** `mcr-network-coordinator eth live <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/live/mod.rs

###### **Subcommands:**

* `up` — Brings-up an MCR network on a live Ethereum network



## `mcr-network-coordinator eth live up`

Brings-up an MCR network on a live Ethereum network

**Usage:** `mcr-network-coordinator eth live up [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

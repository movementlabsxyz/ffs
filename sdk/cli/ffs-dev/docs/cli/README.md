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
* [`ffs-dev mcr network markdown`↴](#ffs-dev-mcr-network-markdown)
* [`ffs-dev mcr network markdown generate`↴](#ffs-dev-mcr-network-markdown-generate)
* [`ffs-dev mcr network markdown file`↴](#ffs-dev-mcr-network-markdown-file)
* [`ffs-dev mcr network markdown print`↴](#ffs-dev-mcr-network-markdown-print)
* [`ffs-dev mcr network markdown workspace`↴](#ffs-dev-mcr-network-markdown-workspace)
* [`ffs-dev mcr network client`↴](#ffs-dev-mcr-network-client)
* [`ffs-dev mcr network client markdown`↴](#ffs-dev-mcr-network-client-markdown)
* [`ffs-dev mcr network client markdown generate`↴](#ffs-dev-mcr-network-client-markdown-generate)
* [`ffs-dev mcr network client markdown file`↴](#ffs-dev-mcr-network-client-markdown-file)
* [`ffs-dev mcr network client markdown print`↴](#ffs-dev-mcr-network-client-markdown-print)
* [`ffs-dev mcr network client markdown workspace`↴](#ffs-dev-mcr-network-client-markdown-workspace)
* [`ffs-dev mcr network coordinator`↴](#ffs-dev-mcr-network-coordinator)
* [`ffs-dev mcr network coordinator markdown`↴](#ffs-dev-mcr-network-coordinator-markdown)
* [`ffs-dev mcr network coordinator markdown generate`↴](#ffs-dev-mcr-network-coordinator-markdown-generate)
* [`ffs-dev mcr network coordinator markdown file`↴](#ffs-dev-mcr-network-coordinator-markdown-file)
* [`ffs-dev mcr network coordinator markdown print`↴](#ffs-dev-mcr-network-coordinator-markdown-print)
* [`ffs-dev mcr network coordinator markdown workspace`↴](#ffs-dev-mcr-network-coordinator-markdown-workspace)
* [`ffs-dev mcr network coordinator eth`↴](#ffs-dev-mcr-network-coordinator-eth)
* [`ffs-dev mcr network coordinator eth anvil`↴](#ffs-dev-mcr-network-coordinator-eth-anvil)
* [`ffs-dev mcr network coordinator eth anvil up`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up)
* [`ffs-dev mcr network coordinator eth anvil up where`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up-where)
* [`ffs-dev mcr network coordinator eth anvil up using`↴](#ffs-dev-mcr-network-coordinator-eth-anvil-up-using)
* [`ffs-dev mcr network coordinator eth live`↴](#ffs-dev-mcr-network-coordinator-eth-live)
* [`ffs-dev mcr network coordinator eth live up`↴](#ffs-dev-mcr-network-coordinator-eth-live-up)
* [`ffs-dev mcr protocol`↴](#ffs-dev-mcr-protocol)
* [`ffs-dev mcr protocol markdown`↴](#ffs-dev-mcr-protocol-markdown)
* [`ffs-dev mcr protocol markdown generate`↴](#ffs-dev-mcr-protocol-markdown-generate)
* [`ffs-dev mcr protocol markdown file`↴](#ffs-dev-mcr-protocol-markdown-file)
* [`ffs-dev mcr protocol markdown print`↴](#ffs-dev-mcr-protocol-markdown-print)
* [`ffs-dev mcr protocol markdown workspace`↴](#ffs-dev-mcr-protocol-markdown-workspace)
* [`ffs-dev mcr protocol client`↴](#ffs-dev-mcr-protocol-client)
* [`ffs-dev mcr protocol client markdown`↴](#ffs-dev-mcr-protocol-client-markdown)
* [`ffs-dev mcr protocol client markdown generate`↴](#ffs-dev-mcr-protocol-client-markdown-generate)
* [`ffs-dev mcr protocol client markdown file`↴](#ffs-dev-mcr-protocol-client-markdown-file)
* [`ffs-dev mcr protocol client markdown print`↴](#ffs-dev-mcr-protocol-client-markdown-print)
* [`ffs-dev mcr protocol client markdown workspace`↴](#ffs-dev-mcr-protocol-client-markdown-workspace)
* [`ffs-dev mcr protocol client eth`↴](#ffs-dev-mcr-protocol-client-eth)
* [`ffs-dev mcr protocol client eth post-admin-commitment`↴](#ffs-dev-mcr-protocol-client-eth-post-admin-commitment)
* [`ffs-dev mcr protocol client eth post-admin-commitment where`↴](#ffs-dev-mcr-protocol-client-eth-post-admin-commitment-where)
* [`ffs-dev mcr protocol client eth post-admin-commitment using`↴](#ffs-dev-mcr-protocol-client-eth-post-admin-commitment-using)
* [`ffs-dev mcr protocol client eth post-commitment`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment)
* [`ffs-dev mcr protocol client eth post-commitment where`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-where)
* [`ffs-dev mcr protocol client eth post-commitment using`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-using)
* [`ffs-dev mcr protocol client eth post-commitment-batch`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-batch)
* [`ffs-dev mcr protocol client eth post-commitment-batch where`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-batch-where)
* [`ffs-dev mcr protocol client eth post-commitment-batch using`↴](#ffs-dev-mcr-protocol-client-eth-post-commitment-batch-using)
* [`ffs-dev mcr protocol client eth stream-commitments`↴](#ffs-dev-mcr-protocol-client-eth-stream-commitments)
* [`ffs-dev mcr protocol client eth stream-commitments where`↴](#ffs-dev-mcr-protocol-client-eth-stream-commitments-where)
* [`ffs-dev mcr protocol client eth stream-commitments using`↴](#ffs-dev-mcr-protocol-client-eth-stream-commitments-using)
* [`ffs-dev mcr protocol client eth get-commitment`↴](#ffs-dev-mcr-protocol-client-eth-get-commitment)
* [`ffs-dev mcr protocol client eth get-commitment where`↴](#ffs-dev-mcr-protocol-client-eth-get-commitment-where)
* [`ffs-dev mcr protocol client eth get-commitment using`↴](#ffs-dev-mcr-protocol-client-eth-get-commitment-using)
* [`ffs-dev mcr protocol client eth get-accepted-commitment-at-height`↴](#ffs-dev-mcr-protocol-client-eth-get-accepted-commitment-at-height)
* [`ffs-dev mcr protocol client eth get-accepted-commitment-at-height where`↴](#ffs-dev-mcr-protocol-client-eth-get-accepted-commitment-at-height-where)
* [`ffs-dev mcr protocol client eth get-accepted-commitment-at-height using`↴](#ffs-dev-mcr-protocol-client-eth-get-accepted-commitment-at-height-using)
* [`ffs-dev mcr protocol client eth get-posted-commitment-at-height`↴](#ffs-dev-mcr-protocol-client-eth-get-posted-commitment-at-height)
* [`ffs-dev mcr protocol client eth get-posted-commitment-at-height where`↴](#ffs-dev-mcr-protocol-client-eth-get-posted-commitment-at-height-where)
* [`ffs-dev mcr protocol client eth get-posted-commitment-at-height using`↴](#ffs-dev-mcr-protocol-client-eth-get-posted-commitment-at-height-using)
* [`ffs-dev mcr protocol client eth get-max-tolerable-commitment-height`↴](#ffs-dev-mcr-protocol-client-eth-get-max-tolerable-commitment-height)
* [`ffs-dev mcr protocol client eth get-max-tolerable-commitment-height where`↴](#ffs-dev-mcr-protocol-client-eth-get-max-tolerable-commitment-height-where)
* [`ffs-dev mcr protocol client eth get-max-tolerable-commitment-height using`↴](#ffs-dev-mcr-protocol-client-eth-get-max-tolerable-commitment-height-using)
* [`ffs-dev mcr protocol client eth stake`↴](#ffs-dev-mcr-protocol-client-eth-stake)
* [`ffs-dev mcr protocol client eth stake where`↴](#ffs-dev-mcr-protocol-client-eth-stake-where)
* [`ffs-dev mcr protocol client eth stake using`↴](#ffs-dev-mcr-protocol-client-eth-stake-using)
* [`ffs-dev mcr protocol client eth get-stake`↴](#ffs-dev-mcr-protocol-client-eth-get-stake)
* [`ffs-dev mcr protocol client eth get-stake where`↴](#ffs-dev-mcr-protocol-client-eth-get-stake-where)
* [`ffs-dev mcr protocol client eth get-stake using`↴](#ffs-dev-mcr-protocol-client-eth-get-stake-using)
* [`ffs-dev mcr protocol client eth unstake`↴](#ffs-dev-mcr-protocol-client-eth-unstake)
* [`ffs-dev mcr protocol client eth unstake where`↴](#ffs-dev-mcr-protocol-client-eth-unstake-where)
* [`ffs-dev mcr protocol client eth unstake using`↴](#ffs-dev-mcr-protocol-client-eth-unstake-using)
* [`ffs-dev mcr protocol client eth grant-trusted-attester`↴](#ffs-dev-mcr-protocol-client-eth-grant-trusted-attester)
* [`ffs-dev mcr protocol client eth grant-trusted-attester where`↴](#ffs-dev-mcr-protocol-client-eth-grant-trusted-attester-where)
* [`ffs-dev mcr protocol client eth grant-trusted-attester using`↴](#ffs-dev-mcr-protocol-client-eth-grant-trusted-attester-using)
* [`ffs-dev mcr protocol deployer`↴](#ffs-dev-mcr-protocol-deployer)
* [`ffs-dev mcr protocol deployer markdown`↴](#ffs-dev-mcr-protocol-deployer-markdown)
* [`ffs-dev mcr protocol deployer markdown generate`↴](#ffs-dev-mcr-protocol-deployer-markdown-generate)
* [`ffs-dev mcr protocol deployer markdown file`↴](#ffs-dev-mcr-protocol-deployer-markdown-file)
* [`ffs-dev mcr protocol deployer markdown print`↴](#ffs-dev-mcr-protocol-deployer-markdown-print)
* [`ffs-dev mcr protocol deployer markdown workspace`↴](#ffs-dev-mcr-protocol-deployer-markdown-workspace)
* [`ffs-dev mcr protocol deployer eth`↴](#ffs-dev-mcr-protocol-deployer-eth)
* [`ffs-dev mcr protocol deployer eth apply`↴](#ffs-dev-mcr-protocol-deployer-eth-apply)
* [`ffs-dev mcr protocol deployer eth apply where`↴](#ffs-dev-mcr-protocol-deployer-eth-apply-where)
* [`ffs-dev mcr protocol deployer eth apply using`↴](#ffs-dev-mcr-protocol-deployer-eth-apply-using)
* [`ffs-dev mcr protocol deployer eth destroy`↴](#ffs-dev-mcr-protocol-deployer-eth-destroy)

## `ffs-dev`

**Usage:** `ffs-dev <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: sdk/cli/ffs-dev/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generate CLI documentation
* `mcr` — Manage MCR



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

* `network` — Subcommands for bringing-up an MCR network
* `protocol` — Subcommands for the MCR protocol



## `ffs-dev mcr network`

Subcommands for bringing-up an MCR network

**Usage:** `ffs-dev mcr network <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/network/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `client` — The client-specific commands of the MCR network
* `coordinator` — The coordinator-specific commands of the MCR network



## `ffs-dev mcr network markdown`

Generates markdown for the CLI

**Usage:** `ffs-dev mcr network markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr network markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr network markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr network markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr network markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr network markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr network markdown print`



## `ffs-dev mcr network markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr network markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr network client`

The client-specific commands of the MCR network

**Usage:** `ffs-dev mcr network client <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI



## `ffs-dev mcr network client markdown`

Generates markdown for the CLI

**Usage:** `ffs-dev mcr network client markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr network client markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr network client markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr network client markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr network client markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr network client markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr network client markdown print`



## `ffs-dev mcr network client markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr network client markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr network coordinator`

The coordinator-specific commands of the MCR network

**Usage:** `ffs-dev mcr network coordinator <COMMAND>`

###### **Subcommands:**

* `markdown` — Generate markdown for the CLI
* `eth` — Ethereum-specific commands of the network coordinator, i.e., for bringing-up an Ethereum-based MCR network



## `ffs-dev mcr network coordinator markdown`

Generate markdown for the CLI

**Usage:** `ffs-dev mcr network coordinator markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr network coordinator markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr network coordinator markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr network coordinator markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr network coordinator markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr network coordinator markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr network coordinator markdown print`



## `ffs-dev mcr network coordinator markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr network coordinator markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr network coordinator eth`

Ethereum-specific commands of the network coordinator, i.e., for bringing-up an Ethereum-based MCR network

**Usage:** `ffs-dev mcr network coordinator eth <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/mod.rs

###### **Subcommands:**

* `anvil` — Anvil-specific commands of the network coordinator, i.e., for bringing-up an MCR network on Anvil
* `live` — Live-Ethereum-based commands of the network coordinator, i.e., for bringing-up an MCR network on a live Ethereum network



## `ffs-dev mcr network coordinator eth anvil`

Anvil-specific commands of the network coordinator, i.e., for bringing-up an MCR network on Anvil

**Usage:** `ffs-dev mcr network coordinator eth anvil <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/anvil/mod.rs

###### **Subcommands:**

* `up` — Brings-up an MCR network on Anvil



## `ffs-dev mcr network coordinator eth anvil up`

Brings-up an MCR network on Anvil

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

Live-Ethereum-based commands of the network coordinator, i.e., for bringing-up an MCR network on a live Ethereum network

**Usage:** `ffs-dev mcr network coordinator eth live <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: network/mcr/cli/coordinator/src/cli/eth/live/mod.rs

###### **Subcommands:**

* `up` — Brings-up an MCR network on a live Ethereum network



## `ffs-dev mcr network coordinator eth live up`

Brings-up an MCR network on a live Ethereum network

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



## `ffs-dev mcr protocol`

Subcommands for the MCR protocol

**Usage:** `ffs-dev mcr protocol <COMMAND>`

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `client` — The client-specific commands of the MCR protocol
* `deployer` — The deployer-specific commands of the MCR protocol



## `ffs-dev mcr protocol markdown`

Generates markdown for the CLI

**Usage:** `ffs-dev mcr protocol markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr protocol markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr protocol markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr protocol markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr protocol markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr protocol markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr protocol markdown print`



## `ffs-dev mcr protocol markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr protocol markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr protocol client`

The client-specific commands of the MCR protocol

**Usage:** `ffs-dev mcr protocol client <COMMAND>`

KEEP THIS UNTIL PRODUCTION-READY : Defined in: protocol/mcr/cli/client/src/cli/mod.rs

###### **Subcommands:**

* `markdown` — Generates markdown for the CLI
* `eth` — Ethereum-specific commands of the protocol, such as staking and committing



## `ffs-dev mcr protocol client markdown`

Generates markdown for the CLI

**Usage:** `ffs-dev mcr protocol client markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr protocol client markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr protocol client markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr protocol client markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr protocol client markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr protocol client markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr protocol client markdown print`



## `ffs-dev mcr protocol client markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr protocol client markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr protocol client eth`

Ethereum-specific commands of the protocol, such as staking and committing

**Usage:** `ffs-dev mcr protocol client eth <COMMAND>`

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



## `ffs-dev mcr protocol client eth post-admin-commitment`

Force a commitment (admin only)

**Usage:** `ffs-dev mcr protocol client eth post-admin-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth post-admin-commitment where`

Run postadmincommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-admin-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `ffs-dev mcr protocol client eth post-admin-commitment using`

Run postadmincommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-admin-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth post-commitment`

Post a single commitment

**Usage:** `ffs-dev mcr protocol client eth post-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth post-commitment where`

Run postcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `ffs-dev mcr protocol client eth post-commitment using`

Run postcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-commitment using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth post-commitment-batch`

Post a batch of commitments

**Usage:** `ffs-dev mcr protocol client eth post-commitment-batch <COMMAND>`

###### **Subcommands:**

* `where` — Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth post-commitment-batch where`

Run postcommitmentbatch with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-commitment-batch where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE>`

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



## `ffs-dev mcr protocol client eth post-commitment-batch using`

Run postcommitmentbatch with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth post-commitment-batch using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --id <ID> --vote <VOTE> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth stream-commitments`

Stream commitments

**Usage:** `ffs-dev mcr protocol client eth stream-commitments <COMMAND>`

###### **Subcommands:**

* `where` — Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth stream-commitments where`

Run streamcommitments with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth stream-commitments where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `ffs-dev mcr protocol client eth stream-commitments using`

Run streamcommitments with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth stream-commitments using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth get-commitment`

Get a commitment for a given height and attester

**Usage:** `ffs-dev mcr protocol client eth get-commitment <COMMAND>`

###### **Subcommands:**

* `where` — Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth get-commitment where`

Run getcommitment with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-commitment where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT> --attester <ATTESTER>`

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



## `ffs-dev mcr protocol client eth get-commitment using`

Run getcommitment with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-commitment using [OPTIONS] --height <HEIGHT> --attester <ATTESTER> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — Block height to check commitment for
* `--attester <ATTESTER>` — Attester address to check commitment for



## `ffs-dev mcr protocol client eth get-accepted-commitment-at-height`

Get accepted commitment at a specific height

**Usage:** `ffs-dev mcr protocol client eth get-accepted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth get-accepted-commitment-at-height where`

Run getacceptedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-accepted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --height <HEIGHT>`

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



## `ffs-dev mcr protocol client eth get-accepted-commitment-at-height using`

Run getacceptedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-accepted-commitment-at-height using [OPTIONS] --height <HEIGHT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--view-config-path <VIEW_CONFIG_PATH>`
* `--height <HEIGHT>` — The height to get the commitment for



## `ffs-dev mcr protocol client eth get-posted-commitment-at-height`

Get posted commitment at a specific height

**Usage:** `ffs-dev mcr protocol client eth get-posted-commitment-at-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth get-posted-commitment-at-height where`

Run getpostedcommitmentatheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-posted-commitment-at-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --height <HEIGHT>`

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



## `ffs-dev mcr protocol client eth get-posted-commitment-at-height using`

Run getpostedcommitmentatheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-posted-commitment-at-height using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --height <HEIGHT> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height`

Get max tolerable commitment height

**Usage:** `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height <COMMAND>`

###### **Subcommands:**

* `where` — Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height where`

Run getmaxtolerablecommitmentheight with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS>`

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



## `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height using`

Run getmaxtolerablecommitmentheight with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-max-tolerable-commitment-height using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth stake`

Stake tokens for the MCR domain

**Usage:** `ffs-dev mcr protocol client eth stake <COMMAND>`

###### **Subcommands:**

* `where` — Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth stake where`

Run stake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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



## `ffs-dev mcr protocol client eth stake using`

Run stake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth stake using [OPTIONS] --amount <AMOUNT> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--config-path <CONFIG_PATH>`
* `--amount <AMOUNT>` — Amount to stake



## `ffs-dev mcr protocol client eth get-stake`

Get the current epoch stake for an attester

**Usage:** `ffs-dev mcr protocol client eth get-stake <COMMAND>`

###### **Subcommands:**

* `where` — Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth get-stake where`

Run getstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-stake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --custodian <CUSTODIAN> --attester <ATTESTER>`

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



## `ffs-dev mcr protocol client eth get-stake using`

Run getstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth get-stake using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --custodian <CUSTODIAN> --attester <ATTESTER> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth unstake`

Unstake tokens from the MCR domain

**Usage:** `ffs-dev mcr protocol client eth unstake <COMMAND>`

###### **Subcommands:**

* `where` — Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth unstake where`

Run unstake with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth unstake where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT>`

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



## `ffs-dev mcr protocol client eth unstake using`

Run unstake with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth unstake using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --amount <AMOUNT> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol client eth grant-trusted-attester`

Grant TRUSTED_ATTESTER role to an attester

**Usage:** `ffs-dev mcr protocol client eth grant-trusted-attester <COMMAND>`

###### **Subcommands:**

* `where` — Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol client eth grant-trusted-attester where`

Run granttrustedattester with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth grant-trusted-attester where [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --attester <ATTESTER>`

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



## `ffs-dev mcr protocol client eth grant-trusted-attester using`

Run granttrustedattester with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol client eth grant-trusted-attester using [OPTIONS] --mcr-contract-address <MCR_CONTRACT_ADDRESS> --rpc-url <RPC_URL> --ws-url <WS_URL> --chain-id <CHAIN_ID> --gas-limit <GAS_LIMIT> --transaction-send-retries <TRANSACTION_SEND_RETRIES> --mcr-address <MCR_ADDRESS> --move-token-address <MOVE_TOKEN_ADDRESS> --staking-address <STAKING_ADDRESS> --signer-identifier <SIGNER_IDENTIFIER> --attester <ATTESTER> [EXTRA_ARGS]...`

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



## `ffs-dev mcr protocol deployer`

The deployer-specific commands of the MCR protocol

**Usage:** `ffs-dev mcr protocol deployer <COMMAND>`

###### **Subcommands:**

* `markdown` — 
* `eth` — 



## `ffs-dev mcr protocol deployer markdown`

**Usage:** `ffs-dev mcr protocol deployer markdown <COMMAND>`

###### **Subcommands:**

* `generate` — Generate and update the documentation
* `file` — Print the documentation to a file (providing the file path)
* `print` — Print the documentation in the shell
* `workspace` — Generate the documentation for the workspace



## `ffs-dev mcr protocol deployer markdown generate`

Generate and update the documentation

**Usage:** `ffs-dev mcr protocol deployer markdown generate [OPTIONS]`

###### **Options:**

* `--file <FILE>` — Override the default docs location



## `ffs-dev mcr protocol deployer markdown file`

Print the documentation to a file (providing the file path)

**Usage:** `ffs-dev mcr protocol deployer markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `ffs-dev mcr protocol deployer markdown print`

Print the documentation in the shell

**Usage:** `ffs-dev mcr protocol deployer markdown print`



## `ffs-dev mcr protocol deployer markdown workspace`

Generate the documentation for the workspace

**Usage:** `ffs-dev mcr protocol deployer markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `ffs-dev mcr protocol deployer eth`

**Usage:** `ffs-dev mcr protocol deployer eth <COMMAND>`

###### **Subcommands:**

* `apply` — 
* `destroy` — 



## `ffs-dev mcr protocol deployer eth apply`

**Usage:** `ffs-dev mcr protocol deployer eth apply <COMMAND>`

###### **Subcommands:**

* `where` — Run config with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>
* `using` — Run config with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>



## `ffs-dev mcr protocol deployer eth apply where`

Run config with all parameters passed explicitly as CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol deployer eth apply where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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



## `ffs-dev mcr protocol deployer eth apply using`

Run config with parameters from environment variables, config files, and CLI flags. See Orfile documentation for more details: <https://github.com/movementlabsxyz/orfile>

**Usage:** `ffs-dev mcr protocol deployer eth apply using [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>` — Extra arguments to be passed to the CLI

###### **Options:**

* `--script-args-path <SCRIPT_ARGS_PATH>`
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `ffs-dev mcr protocol deployer eth destroy`

**Usage:** `ffs-dev mcr protocol deployer eth destroy [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --proxy-admin <PROXY_ADMIN> --token-proxy <TOKEN_PROXY> --staking-proxy <STAKING_PROXY> --mcr-proxy <MCR_PROXY> --reward-proxy <REWARD_PROXY>`

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

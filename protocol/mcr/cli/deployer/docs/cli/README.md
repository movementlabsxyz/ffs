# Command-Line Help for `mcr-protocol-deployer`

This document contains the help content for the `mcr-protocol-deployer` command-line program.

**Command Overview:**

* [`mcr-protocol-deployer`↴](#mcr-protocol-deployer)
* [`mcr-protocol-deployer markdown`↴](#mcr-protocol-deployer-markdown)
* [`mcr-protocol-deployer markdown file`↴](#mcr-protocol-deployer-markdown-file)
* [`mcr-protocol-deployer markdown print`↴](#mcr-protocol-deployer-markdown-print)
* [`mcr-protocol-deployer markdown workspace`↴](#mcr-protocol-deployer-markdown-workspace)
* [`mcr-protocol-deployer eth`↴](#mcr-protocol-deployer-eth)
* [`mcr-protocol-deployer eth apply`↴](#mcr-protocol-deployer-eth-apply)
* [`mcr-protocol-deployer eth apply where`↴](#mcr-protocol-deployer-eth-apply-where)
* [`mcr-protocol-deployer eth apply using`↴](#mcr-protocol-deployer-eth-apply-using)
* [`mcr-protocol-deployer eth destroy`↴](#mcr-protocol-deployer-eth-destroy)

## `mcr-protocol-deployer`

The `mcr-protocol-client` CLI

**Usage:** `mcr-protocol-deployer [COMMAND]`

###### **Subcommands:**

* `markdown` — 
* `eth` — 



## `mcr-protocol-deployer markdown`

**Usage:** `mcr-protocol-deployer markdown <COMMAND>`

###### **Subcommands:**

* `file` — Write the CLI documentation to a file
* `print` — Print the CLI documentation to the console
* `workspace` — Write the CLI documentation to a file in the workspace



## `mcr-protocol-deployer markdown file`

Write the CLI documentation to a file

**Usage:** `mcr-protocol-deployer markdown file --file <FILE>`

###### **Options:**

* `--file <FILE>` — the file to write out to



## `mcr-protocol-deployer markdown print`

Print the CLI documentation to the console

**Usage:** `mcr-protocol-deployer markdown print`



## `mcr-protocol-deployer markdown workspace`

Write the CLI documentation to a file in the workspace

**Usage:** `mcr-protocol-deployer markdown workspace --relative-path <RELATIVE_PATH>`

###### **Options:**

* `--relative-path <RELATIVE_PATH>` — The file to write out to, relative to the crate root



## `mcr-protocol-deployer eth`

**Usage:** `mcr-protocol-deployer eth <COMMAND>`

###### **Subcommands:**

* `apply` — 
* `destroy` — 



## `mcr-protocol-deployer eth apply`

**Usage:** `mcr-protocol-deployer eth apply <COMMAND>`

###### **Subcommands:**

* `where` — Run config with all parameters passed explicitly as CLI flags
* `using` — Run config with parameters from environment variables, config files, and CLI flags



## `mcr-protocol-deployer eth apply where`

Run config with all parameters passed explicitly as CLI flags

**Usage:** `mcr-protocol-deployer eth apply where [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --contract-admin <CONTRACT_ADMIN>`

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



## `mcr-protocol-deployer eth apply using`

Run config with parameters from environment variables, config files, and CLI flags

**Usage:** `mcr-protocol-deployer eth apply using [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> [EXTRA_ARGS]...`

###### **Arguments:**

* `<EXTRA_ARGS>`

###### **Options:**

* `--script-args-path <SCRIPT_ARGS_PATH>`
* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



## `mcr-protocol-deployer eth destroy`

**Usage:** `mcr-protocol-deployer eth destroy [OPTIONS] --signer-identifier <SIGNER_IDENTIFIER> --fork-url <FORK_URL> --proxy-admin <PROXY_ADMIN> --move-token-proxy <MOVE_TOKEN_PROXY> --staking-proxy <STAKING_PROXY> --mcr-proxy <MCR_PROXY> --reward-proxy <REWARD_PROXY>`

###### **Options:**

* `--signer-identifier <SIGNER_IDENTIFIER>` — The signer identifier
* `--fork-url <FORK_URL>` — The fork url for deployment
* `--proxy-admin <PROXY_ADMIN>` — The proxy admin
* `--move-token-proxy <MOVE_TOKEN_PROXY>` — The move token proxy
* `--staking-proxy <STAKING_PROXY>` — The staking proxy
* `--mcr-proxy <MCR_PROXY>` — The mcr proxy
* `--reward-proxy <REWARD_PROXY>` — The reward proxy
* `--jsonl-prefix <JSONL_PREFIX>` — The JSONL prefix to give to the output from the deployer



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

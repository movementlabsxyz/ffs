# `cli`

The `cli` directory contains the command line interface for FFS.

## Overview

The `cli` directory contains the command line interface for FFS. It is organized into the following subdirectories:

- [`ffs-client`](./ffs-client/README.md): the command line interface for FFS clients
- [`ffs-dev`](./ffs-dev/README.md): the command line interface for FFS developers
- [`ffs`](./ffs/README.md): the main FFS command line tool

## Usage

We recommend using [`ffs-dev`](sdk/cli/ffs-dev/README.md) as the best starting point for using this repo.

To build `ffs-dev` manually you can run the following command:

```bash
cargo build -p ffs-dev --release
```

The `ffs-dev` binary will then be available in `target/release/ffs-dev`.

> [!NOTE]
> We use [`clap`](https://docs.rs/clap/latest/clap/) to build our CLIs, so you can always call `--help` to get a list of available commands and their usage.

The commands are composed as follows:

```
./target/release/ffs-dev 
  <protocol> // the protocol to run
  <ffs-dev subcommands> // subcommands for the protocol
  eth <anvil-command> // the anvil command to run
  <using/where> // the using or where command to run
  --config-path <config-file> // only if `using` 
  -- <any anvil data> // any anvil data to pass to anvil
```

**CLI supported protocols**

The following protocols are supported:

- `mcr` (Multi-Commit Rollup Protocol)
- `pcp` (Postconfirmation Protocol)

**`where` and `using`**

Many of our CLI subcommands share a common pattern where `where` and `using` subcommand variants are tied into the same logic, but accept different parameters.

> [!NOTE]
> A helpful pattern is to check command requirements with `where` and then develop with `using`.

- **`where`**: Explicitly requires parameters to be passed in as args. This is best for when you're learning to use a given command, or want to see what is necessary to run a command.
- **`using`**: Allows parameters to be passed in a hierarchy from environment variables, to config files, to command line args in order of override. This is useful for production settings. The subcommand will still validate the config.

**Example**
For an example for `using`, observe the config logged at the top, when running the following command:

```bash
UP_CONTRACT_ADMIN=0x911 ./target/release/ffs-dev mcr network coordinator eth anvil up using --config-path ./example/using.json -- --fork-url http://localhost:8545
```

where

- `UP_CONTRACT_ADMIN`: sets an environment variable
- `mcr`: uses the `mcr` sub-protocol
- `network` and `coordinator`: some parameters the sub-protocol
- `eth anvil up`: uses Ethereum Anvil local testnet
- `using`: uses the example config file
- passes the fork url to anvil

## CLI Conventions

This section outlines the conventions used across FFS CLIs. Following these conventions ensures consistent user experience across all FFS CLIs and prevents namespace conflicts between protocols.

### Naming

- CLI commands use kebab-case: `post-commitment` not `postCommitment`
- Rust types use PascalCase: `PostCommitment`
- Modules use snake_case: `post_commitment`

### Module Organization

Modules are implemented as subdirectories in the `cli` directory of a given protocol, e.g. in `protocol/mcr/client/src/cli/`.

```
cli/
  mod.rs           # Main CLI structure
  post_commitment/ # Command-specific module
  eth/
```

This structure keeps related code together and makes it easy to add new commands without duplicating code.

### Testing

- Each CLI command should have integration tests
- Use `clap`'s test utilities for argument parsing
- Test both success and error cases

### Documentation

- Use doc comments to explain command purpose
- Include examples in complex commands
- Document any environment variables or config files needed

### Nested Commands (Command Hierarchy)

Commands are organized in a tree structure, where each level narrows down the scope:

```
ffs-dev                    # Root level
   └── mcr                 # Protocol level
       └── network         # Component level
           └── coordinator # Action level
```

This tree structure is implemented through nested enums in the code. Each level in the tree is represented by its own enum:

```rust
// Root level: ffs-dev
#[derive(Parser)]
pub struct FfsDev {
    #[clap(subcommand)]
    command: Option<FfsDevSubcommand>,
}

// Protocol level: mcr
#[derive(Subcommand)]
pub enum FfsDevSubcommand {
    Mcr(McrCommands),  // branches into MCR-specific commands
    Pcp(PcpCommands),  // branches into PCP-specific commands
}

// Component level: network
#[derive(Subcommand)]
pub enum McrCommands {
    Network(NetworkCommands),  // branches into network-specific commands
}

// Action level: coordinator
#[derive(Subcommand)]
pub enum NetworkCommands {
    Coordinator(CoordinatorCommands),  // final level with actual actions
}
```

Each level in the tree corresponds to a nested enum in the code, allowing commands to be organized hierarchically.

This structure ensures:

- each protocol gets its own namespace (e.g., `mcr` commands won't conflict with `pcp` commands)
- users can discover commands by moving from general to specific (e.g., protocol → component → action)
- each protocol can add its own specific commands without affecting others

**Example:**
❌ Bad:

```rust
// Conflicts when multiple protocols have "run" command
enum Commands {
    Run,
    Start,
}
```

✅ Good:

```rust
enum McrCommands {
    Run,
}

enum PcpCommands {
    Run,
}
```

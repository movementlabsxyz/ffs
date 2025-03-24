<div align="center">
  <pre>
          ------------
f ------ |            |
f ------ | 5e771e3e47 |
5 ------ |            |
          ------------
  </pre>
</div>

# ffs

> FFS, just take a vote.

Movement Labs' Fast Finality Settlement is a proof of stake settlement system.

## Getting started

We develop in nix. Hence start by entering the nix shell:

```bash
nix develop
```

The easiest entry point for all protocols and use cases is the [`ffs-dev`](sdk/cli/ffs-dev/README.md) CLI. Subcomponents of `ffs-dev` will have their own CLIs and these CLIs have their core libraries.

For example, to spin up a network with Anvil, you can run the following command (after you build the `ffs-dev` binary):

```bash
./target/release/ffs-dev mcr network coordinator eth anvil up
```

For a more in-depth usage guide, see [Usage](#usage).

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |

Please see the [CONTRIBUTING.md](CONTRIBUTING.md) file for contribution guidelines.

## Organization

There are five subdirectories which progressively build on one another for node logic.

1. [`util`](./util): contains utility logic mainly reused in [`protocol`](./protocol).
2. [`protocol`](./protocol): contains implementations of the protocol logic.
3. [`node`](./node): contains single-process runnable binaries that aggregate the protocol logic.
4. [`network`](./network): contains logic for running multiple nodes in a network.
5. [`sdk`](./sdk): contains logic for interacting nodes and networks.

There are several other subdirectories of note:

- [`spec`](./spec): contains formal verification of FFS protocols.

## Usage
We recommend  [`ffs-dev`](sdk/cli/ffs-dev/README.md) as the best starting point for using this repo. 

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

### Crates

To better understand the available crates we recommend reviewing the `cargo doc` documentation:

```bash
cargo doc --open --no-deps
```

### Production

Coming soon!

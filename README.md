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

For a more in-depth usage guide, see [Usage of CLI](sdk/cli/README.md).

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

### Crates

To better understand the available crates we recommend reviewing the `cargo doc` documentation:

```bash
cargo doc --open --no-deps
```

### Production

Coming soon!

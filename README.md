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

The Fast Finality Settlement is a proof of stake settlement system for Movement Network.

## Getting started
To get started using `ffs` we recommending reviewing our CLI documentation:

- [The SDK CLI Guide](sdk/cli/README.md) provides a general introduction to the CLI.
- [The CLI Docs](docs/cli/README.md) provide subcommand level documentation of the CLIs. 

If you are interested in lower-level programmatic usage, we recommend reading through the crate docs:

```bash
cargo doc --open --no-deps
```

To develop a better sense of the protocol, please review the associated MIPs, beginning with [MIP-34](https://github.com/movementlabsxyz/MIP/pull/34). 

If you are considering using `ffs` in production, jump to [Production](#production). 

## Contributing

| Task | Description |
|------|-------------|
| [Upcoming Events](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Apriority%3Ahigh%2Cpriority%3Amedium%20label%3Aevent) | High-priority `event` issues with planned completion dates. |
| [Release Candidates](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Arelease-candidate) | Feature-complete versions linked to events. |
| [Features & Bugs](https://github.com/movementlabsxyz/ffs/issues?q=is%3Aissue%20state%3Aopen%20label%3Afeature%2Cbug%20label%3Apriority%3Aurgent%2Cpriority%3Ahigh) | High-priority `feature` and `bug` issues. |

Please see [CONTRIBUTING.md](CONTRIBUTING.md) file for additional contribution guidelines.

## Organization

There are five subdirectories which progressively build on one another for node logic.

1. [`util`](./util): contains utility logic mainly reused in [`protocol`](./protocol).
2. [`protocol`](./protocol): contains implementations of the protocol logic.
3. [`node`](./node): contains single-process runnable binaries that aggregate the protocol logic.
4. [`network`](./network): contains logic for running multiple nodes in a network.
5. [`sdk`](./sdk): contains logic for interacting nodes and networks.

There are several other subdirectories of note:

- [`spec`](./spec): contains formal verification of FFS protocols.

## Production

Coming soon!

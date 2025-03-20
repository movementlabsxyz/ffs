# `ffs-client`

The command line tool for interacting with live FFS services.

The `ffs-client` relies on the implementation of the following components in each of the sub-protocols in the `[protocol/](../protocol/README.md)` directory:

- `cli/client/` - Command line interface tools for interacting with the protocol
- `clients/eth/` - Ethereum client implementation that handles blockchain interactions

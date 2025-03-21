# `ffs-client`

The command line tool for interacting with live FFS services.

The `ffs-client` relies on the implementation of the following components in each of the sub-protocols in the `[protocol/](../protocol/README.md)` directory:

- `cli/client/` - Command line interface tools for interacting with the protocol
- `clients/eth/` - Ethereum client implementation that handles blockchain interactions

## Commands

### Some test commands

Post a commitment using a hex-encoded commitment

```bash
cargo run --bin ffs-client -- protocol mcr post-commitment --commitment-hex <hex>
cargo run --bin ffs-client -- protocol pcp post-commitment --commitment-hex <hex>
    ```

Post a commitment using a preimage string

```bash
cargo run --bin ffs-client -- protocol mcr post-commitment --preimage-string <string>
cargo run --bin ffs-client -- protocol pcp post-commitment --preimage-string <string>
```

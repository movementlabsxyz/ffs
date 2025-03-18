# `ffs-client`

The command line tool for interacting with live FFS services.

## Commands

### Some test commands

Post a commitment to an MCR implementation:

Post using a hex-encoded commitment

```bash
cargo run --bin ffs-client -- protocol pcp post-commitment --commitment-hex <hex>
```

Post using a preimage string

```bash
cargo run --bin ffs-client -- protocol pcp post-commitment --preimage-string <string>
```

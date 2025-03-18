# `ffs-client`

The command line tool for interacting with live FFS services.

## Commands

### Protocol MCR

Post a commitment to an MCR implementation:

Post using a hex-encoded commitment

```bash
ffs-client protocol mcr post-commitment --commitment-hex <hex>
```

Post using a preimage string

```bash
ffs-client protocol mcr post-commitment --preimage-string <string>
```

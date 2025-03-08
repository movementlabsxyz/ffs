# `protocol-units`
`protocol-units` are used to collect runnable and composable units for interacting with the `ffs` protocol. We store contracts within protocol units, for example. 

## Contents
- **[`mcr`](./mcr/)**: the `MCR` implementation of `ffs` provided prior to the specification of the `fast-confirmations` and `post-confirmations` protocols. It is maintained for legacy purposes.
- **[`pcp`](./pcp/)**: the Postconfirmations sub-protocol implementation for `ffs`. 
- **[`fcp`](./fast-confirmations/)**: the fastconfirmations sub-protocol implementation for `ffs`.
- **[`ffs`](./ffs/)**: the full `ffs` protocol implementation.

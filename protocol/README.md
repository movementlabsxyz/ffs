# `protocol-units`

`protocol-units` are used to collect runnable and composable units for interacting with the `ffs` protocol. We store contracts within protocol units, for example.

## Contents

- **[`mcr`](./mcr/README.md)**: the `MCR` sub-protocol implementation of `ffs` provided prior to the specification of the `fast-confirmations` and `post-confirmations` protocols. It is maintained for legacy purposes.
- **[`pcp`](./pcp/README.md)**: the Postconfirmation sub-protocol implementation for `ffs`.
- **[`fcp`](./fcp/README.md)**: the Fastconfirmation sub-protocol implementation for `ffs`.
- **[`ffs`](./ffs/README.md)**: the full `ffs` protocol implementation.

## Architecture for each protocol unit

- `cli/` - Command line interface tools
  - `client/` - CLI tool for users to interact with the protocol (posting commitments, querying state)
  - `deployer/` - CLI tool for deploying the protocol contracts
  - `protocol/` - CLI tool for protocol-specific operations

- `clients/` - Protocol client implementations
  - `eth/` - Ethereum client implementation that handles blockchain interactions
  - `mock/` - Mock client for testing
  - `util/` - Shared utilities for clients

- `dlu/` - Deployment and Lifecycle Utilities
  - `eth/` - Ethereum-specific deployment tools
    - `contracts/` - Smart contract implementations. Includes settlement contracts for block commitments, staking contracts for validator management, token contracts for custody.
    - `deployer-core/` - Core deployment logic
    - `anvil/` - Local testnet configuration

- `manager/` - Protocol management and orchestration. Manages block commitments by batching and submitting them, interacts with clients, and processes commitment events (acceptance or rejection) for the settlement system.

- [`util/`](util/) - Shared utilities
  - `config/` - Configuration management
  - `types/` - Common type definitions

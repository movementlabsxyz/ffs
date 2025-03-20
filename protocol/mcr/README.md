# MCR (Multi Commit Rollup)

This directory contains the implementation of the MCR protocol, which handles rollup commitment aggregation.

## Overview

**MCR** implements a staking-based settlement where validators commit L2-blocks on Layer 1 (L1). MCR is responsible for aggregating multiple commitments into a single commitment. It provides:

The distinguishing feature is that there is only one type of actor, which is the attester.

Validators stake tokens to participate in block validation. They commit to L2-blocks on L1, and the contract on L1 tracks block commitments, epochs, and stake. The contracts also manage validators and custodian staking and unstaking. The contract validates if commitments have reached two-thirds supermajority stake, and rewards or slashes validators based on their actions.

For further details see the [RFC for MCR](https://github.com/movementlabsxyz/rfcs/pull/29) and the [MIP-34](https://github.com/movementlabsxyz/MIP/blob/main/MIP/mip-34).

## Architecture

- [`cli/`](cli/) - Command line interface tools
  - [`client/`](cli/client/) - Client implementation for interacting with the protocol. Handles interaction with the protocol by posting block commitments, streaming commitment data, and managing Ethereum blockchain interactions.
  - [`deployer/`](cli/deployer/) - Tools for deploying the protocol contracts
  - [`protocol/`](cli/protocol/) - Core protocol implementation

- [`clients/`](clients/) - Protocol client implementations
  - [`eth/`](clients/eth/) - Ethereum client implementation
  - [`mock/`](clients/mock/) - Mock client for testing
  - [`util/`](clients/util/) - Shared utilities for clients

- [`dlu/`](dlu/) - Deployment and Lifecycle Utilities
  - [`eth/`](dlu/eth/) - Ethereum-specific deployment tools
    - [`contracts/`](dlu/eth/contracts/) - Smart contract implementations. Includes settlement contracts for block commitments, staking contracts for validator management, token contracts for custody.
    - [`deployer-core/`](dlu/eth/deployer-core/) - Core deployment logic
    - [`anvil/`](dlu/eth/anvil/) - Local testnet configuration

- [`manager/`](manager/) - Protocol management and orchestration. Manages block commitments by batching and submitting them, interacts with clients, and processes commitment events (acceptance or rejection) for the settlement system.

- [`util/`](util/) - Shared utilities
  - [`config/`](util/config/) - Configuration management
  - [`types/`](util/types/) - Common type definitions

-----

TODO: remove once we have used this content at the correct place

- **Setup**: Prepares local environments or deploys contracts, manages configuration for local and deployment setups, and ensures contract deployment when needed.
- **Runner**: Orchestrates the setup and execution of configuration tasks, applies setup steps, and logs processes for debugging.


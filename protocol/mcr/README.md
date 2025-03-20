# MCR (Multi Commit Rollup)

This directory contains the implementation of the MCR protocol, which handles rollup commitment aggregation.

## Overview

**MCR** implements a staking-based settlement where validators commit to a state from L2 on Layer 1 (L1). MCR accepts commitments from all staked attesters and aggregates them into a single confirmation. The contracts on L1 tracks block commitments, epochs, stake and rewards.

The distinguishing feature to PCP is that there is only one type of actor, which is the attester.

For further details see the [RFC for MCR](https://github.com/movementlabsxyz/rfcs/pull/29) and the [MIP-34](https://github.com/movementlabsxyz/MIP/blob/main/MIP/mip-34).

## Architecture

- [`cli/`](cli/) - Command line interface tools
  - [`client/`](cli/client/) - CLI tool for users to interact with the protocol (posting commitments, querying state)
  - [`deployer/`](cli/deployer/) - CLI tool for deploying the protocol contracts
  - [`protocol/`](cli/protocol/) - CLI tool for protocol-specific operations

- [`clients/`](clients/) - Protocol client implementations
  - [`eth/`](clients/eth/) - Ethereum client implementation that handles blockchain interactions
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

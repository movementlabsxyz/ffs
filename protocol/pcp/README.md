# PCP (Postconfirmation Protocol)

This directory contains the implementation of the PCP protocol, which handles commitment settlement through postconfirmation.

## Overview

**PCP** implements a staking-based settlement where validators commit to a state from L2 on Layer 1 (L1). PCP accepts commitments from all staked attesters. A specialized attester - the acceptor - aggregates them into a single confirmation, called a postconfirmation. The contracts on L1 tracks block commitments, epochs, stake and rewards.

The distinguishing feature to MCR is that there are two types of actors, which are the attesters and the acceptors.

For further details see the [MIP-37](https://github.com/movementlabsxyz/MIP/blob/main/MIP/mip-37).

## Architecture

- [`cli/`](cli/) - Command line interface tools
  - [`client/`](cli/client/) - CLI tool for users to interact with the protocol (posting commitments, querying state)
  - [`deployer/`](cli/deployer/) - CLI tool for deploying the protocol contracts
  - [`protocol/`](cli/protocol/) - CLI tool for protocol-specific operations

- [`dlu/`](dlu/) - Deployment and Lifecycle Utilities
  - [`eth/`](dlu/eth/) - Ethereum-specific deployment tools
    - [`contracts/`](dlu/eth/contracts/) - Smart contract implementations for post-confirmation settlement
    - [`deployer-core/`](dlu/eth/deployer-core/) - Core deployment logic for contracts
    - [`anvil/`](dlu/eth/anvil/) - Local testnet configuration for development

- [`util/`](util/) - Shared utilities
  - [`config/`](util/config/) - Configuration management for PCP clients and deployments
  - [`types/`](util/types/) - Common type definitions for PCP protocol

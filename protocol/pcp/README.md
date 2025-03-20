# PCP (Postconfirmation Protocol)

This directory contains the implementation of the PCP protocol, which handles commitment settlement through postconfirmation.

## Overview

PCP (Postconfirmation Protocol) is responsible for handling post-confirmation commitment settlement between different parts of the system. It provides:

1. Contract deployment and management
2. Commitment posting and verification
3. Client interfaces for interacting with the protocol
4. Testing and mock implementations

## Architecture

- [`cli/`](cli/) - Command line interface tools
  - [`client/`](cli/client/) - Client implementation for interacting with PCP. Handles posting commitments and managing blockchain interactions.
  - [`deployer/`](cli/deployer/) - Tools for deploying PCP contracts
  - [`protocol/`](cli/protocol/) - Core protocol implementation

- [`dlu/`](dlu/) - Deployment and Lifecycle Utilities
  - [`eth/`](dlu/eth/) - Ethereum-specific deployment tools
    - [`contracts/`](dlu/eth/contracts/) - Smart contract implementations for post-confirmation settlement
    - [`deployer-core/`](dlu/eth/deployer-core/) - Core deployment logic for contracts
    - [`anvil/`](dlu/eth/anvil/) - Local testnet configuration for development

- [`util/`](util/) - Shared utilities
  - [`config/`](util/config/) - Configuration management for PCP clients and deployments
  - [`types/`](util/types/) - Common type definitions for PCP protocol

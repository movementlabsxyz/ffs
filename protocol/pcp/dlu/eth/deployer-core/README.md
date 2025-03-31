# Deployer Core

A Rust library that manages contract deployments for the PCP protocol. This library handles the complexities of contract deployment by providing a consistent workspace and configuration management.

## Overview

The deployer-core:

1. Manages contract workspaces
2. Embeds contract files (as a zip), and unzips them when needed
3. Handles deployment configuration and contract dependencies
4. Provides a safe interface for running deployment commands

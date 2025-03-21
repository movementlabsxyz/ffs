#!/bin/bash

# Remove existing libs
rm -rf lib/

# Install all dependencies
forge install foundry-rs/forge-std --no-commit
forge install OpenZeppelin/openzeppelin-contracts --no-commit
forge install safe-global/safe-smart-account --no-commit
forge install transmissions11/solmate --no-commit
forge install OpenZeppelin/openzeppelin-contracts-upgradeable --no-commit 
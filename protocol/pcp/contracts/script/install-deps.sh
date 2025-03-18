#!/bin/bash

echo "🧹 Cleaning up existing dependencies..."
rm -rf lib/openzeppelin-contracts
rm -rf lib/openzeppelin-contracts-upgradeable
rm -rf lib/safe-contracts
rm -rf lib/forge-std

echo "📦 Installing core dependencies..."
forge install foundry-rs/forge-std --no-git
forge install OpenZeppelin/openzeppelin-contracts --no-git
forge install OpenZeppelin/openzeppelin-contracts-upgradeable --no-git

echo "🔒 Installing Safe contracts with submodules..."
forge install safe-global/safe-contracts@v1.4.1 --no-git --recurse-submodules

echo "🔨 Verifying installation..."
forge build

echo "✅ Dependencies installed successfully!" 
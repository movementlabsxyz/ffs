#!/bin/bash

# Define source and destination directories
SOURCE_DIR="./out"
ETH_DEST_DIR="../../../clients/eth/abis"

# Create destination directories if they don't exist
mkdir -p "$ETH_DEST_DIR"

# Copy specific files
cp "$SOURCE_DIR/MCR.sol/MCR.json" "$ETH_DEST_DIR/"
cp "$SOURCE_DIR/MOVEToken.sol/MOVEToken.json" "$ETH_DEST_DIR/"
cp "$SOURCE_DIR/MovementStaking.sol/MovementStaking.json" "$ETH_DEST_DIR/"

echo "Specific ABI files copied successfully!" 
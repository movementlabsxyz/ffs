#!/bin/bash

# Define source and destination directories
SOURCE_DIR="./out"
CLIENT_DEST_DIR="../../../cli/client/abis"
ETH_DEST_DIR="../../../clients/eth/abis"

# Create destination directories if they don't exist
mkdir -p "$CLIENT_DEST_DIR"
mkdir -p "$ETH_DEST_DIR"

# Copy specific files
cp "$SOURCE_DIR/PCP.sol/PCP.json" "$CLIENT_DEST_DIR/"
cp "$SOURCE_DIR/MOVEToken.sol/MOVEToken.json" "$CLIENT_DEST_DIR/"
cp "$SOURCE_DIR/MovementStaking.sol/MovementStaking.json" "$CLIENT_DEST_DIR/"

cp "$SOURCE_DIR/PCP.sol/PCP.json" "$ETH_DEST_DIR/"
cp "$SOURCE_DIR/MOVEToken.sol/MOVEToken.json" "$ETH_DEST_DIR/"
cp "$SOURCE_DIR/MovementStaking.sol/MovementStaking.json" "$ETH_DEST_DIR/"

echo "Specific ABI files copied successfully!" 
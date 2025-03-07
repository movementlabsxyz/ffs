#!/bin/bash

# Check if RPC URL is provided
if [ -z "$1" ]; then
  echo "Usage: $0 <RPC_URL>"
  echo "Example: $0 http://127.0.0.1:50955"
  exit 1
fi

RPC_URL=$1

# Read the deployment addresses from the latest broadcast
LATEST_BROADCAST=$(ls -t broadcast/DeployMCRDev.s.sol/*/run-latest.json | head -n1)
# Get the proxy addresses (they're created after the implementations)
MOVE_TOKEN=$(cat $LATEST_BROADCAST | jq -r '.transactions[] | select(.contractName=="ERC1967Proxy") | .contractAddress' | sed -n '1p')
STAKING_PROXY=$(cat $LATEST_BROADCAST | jq -r '.transactions[] | select(.contractName=="ERC1967Proxy") | .contractAddress' | sed -n '2p')
MCR_PROXY=$(cat $LATEST_BROADCAST | jq -r '.transactions[] | select(.contractName=="ERC1967Proxy") | .contractAddress' | sed -n '3p')
DEPLOYER=$(cat $LATEST_BROADCAST | jq -r '.transactions[0].from')
PRIVATE_KEY="39725efee3fb28614de3bacaffe4cc4bd8c436257e2c8bb887c4b5c4be45e76d"

echo "=== Verifying MCR Deployment ==="
echo "MOVE Token: $MOVE_TOKEN"
echo "MCR Proxy: $MCR_PROXY"
echo "Staking Proxy: $STAKING_PROXY"
echo "Deployer: $DEPLOYER"./

echo -e "\n=== Checking MCR Configuration ==="
echo "Epoch Duration:"
cast call --rpc-url $RPC_URL $MCR_PROXY "getEpochDuration()(uint256)"
echo "Postconfirmer Duration:"
cast call --rpc-url $RPC_URL $MCR_PROXY "getPostconfirmerDuration()(uint256)"

echo -e "\n=== Checking Staking Setup ==="
echo "Epoch Duration for MCR domain:"
cast call --rpc-url $RPC_URL $STAKING_PROXY "epochDurationByDomain(address)(uint256)" $MCR_PROXY

echo -e "\n=== Checking Token Setup ==="
echo "Deployer Balance:"
cast call --rpc-url $RPC_URL $MOVE_TOKEN "balanceOf(address)(uint256)" $DEPLOYER
echo "Staking Contract has Minter Role:"
cast call --rpc-url $RPC_URL $MOVE_TOKEN "hasMinterRole(address)(bool)" $STAKING_PROXY

echo -e "\n=== Testing Staking ==="
echo "Approving tokens for staking..."
cast send --rpc-url $RPC_URL $MOVE_TOKEN "approve(address,uint256)" $STAKING_PROXY 1000ether --private-key $PRIVATE_KEY
echo "Staking tokens..."
cast send --rpc-url $RPC_URL $STAKING_PROXY "stake(address,uint256)" $MCR_PROXY 1000ether --private-key $PRIVATE_KEY

echo -e "\n=== Testing MCR Attestation ==="
echo "Submitting test attestation..."
cast send --rpc-url $RPC_URL $MCR_PROXY "attest(uint256,bytes32)" 1 0x1234567890123456789012345678901234567890123456789012345678901234 --private-key $PRIVATE_KEY

echo -e "\n=== Verification Complete ===" 
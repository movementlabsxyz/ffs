#!/bin/bash

# Require RPC URL and private key as parameters
if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Error: Both RPC URL and private key are required"
    echo "Usage: $0 <RPC_URL> <PRIVATE_KEY>"
    echo "Example: $0 http://127.0.0.1:60743 0123456789abcdef... (private key without 0x prefix)"
    exit 1
fi

RPC_URL=$1
PRIVATE_KEY=$2
echo "Using RPC URL: $RPC_URL"

# Get the address from the private key
ADDRESS=$(cast wallet address --private-key $PRIVATE_KEY)
echo "Using address: $ADDRESS"

# Check balance
BALANCE=$(cast balance $ADDRESS --rpc-url $RPC_URL)
echo "Current balance: $BALANCE ETH"

if [ "$BALANCE" = "0" ]; then
    echo "Error: Account has no funds. Please fund it first using:"
    echo "cast send --private-key <FUNDED_KEY> $ADDRESS --value 1ether --rpc-url $RPC_URL"
    exit 1
fi

# Create a file to store the addresses
DEPLOYMENT_FILE="script/helpers/safe-deployments.json"
echo "{}" > $DEPLOYMENT_FILE

# Deploy Safe singleton
echo "Deploying Safe singleton..."
SAFE_OUTPUT=$(forge create lib/safe-contracts/contracts/Safe.sol:Safe \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --gas-price 100000000000 \
  --legacy)
SAFE_ADDRESS=$(echo "$SAFE_OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo "Safe deployed to: $SAFE_ADDRESS"

# Deploy Fallback Handler
echo "Deploying Fallback Handler..."
HANDLER_OUTPUT=$(forge create lib/safe-contracts/contracts/handler/CompatibilityFallbackHandler.sol:CompatibilityFallbackHandler \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --gas-price 100000000000 \
  --legacy)
HANDLER_ADDRESS=$(echo "$HANDLER_OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo "Fallback Handler deployed to: $HANDLER_ADDRESS"

# Deploy Safe Factory
echo "Deploying Safe Factory..."
FACTORY_OUTPUT=$(forge create lib/safe-contracts/contracts/proxies/SafeProxyFactory.sol:SafeProxyFactory \
  --rpc-url $RPC_URL \
  --private-key $PRIVATE_KEY \
  --gas-price 100000000000 \
  --legacy)
FACTORY_ADDRESS=$(echo "$FACTORY_OUTPUT" | grep "Deployed to:" | awk '{print $3}')
echo "Safe Factory deployed to: $FACTORY_ADDRESS"

# Save addresses to JSON file
cat > $DEPLOYMENT_FILE << EOF
{
  "Safe": "$SAFE_ADDRESS",
  "FallbackHandler": "$HANDLER_ADDRESS",
  "SafeFactory": "$FACTORY_ADDRESS"
}
EOF

echo "Deployment addresses saved to $DEPLOYMENT_FILE" 
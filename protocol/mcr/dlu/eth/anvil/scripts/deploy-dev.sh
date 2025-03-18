#!/bin/bash
set -e

# ANVIL_URL should either be a a default value or the one specified by the surrounding en
ANVIL_URL=${ANVIL_URL:-http://127.0.0.1:8545}

# SCRIPT_PATH should be assumed to be relative to the ETH DLU
SCRIPT_PATH=${SCRIPT_PATH:-"./contracts/script/DeployMCRDev.s.sol"}

# Validate that PRIVATE_KEY is set
if [ -z "$PRIVATE_KEY" ]; then
  echo "Error: PRIVATE_KEY is not set."
  exit 1
fi

# Now run the script to deploy the contracts
forge script $SCRIPT_PATH --rpc-url $ANVIL_URL --private-key $PRIVATE_KEY
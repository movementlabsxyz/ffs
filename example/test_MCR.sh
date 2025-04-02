#!/bin/bash


# NOTE: 
# the functions that we need, are defined in the following files:
# ./protocol/mcr/clients/eth/src/client/mod.rs
# ./protocol/mcr/clients/mock/src/lib.rs
# ./protocol/mcr/clients/util/src/lib.rs

# Exit on error
# set -e
# Print each command before executing
# set -x

# This script tests the MCR contracts using anvil.

# The contract should be deployed in a separate terminal using
# UP_CONTRACT_ADMIN=0x911 ./target/debug/ffs-dev mcr network coordinator eth anvil up using --config-path ./example/using.json -- --fork-url http://localhost:8545

# - - - - - - - - - - - - - - - - - - - - - - - - 
# - - - - - - - Definitions - - - - - - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - 

# At this point, the contracts should be deployed and the following environment variables should be set:

# Function to check block height and advance one block
advance_by_a_block() {
    echo -n "... block height: "
    cast block-number --rpc-url http://localhost:8545
    sleep 1  # wait for async calls to complete
    cast rpc anvil_mine --rpc-url http://localhost:8545 > /dev/null # Advance 1 block
}

export PRIVATE_KEY_A=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
export PRIVATE_KEY_B=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
export PRIVATE_KEY_C=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
export ADDRESS_A=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
export ADDRESS_B=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
export ADDRESS_C=0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC

# moveTokenProxy
export MOVE_TOKEN=0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9
# movementStakingProxy
export MOVEMENT_STAKING=0x5FC8d32690cc91D4c39d9d3abcBD16989F875707
# mcrProxy
export MCR=0x0165878A594ca255338adfa4d48449f69242Eb8F

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# - - - - - - - Provide accounts with MOVE tokens - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

# Some basic tests
echo -n "Token Symbol: "
cast call $MOVE_TOKEN "symbol()" --rpc-url http://localhost:8545 | tr -d '\n' | cast --to-ascii
echo -n -e "Total Supply: "
cast call $MOVE_TOKEN "totalSupply()" --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
echo -n -e "Balance of Address B: "
cast call $MOVE_TOKEN "balanceOf(address)" $ADDRESS_B --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

# Transfer 1000 MOVE tokens to each address
echo "Transferring 1000 MOVE tokens to each address..."
AMOUNT=$((1000 * 100000000))  # = 100000000000
cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_C $AMOUNT --rpc-url http://localhost:8545 > /dev/null

echo "Transfers complete!"

# Check initial balances
echo "=================== Balances and transfers ==================="
for letter in {A..C}; do
    echo -n "Initial MOVE Balance of Address $letter: "
    cast call $MOVE_TOKEN "balanceOf(address)" $(eval echo \$ADDRESS_$letter) --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
done

# Test transfer C -> B
SENDAMOUNT=$((AMOUNT/10))
echo "C -> B: $(echo "scale=8; $SENDAMOUNT/100000000" | bc) MOVE"
cast send --private-key $PRIVATE_KEY_C $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_B $SENDAMOUNT --rpc-url http://localhost:8545 > /dev/null
# Test transfer B -> C
SENDAMOUNT=$((AMOUNT/100))
echo "B -> C: $(echo "scale=8; $SENDAMOUNT/100000000" | bc) MOVE"
cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_C $SENDAMOUNT --rpc-url http://localhost:8545 > /dev/null

# Check final balances
for letter in {A..C}; do
    echo -n "Final MOVE Balance of Address $letter: "
    cast call $MOVE_TOKEN "balanceOf(address)" $(eval echo \$ADDRESS_$letter) --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
done

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# - - - - - - - Staking - - - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

echo -e "\n================== Staking for Address C =================="

echo -n "Current MOVE allowance for staking contract: "
cast call $MOVE_TOKEN "allowance(address,address)" $ADDRESS_C $MOVEMENT_STAKING --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
if [ $(cast call $MOVE_TOKEN "allowance(address,address)" $ADDRESS_C $MOVEMENT_STAKING --rpc-url http://localhost:8545 | cast --to-dec) -ne 0 ]; then
    echo "!!!  WARNING: MOVE token allowance is not 0. We may be rerunning this script."
fi

echo -n "Staked MOVE for Address C: "
cast call $MOVEMENT_STAKING "getCurrentEpochStake(address,address,address)" \
    $MCR \
    $MOVE_TOKEN \
    $ADDRESS_C \
    --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc


# Stake using CLI command with explicit private key
# Approve staking contract to spend MOVE tokens
cast send --private-key $PRIVATE_KEY_C $MOVE_TOKEN "approve(address,uint256)" $MOVEMENT_STAKING 10000000 --rpc-url http://localhost:8545 > /dev/null
if [ $? -ne 0 ]; then
    echo "Error: Failed to approve MOVE tokens"
    exit 1
fi

# check if token is approved
echo -n "Current MOVE allowance for staking contract: "
cast call $MOVE_TOKEN "allowance(address,address)" $ADDRESS_C $MOVEMENT_STAKING --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
# current MOVE balance of Address C
echo -n "Current MOVE balance of Address C: "
cast call $MOVE_TOKEN "balanceOf(address)" $ADDRESS_C --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

# Check if address C is whitelisted
WHITELIST_CHECK=$(cast call $MOVEMENT_STAKING "hasRole(bytes32,address)" \
    $(cast keccak "WHITELIST_ROLE") \
    $ADDRESS_C \
    --rpc-url http://localhost:8545)
if [ "$WHITELIST_CHECK" = "0x0000000000000000000000000000000000000000000000000000000000000000" ]; then
    # If not whitelisted, whitelist it using admin account
    echo "Whitelisting Address C..."
    cast send --private-key $PRIVATE_KEY_A $MOVEMENT_STAKING "whitelistAddress(address)" \
        $ADDRESS_C \
        --rpc-url http://localhost:8545 > /dev/null
fi

# cast call if address C is whitelisted
# we dont need to print the result, we just want to check if it is whitelisted
cast call $MOVEMENT_STAKING "hasRole(bytes32,address)" \
    $(cast keccak "WHITELIST_ROLE") \
    $ADDRESS_C \
    --rpc-url http://localhost:8545 > /dev/null
if [ $? -ne 0 ]; then
    echo "Error: Address C still not whitelisted"
    exit 1
else
    echo "Address C is whitelisted"
fi

# Stake MOVE octas
./target/debug/ffs-dev mcr protocol client eth stake \
    --amount 1002 \
    --private-key $PRIVATE_KEY_C \
    --mcr-address $MCR \
    --move-token-address $MOVE_TOKEN \
    --address $ADDRESS_C \
    --staking-address $MOVEMENT_STAKING
if [ $? -ne 0 ]; then
    echo "Error: Failed to stake tokens"
    exit 1
fi

advance_by_a_block

# Verify stake
echo -n "Staked MOVE for Address C: "
cast call $MOVEMENT_STAKING "getCurrentEpochStake(address,address,address)" \
    $MCR \
    $MOVE_TOKEN \
    $ADDRESS_C \
    --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# - - - - - - - Granting roles - - - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

echo -e "\n================== Granting roles =================="

# Check if Attester is a coimmtment admin (it should not be!)
COMMITMENT_ADMIN_CHECK=$(cast call $MCR "hasRole(bytes32,address)" \
    $(cast keccak "COMMITMENT_ADMIN") \
    $ADDRESS_C \
    --rpc-url http://localhost:8545)
# if it is a commitment admin, exit
if [ "$COMMITMENT_ADMIN_CHECK" = "0x0000000000000000000000000000000000000000000000000000000000000001" ]; then
    echo "Error: Address C is a commitment admin"
    exit 1
fi

# Check if openAttestationEnabled is true
OPEN_ATTESTATION=$(cast call $MCR "openAttestationEnabled()" --rpc-url http://localhost:8545)
if [ "$OPEN_ATTESTATION" = "0x0000000000000000000000000000000000000000000000000000000000000001" ]; then
    echo "Error: Open attestation is enabled"
    exit 1
fi

# Check if address C has TRUSTED_ATTESTER role
TRUSTED_ATTESTER_CHECK=$(cast call $MCR "hasRole(bytes32,address)" \
    $(cast call $MCR "TRUSTED_ATTESTER()" --rpc-url http://localhost:8545) \
    $ADDRESS_C \
    --rpc-url http://localhost:8545)
if [ "$TRUSTED_ATTESTER_CHECK" = "0x0000000000000000000000000000000000000000000000000000000000000000" ]; then
    echo "Address C is not a trusted attester"
    # Grant TRUSTED_ATTESTER role using admin account
    echo "Granting TRUSTED_ATTESTER role to Address C..."
    ./target/debug/ffs-dev mcr protocol client eth grant-trusted-attester \
        --attester $ADDRESS_C \
        --mcr-address $MCR \
        --private-key ${PRIVATE_KEY_A#"0x"}
    # Check if the transaction was successful
    if [ $? -ne 0 ]; then
        echo "Error: Failed to grant TRUSTED_ATTESTER role"
        exit 1
    fi
    # now check again and if this did not work, exit
    TRUSTED_ATTESTER_CHECK=$(cast call $MCR "hasRole(bytes32,address)" \
        $(cast call $MCR "TRUSTED_ATTESTER()" --rpc-url http://localhost:8545) \
        $ADDRESS_C \
        --rpc-url http://localhost:8545)
    if [ "$TRUSTED_ATTESTER_CHECK" = "0x0000000000000000000000000000000000000000000000000000000000000000" ]; then
        echo "Error: Failed to grant TRUSTED_ATTESTER role to Address C"
        echo "TRUSTED_ATTESTER_CHECK: $TRUSTED_ATTESTER_CHECK"
        exit 1
    else
        echo "Address C is a trusted attester"
    fi
else 
    echo "Address C is a trusted attester"
fi


advance_by_a_block

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# - - - - - - - Posting commitment - - - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

echo -e "\n================== Posting Commitment =================="


# Check commitment before posting (should show nothing)
./target/debug/ffs-dev mcr protocol client eth check-commitment \
    --height 1 \
    --mcr-address $MCR \
    --attester $ADDRESS_C \
    --private-key $PRIVATE_KEY_C

# Check if commitment was accepted (should show nothing)
./target/debug/ffs-dev mcr protocol client eth check-postconfirmation \
    --height 1 \
    --mcr-address $MCR \
    --private-key $PRIVATE_KEY_C
advance_by_a_block

# Check again if Address C has TRUSTED_ATTESTER role
TRUSTED_ATTESTER_CHECK=$(cast call $MCR "hasRole(bytes32,address)" \
    $(cast call $MCR "TRUSTED_ATTESTER()" --rpc-url http://localhost:8545) \
    $ADDRESS_C \
    --rpc-url http://localhost:8545)
if [ "$TRUSTED_ATTESTER_CHECK" = "0x0000000000000000000000000000000000000000000000000000000000000000" ]; then
    echo "Error: Address C is not a trusted attester"
    exit 1
fi

# Post commitment using the CLI
echo "Posting commitment for Address C..."
./target/debug/ffs-dev mcr protocol client eth post-commitment \
    --preimage-string "commitment_from_C" \
    --private-key $PRIVATE_KEY_C \
    --height 1 \
    --mcr-address $MCR
if [ $? -ne 0 ]; then
    echo "Error: Failed to post commitment"
    exit 1
fi

advance_by_a_block

# Check attester's commitment after posting
echo "Checking attester's commitment after posting..."
./target/debug/ffs-dev mcr protocol client eth get-block-commitment \
    --height 1 \
    --mcr-address $MCR \
    --attester $ADDRESS_C \
    --private-key $PRIVATE_KEY_C

# Check if commitment was accepted
echo "Checking if commitment was accepted..."
./target/debug/ffs-dev mcr protocol client eth get-accepted-commitment-at-height \
    --height 1 \
    --mcr-address $MCR \
    --private-key $PRIVATE_KEY_C

echo "Setup complete!"

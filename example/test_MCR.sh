#!/bin/bash

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
export PRIVATE_KEY_A=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
export PRIVATE_KEY_B=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
export PRIVATE_KEY_C=0x5de4111afa1a4b94908f83103eb1f1706367c2e68ca870fc3fb9a804cdab365a
export PRIVATE_KEY_D=0x7c852118294e51e653712a81e05800f419141751be58f605c371e15141b007a6
export PRIVATE_KEY_E=0x47e179ec197488593b187f80a00eb0da91f1b9d0b13f8733639f19c30a34926a
export PRIVATE_KEY_F=0x8b3a350cf5c34c9194ca85829a2df0ec3153be0318b5e2d3348e872092edffba

export ADDRESS_A=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
export ADDRESS_B=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
export ADDRESS_C=0x3C44CdDdB6a900fa2b585dd299e03d12FA4293BC
export ADDRESS_D=0x90F79bf6EB2c4f870365E785982E1f101E93b906
export ADDRESS_E=0x15d34AAf54267DB7D7c367839AAf71A00a2C6A65
export ADDRESS_F=0x9965507D1a55bcC2695C58ba16FB37d819B0A4dc

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

# Convert 10,000 MOVE to the correct decimals (8 decimals 10,000 MOVE -> 1000000000000)
AMOUNT=1000000000000

cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_C $AMOUNT --rpc-url http://localhost:8545 > /dev/null
cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_D $AMOUNT --rpc-url http://localhost:8545 > /dev/null
cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_E $AMOUNT --rpc-url http://localhost:8545 > /dev/null
cast send --private-key $PRIVATE_KEY_B $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_F $AMOUNT --rpc-url http://localhost:8545 > /dev/null

echo "Transfers complete!"

# Check initial balances
echo "=== Initial Balances ==="
for letter in {A..F}; do
    echo -n "MOVE Balance of Address $letter: "
    cast call $MOVE_TOKEN "balanceOf(address)" $(eval echo \$ADDRESS_$letter) --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
done

# Make some test transfers
echo -e "\n=== Making test transfers ==="
# C transfers 100 MOVE to D
SENDAMOUNT=$((AMOUNT/10))
echo "C -> D: $(echo "scale=8; $SENDAMOUNT/100000000" | bc) MOVE"
cast send --private-key $PRIVATE_KEY_C $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_D $SENDAMOUNT --rpc-url http://localhost:8545 > /dev/null

# E transfers 50 MOVE to F
SENDAMOUNT=$((AMOUNT/100))
echo "E -> F: $(echo "scale=8; $SENDAMOUNT/100000000" | bc) MOVE"
cast send --private-key $PRIVATE_KEY_E $MOVE_TOKEN "transfer(address,uint256)" $ADDRESS_F $SENDAMOUNT --rpc-url http://localhost:8545 > /dev/null

# Check final balances
echo -e "\n=== Final Balances ==="
for letter in {A..F}; do
    echo -n "MOVE Balance of Address $letter: "
    cast call $MOVE_TOKEN "balanceOf(address)" $(eval echo \$ADDRESS_$letter) --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
done

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
# - - - - - - - Staking and Commitment Setup - - - - - - - - 
# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

echo -e "\n=== Staking and Commitment Setup for Address C ==="

# Debug checks before staking
echo "=== Debug Checks ==="
echo -n "MOVE token balance of Address C: "
cast call $MOVE_TOKEN "balanceOf(address)" $ADDRESS_C --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

echo -n "Current allowance for staking contract: "
cast call $MOVE_TOKEN "allowance(address,address)" $ADDRESS_C $MOVEMENT_STAKING --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

# Check commitment before posting (should show nothing)
echo "Check commitment status of Address C for height 1 before posting..."
./target/debug/ffs-dev mcr protocol client check-commitment \
    --height 1 \
    --mcr-address $MCR \
    --attester $ADDRESS_C \
    --private-key $PRIVATE_KEY_C

echo -n "... block height: "
cast block-number --rpc-url http://localhost:8545
sleep 2  # wait for async calls to complete
cast rpc anvil_mine --rpc-url http://localhost:8545 # Advance 1 block

# Check if commitment was accepted (should show nothing)
echo "Check if a commitment was accepted for height 1..."
./target/debug/ffs-dev mcr protocol client check-postconfirmation \
    --height 1 \
    --mcr-address $MCR \
    --private-key $PRIVATE_KEY_C

echo -n "... block height: "
cast block-number --rpc-url http://localhost:8545
sleep 2  # wait for async calls to complete
cast rpc anvil_mine --rpc-url http://localhost:8545 # Advance 1 block

# # Post commitment from C
# echo "Posting commitment from C..."
# ./target/debug/ffs-dev mcr protocol client post-commitment \
#     --preimage-string "commitment_from_C" \
#     --private-key $PRIVATE_KEY_C \
#     --height 1 \
#     --mcr-address $MCR

# echo -n "... block height: "
# cast block-number --rpc-url http://localhost:8545
# sleep 2  # wait for async calls to complete
# # Advance 2 blocks
# cast rpc anvil_mine --rpc-url http://localhost:8545 2

# # Check commitment after posting
# echo "Check commitment from C after posting..."
# ./target/debug/ffs-dev mcr protocol client check-commitment \
#     --height 1 \
#     --mcr-address $MCR \
#     --attester $ADDRESS_C \
#     --private-key $PRIVATE_KEY_C

# echo -n "... block height: "
# cast block-number --rpc-url http://localhost:8545
# sleep 2  # wait for async calls to complete
# # Advance 1 block
# cast rpc anvil_mine --rpc-url http://localhost:8545


# get the stake for address C and print it
echo -n "Stake for Address C: "
./target/debug/ffs-dev mcr protocol client get-stake \
    --custodian $ADDRESS_C \
    --attester $ADDRESS_C \
    --mcr-address $MCR \
    --rpc-url http://localhost:8545

# Stake using CLI command with explicit private key
echo "Staking MOVE tokens..."
# Approve staking contract to spend MOVE tokens
cast send --private-key $PRIVATE_KEY_C $MOVE_TOKEN "approve(address,uint256)" $MOVEMENT_STAKING 10000000 --rpc-url http://localhost:8545 > /dev/null
if [ $? -ne 0 ]; then
    echo "Error: Failed to approve MOVE tokens"
    exit 1
fi

./target/debug/ffs-dev mcr protocol client stake \
    --amount 0.1 \
    --private-key $PRIVATE_KEY_C \
    --mcr-address $MCR \
    --move-token-address $MOVE_TOKEN \
    --staking-address $MOVEMENT_STAKING
if [ $? -ne 0 ]; then
    echo "Error: Failed to stake tokens"
    exit 1
fi

echo -n "... block height: "
cast block-number --rpc-url http://localhost:8545
sleep 2  # wait for async calls to complete
cast rpc anvil_mine --rpc-url http://localhost:8545 # Advance 1 block

# Verify stake
echo -n "Staked amount for Address C: "
cast call $MOVEMENT_STAKING "getStake(address)" $ADDRESS_C --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc

# Post commitment using the CLI
echo -e "\n=== Posting Commitment ==="
echo "Posting commitment for Address C..."
./target/debug/ffs-dev mcr protocol client post-commitment \
    --preimage-string "commitment_from_C" \
    --private-key $PRIVATE_KEY_C \
    --height 1 \
    --mcr-address $MCR

echo -n "... block height: "
cast block-number --rpc-url http://localhost:8545
sleep 2  # wait for async calls to complete
cast rpc anvil_mine --rpc-url http://localhost:8545 # Advance 1 block


# Check attester's commitment after posting
echo "Checking attester's commitment after posting..."
./target/debug/ffs-dev mcr protocol client check-commitment \
    --height 1 \
    --mcr-address $MCR \
    --attester $ADDRESS_C \
    --private-key $PRIVATE_KEY_C

# Check if commitment was accepted
echo "Checking if commitment was accepted..."
./target/debug/ffs-dev mcr protocol client check-postconfirmation \
    --height 1 \
    --mcr-address $MCR \
    --private-key $PRIVATE_KEY_C

echo "Setup complete!"

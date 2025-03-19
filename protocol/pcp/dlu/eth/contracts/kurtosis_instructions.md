# Testing the PCP Contract with Kurtosis

This guide provides step-by-step instructions for setting up and testing the PCP contract using Kurtosis.

## 1. Prerequisites
- [Docker](https://docs.docker.com/get-docker/)
- [Kurtosis CLI](https://docs.kurtosis.com/install) 
- [Node.js](https://nodejs.org/) with `yarn`
- [Foundry](https://book.getfoundry.sh/) for Solidity development
- [Hardhat](https://hardhat.org/) for testing

## 2. Installation & Setup

### 2.1 Install Kurtosis
```sh
brew install kurtosis-tech/tap/kurtosis-cli
```

Start Docker and verify installation:
```sh
docker image ls
```

### 2.2 Install Development Dependencies
```sh
# Initialize Foundry
cd pcp/contracts
forge init --force --no-git

# Install dependencies
chmod +x script/install-deps.sh
./script/install-deps.sh

# Install npm dependencies
yarn install
```

## 3. Setting Up Local Ethereum Testnet

### 3.1 Start the Testnet
```sh
kurtosis --enclave local-eth-testnet run github.com/ethpandaops/ethereum-package
```
check: maybe the following command is better:
```sh
kurtosis run github.com/ethpandaops/ethereum-package
```

Expected output:
```sh
INFO[2023-04-04T18:09:44-04:00] ======================================================
INFO[2023-04-04T18:09:44-04:00] || Created enclave: local-eth-testnet ||
INFO[2023-04-04T18:09:44-04:00] ======================================================
```

To stop, run

```sh
kurtosis clean -a
```

### 3.2 Verify Testnet Status
```sh
kurtosis enclave inspect local-eth-testnet
# or
kurtosis enclave ls
```

Note the `rpc` port from the `el-1-geth-lighthouse` output (e.g., `127.0.0.1:49653`).

### 3.3 Note Default Funded Account

The pre-funded accounts can be found in the Kurtosis output when starting the network.
Look for the "pre_funded_accounts" section which contains addresses and private keys. The first address is

```json
{
    "address": "0x8943545177806ED17B9F23F0a21ee5948eCaa776",
    "private_key": "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31"
}
```

## 4. Deploying Contracts

There are two ways to deploy the contracts:

1. Using Hardhat (recommended)
2. Using Foundry (currently not recommended)

### 4.1 Option A: Development Deployment using `hardhat`

Try some basic tests first to make sure we can apply transactions

```sh
npx hardhat balance --network localnet --address 0x8943545177806ED17B9F23F0a21ee5948eCaa776
npx hardhat sendTx --network localnet --to 0x1234567890abcdef1234567890abcdef12345678 --amount 0.1
```

> ⚠️ **Note**: Version compatibility is important:
> - ethers: v6.0.0
> - @nomicfoundation/hardhat-ethers: v3.0.4
> - @typechain/ethers-v6: v0.5.0

Install all required dependencies:
```sh
yarn install

# If you see any missing dependency errors, run yarn install again
# This may happen as some dependencies have peer dependencies
```

Update the [hardhat.config.ts](hardhat.config.ts) file with your RPC port:
```ts
localnet: {
  url: 'http://127.0.0.1:<YOUR_RPC_PORT>',
...
```

Replace `<YOUR_RPC_PORT>` with the actual RPC port noted earlier and then test the connection:

```sh
npx hardhat accounts --network localnet
```

**Deploy Contracts**
```sh
npx hardhat clean
npx hardhat compile
npx hardhat run scripts/deploy.ts --network localnet
```

**Run Tests**
```sh
npx hardhat test --network localnet
```

**Verify the deployment**
```sh
npx hardhat run scripts/test-deployment.ts --network localnet
```

#### 4.2 Option B: Development Deployment (with `forge`)

> ⚠️ **Warning**: The forge deployment approach is currently not working reliably with Kurtosis.
> Development has been paused in favor of the Hardhat approach. The Foundry instructions below are kept for reference.

**Setup**
First, ensure Foundry is initialized. You may have to run the following.
```sh
cd protocol/pcp/contracts
forge init --force --no-git
```

Then install dependencies using the provided script:
```sh
chmod +x script/install-deps.sh
./script/install-deps.sh
```

It may be that this causes an error and requires some dependencies to be installed manually. (Remove this comment once this is resolved). Such as:
```sh
forge install safe-global/safe-smart-account --no-commit
forge install transmissions11/solmate --no-commit
```

Install npm dependencies using the provided package.json:
```sh
yarn install
```

**Deployment Files**
The following files are required for deployment and need to be updated when deploying a Kurtosis enclave:

1. The configuration settings are defined in `script/helpers/config.json`
2. The deployment addresses will be tracked in `script/helpers/deployments.json`. This file will be updated as contracts are deployed.

**Deployment**
```sh
forge script script/DeployPCPDev.s.sol:DeployPCPDev \
  --rpc-url http://127.0.0.1:<RPC_PORT> \
  --private-key <PREFUNDED_PRIVATE_KEY> \
  --broadcast
```

Run the automated verification script:

```sh
chmod +x script/verify-pcp.sh
./script/verify-pcp.sh
```

#### 4.3 Option C: Production Deployment (with `forge`)

Production-like Deployment (Optional, requires Safe contracts):

```sh
# Only attempt this after testing with the development deployment
# This deploys PCP using previously deployed Safe contracts
forge script script/DeployPCP.s.sol:DeployPCP \
  --rpc-url http://127.0.0.1:<YOUR_RPC_PORT> \
  --private-key <PREFUNDED_PRIVATE_KEY> \
  --broadcast
```

### 5 Additional Deployment Steps

### 5.1 Deploy Safe Contracts

The PCP deployment requires Safe contracts for governance. Deploy them using the provided script:

```sh
cd pcp/contracts
chmod +x script/deploy-safe.sh
./script/deploy-safe.sh http://127.0.0.1:<RPC_PORT> <PREFUNDED_PRIVATE_KEY>
```

This will deploy all required Safe contracts and save their addresses to `script/helpers/safe-deployments.json`.

Note: Keep track of the deployed addresses as they'll be needed for the PCP deployment.

### 5.2 Verify Deployment

```sh
chmod +x script/verify-pcp.sh
./script/verify-pcp.sh
```

## 5. Advanced Configuration (Optional)

### 5.1 Custom Network Parameters
Create `eth-network-params.json`:
```json
{
  "participants": [
    { "el_client_type": "geth", "cl_client_type": "lighthouse" }
  ],
  "network_params": {
    "network_id": "3151908",
    "seconds_per_slot": 12,
    "genesis_delay": 120
  }
}
```

### 5.2 Run with Custom Configuration
```sh
kurtosis clean -a && kurtosis run --enclave local-eth-testnet github.com/ethpandaops/ethereum-package --args-file eth-network-params.json
```

# Local Deployment with Anvil

This guide walks you through deploying contracts to an Anvil network using two accounts: one for **deploying** and one as the contract **admin**.

## Prerequisites

- [Foundry](https://book.getfoundry.sh/) installed
- `anvil` available in your path

## Setup Instructions

1. **Start Anvil**

```bash
anvil
```

2. **Set up signer variables**

Use the first Anvil account as the deployer (`signer_a`), and the second as the admin (`signer_b`):

```bash
export PRIVATE_KEY_A=<signer_a_private_key>
export PRIVATE_KEY_B=<signer_b_private_key>
export ADDRESS_A=<signer_a_address>
export ADDRESS_B=<signer_b_address>
```

3. **Deploy Contracts**

From the `contracts/` directory, run the deploy script using `signer_a`:

```bash
export PRIVATE_KEY=$PRIVATE_KEY_A
../anvil/scripts/deploy-dev.sh $ADDRESS_B
```

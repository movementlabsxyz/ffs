# Local Development with Anvil

This guide walks you through deploying contracts to an Anvil network using two accounts: one for **deploying** and one as the contract **admin**.

## Development environment

We develop in nix. This requires a few tools to be installed:

- [Foundry](https://book.getfoundry.sh/) installed
- `anvil` available in your path

Start by entering the nix shell:

```bash
nix develop
```

To build the CLI:

```bash
cargo clean
cargo build --release
## for development without optimizations but faster rebuilds, creates the binary in ./target/debug
cargo build
```

## Setup Instructions

**Start Anvil**

```bash
anvil
```

Use the first Anvil account as the deployer (`signer_a`), and the second as the admin (`signer_b`). The following values are the default values provided by Anvil. In production, you should never use these keys.

```bash
export PRIVATE_KEY_A=0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
export PRIVATE_KEY_B=0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d
export ADDRESS_A=0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266
export ADDRESS_B=0x70997970C51812dc3A010C7d01b50e0d17dc79C8
```

<!--Then start the local Anvil network:
```bash
./target/debug/ffs-dev mcr network coordinator eth anvil up
```
-->

**Deploy Contracts**
Deploy the contracts using `signer_a` as deployer and `signer_b` as admin:

```bash
./target/debug/ffs-dev pcp protocol client deploy anvil \
  --admin $ADDRESS_B \
  --private-key $PRIVATE_KEY_A
```

After deployment, you'll see the addresses of all deployed contracts. Set the MOVE token address (`moveTokenProxy`).

The default value for MCR is currently:
```bash
export MOVE_TOKEN=0xDc64a140Aa3E981100a9becA4E685f962f0cF6C9
```

The default value for PCP is:
```bash
export MOVE_TOKEN=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9
```

Then verify the deployment:

```bash
echo -n " Token Symbol: "
cast call $MOVE_TOKEN "symbol()" --rpc-url http://localhost:8545 | tr -d '\n' | cast --to-ascii
echo -n "Total Supply: "
cast call $MOVE_TOKEN "totalSupply()" --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
echo -n "Balance of admin: "
cast call $MOVE_TOKEN "balanceOf(address)" $ADDRESS_B --rpc-url http://localhost:8545 | cast --to-dec | xargs -I {} echo "scale=8; {}/100000000" | bc
```

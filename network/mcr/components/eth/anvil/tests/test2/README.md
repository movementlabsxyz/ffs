# Test2

This test is used to transfer MOVE tokens to a validator, stake, commit and postconfirm.

## Usage

```bash
cargo run --bin test-test2-mcr-network-anvil-component-core
```

You may have to kill the process as anvil does not exit cleanly.

Check with

```bash
ps aux | grep anvil
```

And kill the process with `pkill`.

```bash
pkill anvil
```

## Steps

1. Bring up an anvil network.
2. Bring up an MCR network.
3. Post a commitment on the MCR network.

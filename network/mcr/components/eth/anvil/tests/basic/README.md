# Basic

This test is used to bring up a basic network.

## Usage

```bash
cargo run --bin test-basic-mcr-network-anvil-component-core
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

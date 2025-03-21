# L1 contracts

This directory contains the implementation of the PCP settlement smart contract. To test the contract, run:

## Installation

```bash
chmod +x install-deps.sh
./install-deps.sh
```

## Build

Build and apply new abis

```bash
forge build
chmod +x apply-new-abis.sh
./apply-new-abis.sh
```

## Testing

After installing the dependencies, run

```bash
forge test
```

There is a long-running test covering over 50 epochs. It will likely take a few seconds to run.

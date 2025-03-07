# ffs
Movement Labs' Fast Finality Settlement is a proof of stake settlement system. 

## Organization
There are five subdirectories which progressively build on one another for node logic.

1. [`util`](./util): contains utility logic mainly reused in [`protocol-units`](./protocol-units).
2. [`protocol-units`](./protocol-units): contains implementations of the protocol logic. 
3. [`node`](./node): contains single-process runnable binaries that aggregate the protocol logic.
4. [`network`](./network): contains logic for running multiple nodes in a network.
5. [`sdk`](./sdk): contains logic for interacting nodes and networks.

There are several other subdirectories of note:

- [`spec`](./spec): contains formal verification of FFS protocols. 
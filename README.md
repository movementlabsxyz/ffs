# ffs

Movement Labs' Fast Finality Settlement is a Proof-of-Stake (PoS) settlement system.

The directory is structured as follows.

- [Node](./node/README.md)
is an independent service which is prepared to manage a fast-finality settlement connection for a given user.

- [Protocol-units](./protocol-units/README.md)
are used to collect runnable and composable units for interacting with the `ffs` protocol. We store contracts within protocol units, for example.

- [FFS SDK](./sdk/README.md)
is a set of tools for building FFS services. It is generally the entrypoint for researching and building with FFS.

- [Spec](./spec/README.md)
contains formalizations of the FFS protocol and its components.

- [Util](./util/README.md)
collects miscellaneous software utilities maintained by the FFS team.

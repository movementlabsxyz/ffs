# MCR (Multi Commit Rollup)

This directory contains the implementation of the MCR protocol, which handles rollup commitment aggregation.

## Overview

**MCR** implements a staking-based settlement where validators commit to a state from L2 on Layer 1 (L1). MCR accepts commitments from all staked attesters and aggregates them into a single confirmation. The contracts on L1 tracks block commitments, epochs, stake and rewards.

We use the following terminology:

- `commitment` - a commitment to a state from L2 on L1.
- `block` - a wrapper around the commitment, which also includes the `block` height on L1. This is neither the same as an L2 block, nor the same as an L1 block. In the MIPs this is called a `superblock`.
- `block_commitment` - same as `block`, to add a bit of confusion.
- `epoch` - a period of time that fixes the set of attesters for that interval.

The distinguishing feature to PCP is that there is only one type of actor, which is the attester.

For further details see the [RFC for MCR](https://github.com/movementlabsxyz/rfcs/pull/29) and the [MIP-34](https://github.com/movementlabsxyz/MIP/blob/main/MIP/mip-34).

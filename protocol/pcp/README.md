# PCP (Postconfirmation Protocol)

This directory contains the implementation of the PCP protocol, which handles commitment settlement through postconfirmation.

## Overview

**PCP** implements a staking-based settlement where validators commit to a state from L2 on Layer 1 (L1). PCP accepts commitments from all staked attesters. A specialized attester - the acceptor - aggregates them into a single confirmation, called a postconfirmation. The contracts on L1 tracks block commitments, epochs, stake and rewards.

The distinguishing feature to MCR is that there are two types of actors, which are the attesters and the acceptors.

For further details see the [MIP-37](https://github.com/movementlabsxyz/MIP/blob/main/MIP/mip-37).

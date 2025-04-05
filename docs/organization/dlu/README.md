# Deployed Logic Units (DLU)

DLUs are units of logic that are not generally run within an independent process, but which must be deployed somewhere. Smart contracts are DLU.

In this directory, we include DLU and the means via which to deploy them to various environments.

> ![NOTE]
> Deployed Logic Units are considered part of the protocol. Insofar as the protocol needs particular logic deployed on a particular compute device to run, this logic and the deployment thereof cannot be excluded from the protocol.
> 
> In contrast, logic for deployments in the [`network`](../../../network/) directory handle the realization of a network given the deployment procedures defined herein or given generic deployment procedures that are not specific to the protocol, e.g., starting a node on AWS and opening ports for peer discovery. 
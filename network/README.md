# Network
The network directory is used for storing logic that allows for the provisioning of MCR networks. This includes local networks, remote networks and their IaC, and all supported variations.

The role of a "Coordinator" in network logic is to create an abstract which renders the running of an entire network as single process. In cases where the network is remote--or even simply multi-process--this means that the "Coordinator" primarily functions to aggregate [lifecycles](https://github.com/movementlabsxyz/adud) and monitoring functionality. 

The original intent was to have network primarily compose processes from the [Node](../node) level of abstraction. It is also common, however, for processes and logic to be elevated directly from [Protocol](../protocol/).
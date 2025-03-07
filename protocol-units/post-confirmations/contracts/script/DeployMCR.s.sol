// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/settlement/MCR.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployMCR is Script {
    function run() public {
        // Load Safe addresses from deployments.json
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/script/helpers/safe-deployments.json");
        string memory json = vm.readFile(path);
        
        address safe = abi.decode(vm.parseJson(json, ".Safe"), (address));
        address handler = abi.decode(vm.parseJson(json, ".FallbackHandler"), (address));
        address factory = abi.decode(vm.parseJson(json, ".SafeFactory"), (address));

        vm.startBroadcast();

        // Deploy MCR implementation and proxy
        MCR mcrImplementation = new MCR();
        
        // Get MOVE token and staking addresses from deployments
        string memory deploymentsPath = string.concat(root, "/script/helpers/deployments.json");
        string memory deploymentsJson = vm.readFile(deploymentsPath);
        address moveToken = abi.decode(vm.parseJson(deploymentsJson, ".3151908.move"), (address));
        address staking = abi.decode(vm.parseJson(deploymentsJson, ".3151908.staking"), (address));

        // Initialize MCR with production settings
        address[] memory custodians = new address[](1);
        custodians[0] = moveToken;  // The MOVE token is the custodian for rewards

        bytes memory mcrData = abi.encodeCall(
            MCR.initialize, 
            (
                IMovementStaking(staking),  // _stakingContract: address of staking contract
                0,                          // _lastPostconfirmedSuperBlockHeight: start from genesis
                5,                          // _leadingSuperBlockTolerance: max blocks ahead of last confirmed
                20 seconds,                 // _epochDuration: how long an epoch lasts
                custodians,                 // _custodians: array with moveToken address for rewards
                10 seconds,                 // _postconfirmerDuration: how long a postconfirmer serves
                moveToken                   // _moveTokenAddress: primary custodian for rewards in staking
            )
        );
        address mcrProxy = address(new ERC1967Proxy(address(mcrImplementation), mcrData));

        // Save MCR address to deployments
        console.log("MCR implementation deployed to:", address(mcrImplementation));
        console.log("MCR proxy deployed to:", mcrProxy);

        vm.stopBroadcast();
    }
} 
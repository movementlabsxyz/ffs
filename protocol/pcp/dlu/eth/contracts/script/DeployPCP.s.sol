// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/settlement/PCP.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployPCP is Script {
    function run() public {
        // Load Safe addresses from deployments.json
        string memory root = vm.projectRoot();
        string memory path = string.concat(root, "/script/helpers/safe-deployments.json");
        string memory json = vm.readFile(path);
        
        address safe = abi.decode(vm.parseJson(json, ".Safe"), (address));
        address handler = abi.decode(vm.parseJson(json, ".FallbackHandler"), (address));
        address factory = abi.decode(vm.parseJson(json, ".SafeFactory"), (address));

        vm.startBroadcast();

        // Deploy PCP implementation and proxy
        PCP pcpImplementation = new PCP();
        
        // Get MOVE token and staking addresses from deployments
        string memory deploymentsPath = string.concat(root, "/script/helpers/deployments.json");
        string memory deploymentsJson = vm.readFile(deploymentsPath);
        address moveToken = abi.decode(vm.parseJson(deploymentsJson, ".3151908.move"), (address));
        address staking = abi.decode(vm.parseJson(deploymentsJson, ".3151908.staking"), (address));

        // Initialize PCP with production settings
        address[] memory custodians = new address[](1);
        custodians[0] = moveToken;  // The MOVE token is the custodian for rewards

        bytes memory pcpData = abi.encodeCall(
            PCP.initialize, 
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
        address pcpProxy = address(new ERC1967Proxy(address(pcpImplementation), pcpData));

        // Save PCP address to deployments
        console.log("PCP implementation deployed to:", address(pcpImplementation));
        console.log("PCP proxy deployed to:", pcpProxy);

        vm.stopBroadcast();
    }
} 
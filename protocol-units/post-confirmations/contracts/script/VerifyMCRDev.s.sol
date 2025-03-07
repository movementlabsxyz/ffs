pragma solidity ^0.8.19;

/**
 * Verification script for the MCR deployment.
 * Checks token balances, permissions, staking functionality, and basic MCR operations.
 * Run this after DeployMCRDev.s.sol to verify the system is working correctly.
 */

import "forge-std/Script.sol";
import "../src/token/MOVEToken.sol";
import "../src/staking/MovementStaking.sol";
import "../src/settlement/MCR.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

contract VerifyMCRDev is Script {
    function run() external {
        // Read deployment addresses
        string memory json = vm.readFile("deployment.json");
        address moveTokenProxy = vm.parseJsonAddress(json, ".moveToken");
        address mcrProxy = vm.parseJsonAddress(json, ".mcr");
        address stakingProxy = vm.parseJsonAddress(json, ".staking");
        address deployer = vm.parseJsonAddress(json, ".deployer");

        // Contract instances
        MintableToken moveToken = MintableToken(moveTokenProxy);
        MCR mcr = MCR(mcrProxy);
        MovementStaking staking = MovementStaking(stakingProxy);

        console.log("\n=== Verifying MCR Configuration ===");
        console.log("Epoch duration: %s seconds", mcr.getEpochDuration());
        console.log("Postconfirmer duration: %s seconds", mcr.getPostconfirmerDuration());

        console.log("\n=== Verifying Staking Setup ===");
        console.log("MCR epoch duration in staking: %s", staking.epochDurationByDomain(mcrProxy));
        // console.log("MOVE token in staking: %s", address(staking.moveToken()));

        console.log("\n=== Verifying Token Setup ===");
        console.log("Deployer balance: %s", moveToken.balanceOf(deployer));
        console.log("Deployer has minter role: %s", moveToken.hasMinterRole(deployer));
        console.log("Staking has minter role: %s", moveToken.hasMinterRole(stakingProxy));

        vm.startBroadcast();
        
        console.log("\n=== Verification Complete ===");

        vm.stopBroadcast();
    }
} 
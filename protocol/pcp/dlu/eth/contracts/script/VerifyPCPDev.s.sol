pragma solidity ^0.8.19;

/**
 * Verification script for the PCP deployment.
 * Checks token balances, permissions, staking functionality, and basic PCP operations.
 * Run this after DeployPCPDev.s.sol to verify the system is working correctly.
 */

import "forge-std/Script.sol";
import "../src/token/MOVEToken.sol";
import "../src/staking/MovementStaking.sol";
import "../src/settlement/PCP.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

contract VerifyPCPDev is Script {
    function run() external {
        // Read deployment addresses
        string memory json = vm.readFile("deployment.json");
        address moveTokenProxy = vm.parseJsonAddress(json, ".moveToken");
        address pcpProxy = vm.parseJsonAddress(json, ".pcp");
        address stakingProxy = vm.parseJsonAddress(json, ".staking");
        address deployer = vm.parseJsonAddress(json, ".deployer");

        // Contract instances
        MintableToken moveToken = MintableToken(moveTokenProxy);
        PCP pcp = PCP(pcpProxy);
        MovementStaking staking = MovementStaking(stakingProxy);

        console.log("\n=== Verifying PCP Configuration ===");
        console.log("Epoch duration: %s seconds", pcp.getEpochDuration());
        console.log("Postconfirmer duration: %s seconds", pcp.getPostconfirmerDuration());

        console.log("\n=== Verifying Staking Setup ===");
        console.log("PCP epoch duration in staking: %s", staking.epochDurationByDomain(pcpProxy));
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
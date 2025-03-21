// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/token/MOVEToken.sol";
import "../src/staking/MovementStaking.sol";
import "../src/settlement/MCR.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

contract DeployMCRDev is Script {
    function run(address contractAdmin) external {
        vm.startBroadcast();
        vm.recordLogs();

        // Deploy Proxy Admin
        ProxyAdmin proxyAdmin = new ProxyAdmin(msg.sender);
        console.log("JSONL proxy_admin = %s", address(proxyAdmin));

        // Deploy Implementations
        MintableToken moveTokenImplementation = new MintableToken();
        console.log("JSONL move_token_implementation = %s", address(moveTokenImplementation));
        
        MovementStaking stakingImplementation = new MovementStaking();
        console.log("JSONL staking_implementation = %s", address(stakingImplementation));
        
        MCR mcrImplementation = new MCR();
        console.log("JSONL mcr_implementation = %s", address(mcrImplementation));

        // Deploy the Move Token Proxy
        bytes memory moveTokenData = abi.encodeCall(MintableToken.initialize, ("Move Token", "MOVE"));
        TransparentUpgradeableProxy moveTokenProxy = new TransparentUpgradeableProxy(
            address(moveTokenImplementation), address(proxyAdmin), moveTokenData
        );
        console.log("JSONL move_token_proxy = %s", address(moveTokenProxy));

        // Deploy the Movement Staking Proxy
        bytes memory movementStakingData =
            abi.encodeCall(MovementStaking.initialize, IMintableToken(address(moveTokenProxy)));
        TransparentUpgradeableProxy movementStakingProxy = new TransparentUpgradeableProxy(
            address(stakingImplementation), address(proxyAdmin), movementStakingData
        );
        console.log("JSONL movement_staking_proxy = %s", address(movementStakingProxy));

        // Deploy the MCR Proxy
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveTokenProxy);
        bytes memory mcrData = abi.encodeCall(
            MCR.initialize, (IMovementStaking(address(movementStakingProxy)), 0, 10, 4 seconds, custodians)
        );
        TransparentUpgradeableProxy mcrProxy = new TransparentUpgradeableProxy(
            address(mcrImplementation), address(proxyAdmin), mcrData
        );
        console.log("JSONL mcr_proxy = %s", address(mcrProxy));

        // Grant commitment admin
        MCR mcr = MCR(address(mcrProxy));
        mcr.grantCommitmentAdmin(contractAdmin);
        mcr.grantCommitmentAdmin(msg.sender);
        console.log("JSONL granted_commitment_admin = %s", contractAdmin);

        // Verify custodian setup
        uint256 custodianEpochDuration = MovementStaking(address(movementStakingProxy)).epochDurationByDomain(address(mcrProxy));
        console.log("JSONL mcr_custodian_epoch_duration = %d", custodianEpochDuration);

        // Mint tokens
        MintableToken moveToken = MintableToken(address(moveTokenProxy));
        moveToken.mint(contractAdmin, 100000 ether);
        console.log("JSONL minted_tokens = { \"recipient\" : \"%s\", \"amount\" : \"100000 ether\" }", contractAdmin);

        // Grant minter role
        moveToken.grantMinterRole(msg.sender);
        moveToken.grantMinterRole(contractAdmin);
        console.log("JSONL granted_minter_role = %s", contractAdmin);

        moveToken.grantMinterRole(address(movementStakingProxy));
        // console.log("JSONL granted_minter_role = %s", address(movementStakingProxy));

        vm.stopBroadcast();
    }
}

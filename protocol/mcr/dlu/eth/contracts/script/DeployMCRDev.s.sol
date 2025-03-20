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
        
        console.log("hot: msg.sender: %s", msg.sender);

        // Deploy Proxy Admin
        ProxyAdmin proxyAdmin = new ProxyAdmin(msg.sender);
        console.log("Proxy Admin Deployed: %s", address(proxyAdmin));

        // Deploy Implementations
        MintableToken moveTokenImplementation = new MintableToken();
        console.log("Move Token Implementation Deployed: %s", address(moveTokenImplementation));
        MovementStaking stakingImplementation = new MovementStaking();
        console.log("Movement Staking Implementation Deployed: %s", address(stakingImplementation));
        MCR mcrImplementation = new MCR();
        console.log("MCR Implementation Deployed: %s", address(mcrImplementation));

        // Deploy the Move Token Proxy
        bytes memory moveTokenData = abi.encodeCall(MintableToken.initialize, ("Move Token", "MOVE"));
        TransparentUpgradeableProxy moveTokenProxy = new TransparentUpgradeableProxy(
            address(moveTokenImplementation), address(proxyAdmin), moveTokenData
        );
        console.log("Move Token Proxy Deployed: %s", address(moveTokenProxy));

        // Deploy the Movement Staking Proxy
        bytes memory movementStakingData =
            abi.encodeCall(MovementStaking.initialize, IMintableToken(address(moveTokenProxy)));
        TransparentUpgradeableProxy movementStakingProxy = new TransparentUpgradeableProxy(
            address(stakingImplementation), address(proxyAdmin), movementStakingData
        );
        console.log("Movement Staking Proxy Deployed: %s", address(movementStakingProxy));

        // Deploy the MCR Proxy
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveTokenProxy);
        bytes memory mcrData = abi.encodeCall(
            MCR.initialize, (IMovementStaking(address(movementStakingProxy)), 0, 10, 4 seconds, custodians)
        );
        TransparentUpgradeableProxy mcrProxy = new TransparentUpgradeableProxy(
            address(mcrImplementation), address(proxyAdmin), mcrData
        );
        console.log("MCR Proxy Deployed: %s", address(mcrProxy));

        // Grant commitment admin
        MCR mcr = MCR(address(mcrProxy));
        mcr.grantCommitmentAdmin(contractAdmin);
        console.log("Granted CommitmentAdmin role to: %s", contractAdmin);
        mcr.grantCommitmentAdmin(msg.sender);
        console.log("Granted CommitmentAdmin role to: %s", msg.sender);

        // Verify custodian setup
        console.log("MCR custodian: %s", MovementStaking(address(movementStakingProxy)).epochDurationByDomain(address(mcrProxy)));

        // Mint tokens
        MintableToken moveToken = MintableToken(address(moveTokenProxy));
        moveToken.mint(contractAdmin, 100000 ether);
        console.log("Minted 100000 MOVE to %s", contractAdmin);

        // Grant minter role
        moveToken.grantMinterRole(msg.sender);
        console.log("Granted Minter Role to: %s", msg.sender);
        moveToken.grantMinterRole(contractAdmin);
        console.log("Granted Minter Role to: %s", contractAdmin);
        moveToken.grantMinterRole(address(movementStakingProxy));
        console.log("Granted Minter Role to: %s", address(movementStakingProxy));

        vm.stopBroadcast();
    }
}
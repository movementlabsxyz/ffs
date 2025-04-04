pragma solidity ^0.8.19;

/**
 * Development deployment script for the Multi-Commit-Rollup (PCP) system.
 * This deploys a test environment with short epochs and quick postconfirmer rotations.
 * Includes MOVE token for staking, Movement Staking for managing attesters, and PCP for cross-chain settlement.
 */

import "forge-std/Script.sol";
import "../src/token/MOVEToken.sol";
import "../src/staking/MovementStaking.sol";
import "../src/settlement/PCP.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ERC1967Proxy} from "@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol";

contract DeployPCPDev is Script {
    function run() external {
        vm.startBroadcast();

        console.log("hot: msg.sender: %s", msg.sender);

        // Deploy the implementation contracts that will be delegated to
        MintableToken moveTokenImplementation = new MintableToken();
        MovementStaking stakingImplementation = new MovementStaking();
        PCP pcpImplementation = new PCP();

        // Deploy MOVE token behind proxy and initialize with name and symbol
        bytes memory moveTokenData = abi.encodeCall(
            MintableToken.initialize,
            (
                "Move Token",  // name: Token name for display
                "MOVE"        // symbol: Token symbol for markets/exchanges
            )
        );
        address moveTokenProxy = address(
            new ERC1967Proxy(address(moveTokenImplementation), moveTokenData)
        );

        // Deploy staking contract behind proxy, using MOVE token for rewards
        bytes memory movementStakingData = abi.encodeCall(
            MovementStaking.initialize,
            IMintableToken(address(moveTokenProxy))  // moveToken: Token used for staking and rewards
        );
        address movementStakingProxy = address(
            new ERC1967Proxy(address(stakingImplementation), movementStakingData)
        );

        // Set up PCP with MOVE token as the only custodian for rewards
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveTokenProxy);

        // Deploy PCP behind proxy with test configuration
        bytes memory pcpData = abi.encodeCall(
            PCP.initialize,
            (
                IMovementStaking(address(movementStakingProxy)),  // stakingContract: Contract managing attesters' stakes
                0,                                                // lastPostconfirmedCommitmentHeight: Start from genesis
                5,                                                // leadingCommitmentTolerance: Max blocks ahead of last confirmed
                10 seconds,                                       // epochDuration: How long each epoch lasts (short for testing)
                custodians,                                      // custodians: Array of tokens used for rewards [MOVE]
                5 seconds,                                       // postconfirmerDuration: How long postconfirmer serves
                address(moveTokenProxy)                          // moveTokenAddress: Primary token for staking rewards
            )
        );
        address pcpProxy = address(new ERC1967Proxy(address(pcpImplementation), pcpData));

        // Set up roles and permissions
        PCP pcp = PCP(pcpProxy);
        pcp.grantCommitmentAdmin(msg.sender);

        // Log the deployed addresses
        console.log("Move Token Proxy: %s", moveTokenProxy);
        console.log("PCP Proxy: %s", pcpProxy);
        console.log("PCP custodian: %s", MovementStaking(movementStakingProxy).epochDurationByDomain(pcpProxy));

        // Log initial state
        console.log("\n=== Initial Setup ===");
        console.log("Deployer address: %s", msg.sender);
        console.log("Move Token Proxy: %s", moveTokenProxy);
        console.log("PCP Proxy: %s", pcpProxy);
        console.log("Staking Proxy: %s", movementStakingProxy);

        // Set up initial token distribution and permissions
        MintableToken moveToken = MintableToken(moveTokenProxy);
        
        // Log roles before minting
        console.log("\n=== Roles ===");
        console.log("Deployer has minter role: %s", moveToken.hasMinterRole(msg.sender));
        console.log("Staking has minter role: %s", moveToken.hasMinterRole(address(movementStakingProxy)));

        // Log balances before minting
        console.log("\n=== Balances Before Mint ===");
        // log if the deployer has enough Eth to pay for the deployment
        uint256 ethbalance = address(msg.sender).balance;
        console.log("Deployer has balance (ETH): %s", ethbalance);
        console.log("Deployer balance (MOVE): %s", moveToken.balanceOf(msg.sender));
        console.log("Staking balance (MOVE): %s", moveToken.balanceOf(address(movementStakingProxy)));

        // Mint and grant roles
        moveToken.mint(msg.sender, 100000 ether);           // Mint initial tokens to deployer
        moveToken.grantMinterRole(msg.sender);              // Allow deployer to mint
        moveToken.grantMinterRole(address(movementStakingProxy));  // Allow staking to mint rewards

        // Log final state
        console.log("\n=== Final State ===");
        console.log("Deployer balance after mint (MOVE): %s", moveToken.balanceOf(msg.sender));
        console.log("Deployer balance after mint (ETH): %s", address(msg.sender).balance);
        console.log("Deployer has spend (ETH): %s", ethbalance - address(msg.sender).balance);
        console.log("Deployer has minter role: %s", moveToken.hasMinterRole(msg.sender));
        console.log("Staking has minter role: %s", moveToken.hasMinterRole(address(movementStakingProxy)));

        // Verify deployment
        console.log("\n=== Verifying Deployment ===");
        
        // Verify PCP configuration
        console.log("PCP Configuration:");
        MovementStaking staking = MovementStaking(movementStakingProxy);
        uint256 epochDuration = staking.getEpochDuration(pcpProxy);
        uint256 postconfirmerDuration = pcp.getPostconfirmerDuration();
        console.log("- Epoch Duration: %s seconds", epochDuration);
        console.log("- Postconfirmer Duration: %s seconds", postconfirmerDuration);
        require(epochDuration == 10, "Incorrect epoch duration");
        require(postconfirmerDuration == 5, "Incorrect postconfirmer duration");
        
        // Verify Staking configuration
        console.log("\nStaking Configuration:");
        uint256 stakingEpochDuration = MovementStaking(movementStakingProxy).epochDurationByDomain(pcpProxy);
        console.log("- Epoch Duration for PCP domain: %s seconds", stakingEpochDuration);
        require(stakingEpochDuration == 10, "Incorrect staking epoch duration");

        // Some simple sanity checks
        console.log("\ngetAcceptingEpoch(pcpProxy): %s", staking.getAcceptingEpoch(pcpProxy));
        console.log("\ngetLastPostconfirmedCommitmentHeight(): %s", pcp.getLastPostconfirmedCommitmentHeight());
        console.log("\nList of active attesters:");
        address[] memory stakedAttesters = staking.getStakedAttestersForAcceptingEpoch(pcpProxy);
        if (stakedAttesters.length > 0) {
            for (uint256 i = 0; i < stakedAttesters.length; i++) {
                console.log("- Attester %s: %s", i, stakedAttesters[i]);
            }
        } else {
            console.log("No attesters staked");
        }
        console.log("\nPostconfirmer: %s", pcp.getPostconfirmer());
        
        // Verify token setup
        console.log("\nToken Configuration:");
        uint256 deployerBalance = moveToken.balanceOf(msg.sender);
        console.log("- Deployer Balance: %s", deployerBalance);
        require(deployerBalance == 100000 ether, "Incorrect deployer balance");
        require(moveToken.hasMinterRole(msg.sender), "Deployer missing minter role");
        require(moveToken.hasMinterRole(address(movementStakingProxy)), "Staking missing minter role");
        
        console.log("\nDeployment verified successfully!");

        vm.stopBroadcast();
    }
}

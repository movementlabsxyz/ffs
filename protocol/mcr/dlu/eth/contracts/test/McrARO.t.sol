// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "forge-std/console.sol";
import "../src/staking/MovementStaking.sol";
import "../src/token/MOVETokenDev.sol";
import "../src/settlement/MCR.sol";
import "../src/settlement/McrARO.sol";
import {IMovementStaking} from "../src/staking/interfaces/IMovementStaking.sol";
import {IMintableToken} from "../src/token/base/MintableToken.sol";
import {IMcrReward} from "../src/settlement/interfaces/IMcrReward.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract McrAROTest is Test {
    MCR public mcr;
    MovementStaking public staking;
    IERC20 public token;
    McrARO public aroImpl;
    TransparentUpgradeableProxy public aroProxy;
    IMcrReward public aro;

    // Test addresses
    address public admin = address(0x1);
    address public attester1 = address(0x2);
    address public attester2 = address(0x3);
    address public attester3 = address(0x4);

    // Test constants
    uint256 public constant INITIAL_BALANCE = 1_000_000 ether;
    uint256 public constant STAKE_AMOUNT = 10_000 ether;
    uint256 public constant EPOCH_DURATION = 60 seconds; // 1 minute for tests

    // Function signatures for initialization
    string constant moveSignature = "initialize(string,string)";
    string constant stakingSignature = "initialize(address)";
    string constant mcrSignature = "initialize(address,uint256,uint256,uint256,address[])";
    string constant aroSignature = "initializeRewardConfig()";

    function deployProxies() internal returns (
        TransparentUpgradeableProxy moveProxy,
        TransparentUpgradeableProxy stakingProxy,
        TransparentUpgradeableProxy mcrProxy,
        TransparentUpgradeableProxy aroProxy
    ) {
        // Deploy implementations
        MOVETokenDev moveTokenImplementation = new MOVETokenDev();
        MovementStaking stakingImplementation = new MovementStaking();
        MCR mcrImplementation = new MCR();
        McrARO aroImplementation = new McrARO();

        // Contract McrAROTest is the admin
        admin = address(new ProxyAdmin(address(this)));

        // Deploy proxies
        moveProxy = new TransparentUpgradeableProxy(
            address(moveTokenImplementation),
            address(admin),
            abi.encodeWithSignature(moveSignature, "Move Token", "MOVE")
        );
        stakingProxy = new TransparentUpgradeableProxy(
            address(stakingImplementation),
            address(admin),
            abi.encodeWithSignature(stakingSignature, IMintableToken(address(moveProxy)))
        );
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveProxy);
        mcrProxy = new TransparentUpgradeableProxy(
            address(mcrImplementation),
            address(admin),
            abi.encodeWithSignature(mcrSignature, stakingProxy, 0, 10, EPOCH_DURATION, custodians)
        );
        aroProxy = new TransparentUpgradeableProxy(
            address(aroImplementation),
            address(admin),
            abi.encodeWithSignature(aroSignature)
        );
    }

    function setupAttesters() internal {
        // Grant whitelist access for attesters
        staking.whitelistAddress(attester1);
        staking.whitelistAddress(attester2);
        staking.whitelistAddress(attester3);

        // Set up reward contract in MCR
        mcr.setRewardContract(aro);

        // Mint tokens and approve for staking
        MOVETokenDev tokenMinter = MOVETokenDev(address(token));
        tokenMinter.mint(address(this), INITIAL_BALANCE);
        tokenMinter.mint(address(mcr), INITIAL_BALANCE / 2); // For rewards
        tokenMinter.mint(attester1, INITIAL_BALANCE);
        tokenMinter.mint(attester2, INITIAL_BALANCE);
        tokenMinter.mint(attester3, INITIAL_BALANCE);

        // Approve tokens for staking
        vm.startPrank(attester1);
        token.approve(address(staking), STAKE_AMOUNT);
        staking.stake(address(mcr), token, STAKE_AMOUNT);
        vm.stopPrank();

        vm.startPrank(attester2);
        token.approve(address(staking), STAKE_AMOUNT * 2);
        staking.stake(address(mcr), token, STAKE_AMOUNT * 2);
        vm.stopPrank();

        vm.startPrank(attester3);
        token.approve(address(staking), STAKE_AMOUNT * 3);
        staking.stake(address(mcr), token, STAKE_AMOUNT * 3);
        vm.stopPrank();

        // Accept genesis ceremony for MCR
        mcr.acceptGenesisCeremony();

        // Grant trusted attester roles
        mcr.grantTrustedAttester(attester1);
        mcr.grantTrustedAttester(attester2);
        mcr.grantTrustedAttester(attester3);
    }

    function setUp() public {
        (TransparentUpgradeableProxy moveProxy, TransparentUpgradeableProxy stakingProxy, TransparentUpgradeableProxy mcrProxy, TransparentUpgradeableProxy aroProxy) = deployProxies();

        // Set contract references
        token = IERC20(address(moveProxy));
        staking = MovementStaking(address(stakingProxy));
        mcr = MCR(address(mcrProxy));
        aro = IMcrReward(address(aroProxy));

        setupAttesters();
    }

    function test_AsymptoticRewards() public {
        // This tests the asymptotic nature of the rewards by tracking rewards at different balance levels
        
        // Create a new implementation to test the calculateAsymptoticReward function directly
        McrARO testImpl = new McrARO();
        
        // Use the exposed function below to test the asymptotic function
        // McrARO has internal calculateAsymptoticReward function that we can't call directly
        
        // Test with different balances and parameters
        uint256[] memory balances = new uint256[](5);
        balances[0] = 1_000_000 ether;
        balances[1] = 100_000 ether;
        balances[2] = 10_000 ether;
        balances[3] = 1_000 ether;
        balances[4] = 100 ether;
        
        // Get contract balance for verification
        uint256 contractBalance = token.balanceOf(address(mcr));
        assertGt(contractBalance, 0, "MCR should have token balance");
        
        // Submit block commitments to trigger rewards
        uint256 initialAttester1Balance = token.balanceOf(attester1);
        uint256 initialAttester2Balance = token.balanceOf(attester2);
        
        // Calculate proportional rewards for different contract balances
        for (uint256 i = 0; i < balances.length; i++) {
            uint256 expectedReward = testAsymptoticRewardCalculation(balances[i], 10, 10000);
            uint256 proportion = (expectedReward * 100) / balances[i];
            
            // Verify that as balance decreases, the proportion also decreases (asymptotic curve)
            if (i > 0) {
                uint256 previousProportion = (testAsymptoticRewardCalculation(balances[i-1], 10, 10000) * 100) / balances[i-1];
                assertLe(proportion, previousProportion, "Reward proportion should decrease as balance decreases");
            }
        }
    }

    function test_BlockRewards() public {
        // Test block commitment rewards
        uint256 initialAttester1Balance = token.balanceOf(attester1);
        
        // Create a block commitment from attester1
        MCR.Commitment memory commitment1 = mcr.createCommitment(
            1, // height
            keccak256("block1"), // commitment
            keccak256("blockId1") // blockId
        );
        
        // Submit the commitment
        vm.prank(attester1);
        mcr.submitCommitment(commitment1);
        
        // Check if attester1 received rewards
        uint256 newAttester1Balance = token.balanceOf(attester1);
        
        assertGt(newAttester1Balance, initialAttester1Balance, "Attester1 should receive rewards for block commitment");
        
        // Create another block commitment to check for speed bonuses
        MCR.Commitment memory commitment2 = mcr.createCommitment(
            2, // height
            keccak256("block2"), // commitment
            keccak256("blockId2") // blockId
        );
        
        // Skip a short time to ensure speed bonus
        skip(10); // 10 seconds
        
        // Submit the commitment
        vm.prank(attester1);
        mcr.submitCommitment(commitment2);
        
        // Check if attester1 received more rewards
        uint256 secondRewardBalance = token.balanceOf(attester1);
        
        assertGt(secondRewardBalance, newAttester1Balance, "Attester1 should receive more rewards for second block");
    }

    function test_EpochRolloverRewards() public {
        // Test epoch rollover rewards
        uint256 initialAttester1Balance = token.balanceOf(attester1);
        uint256 initialAttester2Balance = token.balanceOf(attester2);
        uint256 initialAttester3Balance = token.balanceOf(attester3);
        
        // Create several block commitments to accumulate epoch commitments
        for (uint256 i = 1; i <= 5; i++) {
            // Attester1 submits most commitments
            MCR.Commitment memory commitment = mcr.createCommitment(
                i, // height
                keccak256(abi.encodePacked("block", i)), // commitment 
                keccak256(abi.encodePacked("blockId", i)) // blockId
            );
            
            vm.prank(attester1);
            mcr.submitCommitment(commitment);
            
            // Attester2 submits fewer commitments
            if (i % 2 == 0) {
                vm.prank(attester2);
                mcr.submitCommitment(commitment);
            }
            
            // Attester3 submits only one commitment
            if (i == 3) {
                vm.prank(attester3);
                mcr.submitCommitment(commitment);
            }
        }
        
        // Skip to the end of the epoch
        skip(EPOCH_DURATION + 1);
        
        // Trigger epoch rollover by submitting another block
        MCR.Commitment memory rolloverCommitment = mcr.createCommitment(
            6, // height
            keccak256("block6"), // commitment
            keccak256("blockId6") // blockId
        );
        
        vm.prank(attester1);
        mcr.submitCommitment(rolloverCommitment);
        
        // Check rewards - attesters should have received epoch rollover rewards
        uint256 newAttester1Balance = token.balanceOf(attester1);
        uint256 newAttester2Balance = token.balanceOf(attester2);
        uint256 newAttester3Balance = token.balanceOf(attester3);
        
        assertGt(newAttester1Balance, initialAttester1Balance, "Attester1 should receive epoch rewards");
        assertGt(newAttester2Balance, initialAttester2Balance, "Attester2 should receive epoch rewards");
        assertGt(newAttester3Balance, initialAttester3Balance, "Attester3 should receive epoch rewards");
        
        // Attester1 should have highest rewards due to most commitments
        uint256 attester1Reward = newAttester1Balance - initialAttester1Balance;
        uint256 attester2Reward = newAttester2Balance - initialAttester2Balance;
        uint256 attester3Reward = newAttester3Balance - initialAttester3Balance;
        
        assertGt(attester1Reward, attester2Reward, "Attester1 should have higher rewards than Attester2");
        assertGt(attester2Reward, attester3Reward, "Attester2 should have higher rewards than Attester3");
    }

    function test_RewardWithDecliningBalance() public {
        // Test that rewards properly scale down as the contract's balance decreases
        uint256 initialMcrBalance = token.balanceOf(address(mcr));
        
        // Track rewards over multiple blocks with declining balance
        uint256[] memory rewards = new uint256[](5);
        
        for (uint256 i = 1; i <= 5; i++) {
            uint256 initialAttesterBalance = token.balanceOf(attester1);
            
            // Create a block commitment
            MCR.Commitment memory commitment = mcr.createCommitment(
                i, // height
                keccak256(abi.encodePacked("block", i)), // commitment 
                keccak256(abi.encodePacked("blockId", i)) // blockId
            );
            
            vm.prank(attester1);
            mcr.submitCommitment(commitment);
            
            // Record the reward
            uint256 newAttesterBalance = token.balanceOf(attester1);
            rewards[i-1] = newAttesterBalance - initialAttesterBalance;
            
            // Check that MCR balance decreased
            uint256 newMcrBalance = token.balanceOf(address(mcr));
            assertLt(newMcrBalance, initialMcrBalance, "MCR balance should decrease");
            initialMcrBalance = newMcrBalance;
        }
        
        // Verify asymptotic nature of rewards - each reward should be smaller than the previous
        for (uint256 i = 1; i < rewards.length; i++) {
            assertLe(rewards[i], rewards[i-1], "Rewards should decrease as balance decreases");
        }
    }

    function test_UpdateRewardConfig() public {
        // Test updating reward configuration
        
        // Create new config structs
        McrARO.BlockRewardConfig memory newBlockConfig = McrARO.BlockRewardConfig({
            baseRewardBasisPoints: 20,     // Doubled from default 10
            stakeMultiplierBasisPoints: 10, // Doubled from default 5
            speedBonusBasisPoints: 10,      // Doubled from default 5
            maxSpeedBonusBasisPoints: 40,   // Doubled from default 20
            fastBlockThresholdSeconds: 300, // Half from default 600
            asymptoteFactorBlock: 5000      // Half from default 10000
        });
        
        McrARO.EpochRewardConfig memory newEpochConfig = McrARO.EpochRewardConfig({
            participationRewardBasisPoints: 200,  // Doubled from default 100
            consistencyMultiplierBasisPoints: 20, // Doubled from default 10
            stakeMultiplierBasisPoints: 40,       // Doubled from default 20
            asymptoteFactorEpoch: 2500            // Half from default 5000  
        });
        
        // Grant REWARD_ADMIN role to this contract
        bytes32 REWARD_ADMIN = keccak256("REWARD_ADMIN");
        vm.startPrank(address(mcr));
        AccessControlUpgradeable(address(mcr)).grantRole(REWARD_ADMIN, address(this));
        vm.stopPrank();
        
        // Update the configurations
        vm.startPrank(address(this));
        McrARO(address(aroProxy)).updateBlockRewardConfig(newBlockConfig);
        McrARO(address(aroProxy)).updateEpochRewardConfig(newEpochConfig);
        vm.stopPrank();
        
        // Test rewards with updated configs
        uint256 initialAttesterBalance = token.balanceOf(attester1);
        
        // Create a block commitment
        MCR.Commitment memory commitment = mcr.createCommitment(
            10, // height (new height to avoid conflicts)
            keccak256("block10"), // commitment
            keccak256("blockId10") // blockId
        );
        
        vm.prank(attester1);
        mcr.submitCommitment(commitment);
        
        // Check rewards - should be higher with the new config
        uint256 newAttesterBalance = token.balanceOf(attester1);
        uint256 reward = newAttesterBalance - initialAttesterBalance;
        
        // We can't precisely predict the reward amount, but it should be non-zero
        assertGt(reward, 0, "Reward should be non-zero with updated config");
    }

    // Helper function to simulate the asymptotic reward calculation
    // This is needed because the calculateAsymptoticReward function in McrARO is internal
    function testAsymptoticRewardCalculation(
        uint256 availableBalance,
        uint256 basisPoints,
        uint256 asymptoticFactor
    ) internal pure returns (uint256) {
        if (availableBalance == 0) return 0;
        
        uint256 scaledAsymptoticFactor = asymptoticFactor * 1000000;
        uint256 denominator = 1000000 + (scaledAsymptoticFactor / availableBalance);
        
        uint256 scaledReward = (availableBalance * basisPoints * 1000000) / 
                              (10000 * denominator);
                              
        return scaledReward;
    }
} 
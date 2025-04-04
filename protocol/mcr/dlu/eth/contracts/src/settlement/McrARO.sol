// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {IMovementStaking} from "../staking/interfaces/IMovementStaking.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {MCRStorage} from "./MCRStorage.sol";
import {IMCR} from "./interfaces/IMCR.sol";
import {IMcrReward} from "./interfaces/IMcrReward.sol";
import {AccessControlUpgradeable} from "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";

/**
 * @title McrARO - Asymptotic Reward One
 * @notice Implementation of asymptotic reward distribution for the MCR system
 * @dev This contract is designed to be called via delegatecall from the MCR contract
 *      to ensure msg.sender remains the MCR contract when interacting with MovementStaking.
 *      Rewards scale asymptotically based on the MCR contract's token balance to ensure
 *      the reward pool is never depleted.
 */
contract McrARO is MCRStorage, IMcrReward {
    using EnumerableSet for EnumerableSet.AddressSet;

    // Constants for basis point calculations
    uint256 private constant BASIS_POINTS_DENOMINATOR = 10000;
    
    // Constants for asymptotic reward calculations
    uint256 private constant REWARD_SCALE_DENOMINATOR = 1000000;
    
    /**
     * @notice Configuration for block commitment rewards
     * @dev Parameters for configuring block-level rewards
     */
    struct BlockRewardConfig {
        uint256 baseRewardBasisPoints;     // Base reward as basis points of available token balance
        uint256 stakeMultiplierBasisPoints; // Multiplier for stake-based rewards in basis points
        uint256 speedBonusBasisPoints;     // Speed bonus as basis points of available token balance
        uint256 maxSpeedBonusBasisPoints;  // Maximum speed bonus as basis points of available token balance
        uint256 fastBlockThresholdSeconds; // Threshold in seconds for considering a block "fast"
        uint256 asymptoteFactorBlock;      // Controls curve steepness for block rewards (higher = gentler curve)
    }

    /**
     * @notice Configuration for epoch rollover rewards
     * @dev Parameters for configuring epoch-level rewards
     */
    struct EpochRewardConfig {
        uint256 participationRewardBasisPoints; // Participation reward as basis points of available token balance
        uint256 consistencyMultiplierBasisPoints; // Multiplier for consistent attestation in basis points
        uint256 stakeMultiplierBasisPoints;     // Multiplier for stake-weighted rewards in basis points
        uint256 asymptoteFactorEpoch;          // Controls curve steepness for epoch rewards (higher = gentler curve)
    }

    /// @notice Configuration for block commitment rewards
    BlockRewardConfig public blockRewardConfig;

    /// @notice Configuration for epoch rollover rewards
    EpochRewardConfig public epochRewardConfig;

    /// @notice Tracks the last timestamp a block commitment was accepted for speed bonus calculation
    mapping(uint256 commitmentHeight => uint256 timestamp) public lastBlockTimestamp;

    /// @notice Tracks valid commitments by attesters in each epoch for consistency rewards
    mapping(uint256 epoch => mapping(address attester => uint256 commitCount)) public epochCommitments;

    /// @notice Event emitted when a block commitment reward is distributed
    event CommitmentRewarded(
        address indexed attester,
        uint256 commitmentHeight,
        bytes32 indexed commitment,
        uint256 rewardAmount,
        uint256 availableBalance
    );

    /// @notice Event emitted when an epoch rollover reward is distributed
    event EpochRolloverRewarded(
        uint256 indexed previousEpoch,
        uint256 indexed newEpoch,
        address indexed attester,
        uint256 rewardAmount,
        uint256 availableBalance
    );

    /// @notice Role for reward configuration
    bytes32 public constant REWARD_ADMIN = keccak256("REWARD_ADMIN");

    /**
     * @notice Initialize the reward configuration with default values
     * @dev This should be called during MCR initialization
     */
    function initializeRewardConfig() external {
        // Default values - these should be adjusted based on tokenomics
        blockRewardConfig = BlockRewardConfig({
            baseRewardBasisPoints: 10,     // 0.1% of available balance
            stakeMultiplierBasisPoints: 5, // 0.05% per staked token
            speedBonusBasisPoints: 5,      // 0.05% of available balance
            maxSpeedBonusBasisPoints: 20,  // 0.2% of available balance
            fastBlockThresholdSeconds: 600, // 10 minutes
            asymptoteFactorBlock: 10000    // Controls curve steepness
        });

        epochRewardConfig = EpochRewardConfig({
            participationRewardBasisPoints: 100, // 1% of available balance
            consistencyMultiplierBasisPoints: 10, // 0.1% per commitment
            stakeMultiplierBasisPoints: 20,      // 0.2% for stake proportion
            asymptoteFactorEpoch: 5000           // Controls curve steepness
        });
    }

    /**
     * @notice Update block reward configuration
     * @param _config New block reward configuration
     */
    function updateBlockRewardConfig(BlockRewardConfig calldata _config) external {
        // This would check that msg.sender has the REWARD_ADMIN role
        require(AccessControlUpgradeable(address(this)).hasRole(REWARD_ADMIN, msg.sender), "UNAUTHORIZED");
        blockRewardConfig = _config;
    }

    /**
     * @notice Update epoch reward configuration
     * @param _config New epoch reward configuration
     */
    function updateEpochRewardConfig(EpochRewardConfig calldata _config) external {
        // This would check that msg.sender has the REWARD_ADMIN role
        require(AccessControlUpgradeable(address(this)).hasRole(REWARD_ADMIN, msg.sender), "UNAUTHORIZED");
        epochRewardConfig = _config;
    }

    /**
     * @notice Calculate an asymptotic reward amount based on available balance
     * @dev Ensures rewards decrease as the available balance decreases
     * @param availableBalance The token balance available for rewards
     * @param basisPoints The target percentage in basis points (1/10000)
     * @param asymptoticFactor The factor controlling curve steepness
     * @return uint256 The calculated reward amount
     */
    function calculateAsymptoticReward(
        uint256 availableBalance,
        uint256 basisPoints, 
        uint256 asymptoticFactor
    ) internal pure returns (uint256) {
        // Simple asymptotic function: 
        // reward = (balance * basisPoints) / (basis_points_denominator * (1 + (asymptoticFactor / balance)))
        
        // This function approaches basisPoints/BASIS_POINTS_DENOMINATOR * availableBalance as availableBalance
        // becomes large, but decreases faster as availableBalance decreases
        
        if (availableBalance == 0) return 0;
        
        uint256 scaledAsymptoticFactor = asymptoticFactor * REWARD_SCALE_DENOMINATOR;
        uint256 denominator = REWARD_SCALE_DENOMINATOR + (scaledAsymptoticFactor / availableBalance);
        
        uint256 scaledReward = (availableBalance * basisPoints * REWARD_SCALE_DENOMINATOR) / 
                              (BASIS_POINTS_DENOMINATOR * denominator);
                              
        return scaledReward;
    }

    /**
     * @notice Get the available token balance for rewards
     * @return uint256 Available balance
     */
    function getAvailableBalance() internal view returns (uint256) {
        // Get token used for staking from the staking contract
        IERC20 token = IERC20(address(stakingContract.getToken()));
        
        // Return the MCR contract's balance of this token
        return token.balanceOf(address(this));
    }

    /*
     * @notice Reward attesters for a successful block commitment
     * @dev Called during _acceptCommitment in the MCR contract via delegatecall
     * @param commitmentHeight The height of the accepted block
     * @param commitment The accepted block commitment hash
     * @param blockId The unique identifier of the accepted block
     * @param attester The attester who submitted the accepted commitment
     * @return success Whether the reward distribution was successful
     */
    function rewardCommitment(
        uint256 commitmentHeight,
        bytes32 commitment,
        bytes32 _blockId,
        address attester
    ) external override returns (bool success) {
        // silence unused variable warning
        _blockId;
        
        uint256 availableBalance = getAvailableBalance();
        
        // Don't proceed if no balance is available for rewards
        if (availableBalance == 0) return false;
        
        // Calculate speed bonus based on time since last block
        uint256 speedBonus = 0;
        uint256 lastTime = lastBlockTimestamp[commitmentHeight - 1];
        if (lastTime > 0) {
            uint256 timeDiff = block.timestamp - lastTime;
            // If block was produced faster than the threshold, apply speed bonus
            if (timeDiff < blockRewardConfig.fastBlockThresholdSeconds) {
                speedBonus = calculateAsymptoticReward(
                    availableBalance,
                    blockRewardConfig.speedBonusBasisPoints,
                    blockRewardConfig.asymptoteFactorBlock
                );
                
                uint256 maxSpeedBonus = calculateAsymptoticReward(
                    availableBalance,
                    blockRewardConfig.maxSpeedBonusBasisPoints,
                    blockRewardConfig.asymptoteFactorBlock
                );
                
                if (speedBonus > maxSpeedBonus) {
                    speedBonus = maxSpeedBonus;
                }
            }
        }
        
        // Update last block timestamp for future speed bonus calculations
        lastBlockTimestamp[commitmentHeight] = block.timestamp;
        
        // Calculate base reward
        uint256 baseReward = calculateAsymptoticReward(
            availableBalance,
            blockRewardConfig.baseRewardBasisPoints,
            blockRewardConfig.asymptoteFactorBlock
        );
        
        // Calculate stake-based bonus
        uint256 epoch = commitmentHeightEpochAssignments[commitmentHeight];
        uint256 attesterStake = stakingContract.getStakeAtEpoch(
            address(this),
            epoch,
            attester,
            address(stakingContract.getToken())
        );
        
        // Apply stake multiplier in basis points
        uint256 stakeBonus = (attesterStake * blockRewardConfig.stakeMultiplierBasisPoints) / BASIS_POINTS_DENOMINATOR;
        
        // Apply asymptotic scaling to stake bonus as well
        stakeBonus = Math.min(
            stakeBonus,
            calculateAsymptoticReward(
                availableBalance,
                blockRewardConfig.stakeMultiplierBasisPoints * 2, // Cap at twice the basis points
                blockRewardConfig.asymptoteFactorBlock
            )
        );
        
        // Calculate total reward
        uint256 totalReward = baseReward + speedBonus + stakeBonus;
        
        // Ensure reward doesn't exceed available balance
        totalReward = Math.min(totalReward, availableBalance);
        
        // Track commitments for epoch-level rewards
        epochCommitments[epoch][attester]++;
        
        // Get all custodians for this domain
        address[] memory custodians = stakingContract.getCustodiansByDomain(address(this));
        
        // Only reward if we have at least one custodian
        if (custodians.length > 0 && totalReward > 0) {
            // Prepare arrays for the reward call
            address[] memory attesters = new address[](1);
            uint256[] memory amounts = new uint256[](1);
            address[] memory custodianAddresses = new address[](1);
            
            attesters[0] = attester;
            amounts[0] = totalReward;
            custodianAddresses[0] = custodians[0]; // Using the first custodian for simplicity
            
            // Call reward function - since this is executed via delegatecall,
            // msg.sender will be the MCR contract itself
            stakingContract.reward(attesters, amounts, custodianAddresses);
            
            // Emit event
            emit CommitmentRewarded(
                attester,
                commitmentHeight,
                commitment,
                totalReward,
                availableBalance
            );
            
            return true;
        }
        
        return false;
    }

    /**
     * @notice Reward attesters during epoch rollover
     * @dev Called during epoch rollover in the MCR contract via delegatecall
     * @param previousEpoch The epoch that just completed
     * @param newEpoch The new epoch that is starting
     * @return success Whether the reward distribution was successful
     */
    function rewardEpochRollover(
        uint256 previousEpoch,
        uint256 newEpoch
    ) external override returns (bool success) {
        uint256 availableBalance = getAvailableBalance();
        
        // Don't proceed if no balance is available for rewards
        if (availableBalance == 0) return false;
        
        address[] memory attesters = stakingContract.getAttestersByDomain(address(this));
        address[] memory custodians = stakingContract.getCustodiansByDomain(address(this));
        
        // Only proceed if we have attesters and custodians
        if (attesters.length == 0 || custodians.length == 0) {
            return false;
        }
        
        // Using arrays for batch rewards
        address[] memory rewardAttesters = new address[](attesters.length);
        uint256[] memory rewardAmounts = new uint256[](attesters.length);
        address[] memory rewardCustodians = new address[](attesters.length);
        
        // Get total stake in the epoch for relative reward calculation
        uint256 totalEpochStake = stakingContract.getTotalStakeForEpoch(
            address(this), 
            previousEpoch, 
            address(stakingContract.getToken())
        );
        
        // Calculate base participation reward
        uint256 baseParticipationReward = calculateAsymptoticReward(
            availableBalance,
            epochRewardConfig.participationRewardBasisPoints,
            epochRewardConfig.asymptoteFactorEpoch
        );
        
        uint256 validRewards = 0;
        uint256 totalRewardAmount = 0;
        
        for (uint256 i = 0; i < attesters.length; i++) {
            address attester = attesters[i];
            
            // Get attester's stats for reward calculation
            uint256 commitCount = epochCommitments[previousEpoch][attester];
            uint256 attesterStake = stakingContract.getStakeAtEpoch(
                address(this),
                previousEpoch,
                attester,
                address(stakingContract.getToken())
            );
            
            // Skip attesters with no commitments
            if (commitCount == 0) {
                continue;
            }
            
            // Calculate consistency bonus based on number of commitments
            uint256 consistencyBonus = (commitCount * epochRewardConfig.consistencyMultiplierBasisPoints * baseParticipationReward) / BASIS_POINTS_DENOMINATOR;
            
            // Calculate stake proportion bonus
            uint256 stakeProportion = 0;
            if (totalEpochStake > 0) {
                stakeProportion = (attesterStake * epochRewardConfig.stakeMultiplierBasisPoints * baseParticipationReward) / 
                                 (totalEpochStake * BASIS_POINTS_DENOMINATOR);
            }
            
            // Calculate total reward
            uint256 totalReward = baseParticipationReward + consistencyBonus + stakeProportion;
            
            // Add to tracking variables
            totalRewardAmount += totalReward;
            
            // Ensure we don't exceed available balance
            if (totalRewardAmount > availableBalance) {
                // Scale down this reward proportionally
                totalReward = (totalReward * availableBalance) / totalRewardAmount;
                totalRewardAmount = availableBalance;
            }
            
            // Add to reward arrays
            rewardAttesters[validRewards] = attester;
            rewardAmounts[validRewards] = totalReward;
            rewardCustodians[validRewards] = custodians[0]; // Using first custodian for simplicity
            
            // Emit event
            emit EpochRolloverRewarded(
                previousEpoch,
                newEpoch,
                attester,
                totalReward,
                availableBalance
            );
            
            validRewards++;
            
            // Reset commitment counter for next epoch
            epochCommitments[previousEpoch][attester] = 0;
        }
        
        // Only call reward if we have valid rewards to distribute
        if (validRewards > 0 && totalRewardAmount > 0) {
            // If not all attesters received rewards, resize the arrays
            if (validRewards < attesters.length) {
                address[] memory finalAttesters = new address[](validRewards);
                uint256[] memory finalAmounts = new uint256[](validRewards);
                address[] memory finalCustodians = new address[](validRewards);
                
                for (uint256 i = 0; i < validRewards; i++) {
                    finalAttesters[i] = rewardAttesters[i];
                    finalAmounts[i] = rewardAmounts[i];
                    finalCustodians[i] = rewardCustodians[i];
                }
                
                // Call reward function
                stakingContract.reward(finalAttesters, finalAmounts, finalCustodians);
            } else {
                // Call reward function with original arrays
                stakingContract.reward(rewardAttesters, rewardAmounts, rewardCustodians);
            }
            
            return true;
        }
        
        return false;
    }
} 
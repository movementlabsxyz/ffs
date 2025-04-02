// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "forge-std/console.sol";
import {MovementStaking, IMovementStaking} from "../staking/MovementStaking.sol";

/**
 * @title MCRStorage
 * @notice Storage contract for Movement Chain Relay (MCR) data structures
 */
contract MCRStorage {

    /// @notice Reference to the staking contract
    IMovementStaking public stakingContract;

    /// @notice The number of blocks that can be submitted ahead of the lastAcceptedBlockHeight
    /// @dev This allows for batching without attesters locking down the set by pushing too far ahead
    /// @dev Could be replaced by a 2/3 stake vote on block height to epoch assignment, but would require:
    /// @dev 1. Block heights have a non-decreasing mapping to epochs
    /// @dev 2. Votes get accumulated reasonably near the end of the epoch
    /// @dev Using a race with tolerance is simpler and satisfies these constraints
    uint256 public leadingBlockTolerance;

    /// @notice Tracks the last accepted block height
    /// @dev Used to ensure blocks are submitted in order and handle staking effectively
    uint256 public lastAcceptedBlockHeight;

    /**
     * @notice Structure representing a block around a commitment
     * @dev Height 0 represents uncommitted; all other values are legitimate heights
     */
    struct BlockCommitment {
        uint256 height;
        bytes32 commitment;
        bytes32 blockId;
    }

    /// @notice Maps each block height to its corresponding epoch
    mapping(uint256 blockHeight => uint256 epoch) public blockHeightEpochAssignments;

    /// @notice Tracks commitments from each attester for each block height
    mapping(uint256 blockHeight => mapping(address attester => BlockCommitment)) public commitments;

    /// @notice Tracks total stake accumulated for each commitment at each block height
    mapping(uint256 blockHeight => mapping(bytes32 commitement => uint256 stake)) public commitmentStakes;

    /// @notice Maps block height to accepted block commitment
    mapping(uint256 blockHeight => BlockCommitment) public acceptedBlocks;

    /// @notice Whether open attestation is enabled
    bool public openAttestationEnabled;

    /// @notice Versioned mapping of accepted blocks
    mapping(uint256 => mapping(uint256 blockHeight => BlockCommitment)) public versionedAcceptedBlocks;
    
    /// @notice Current version for accepted blocks
    uint256 public acceptedBlocksVersion;

    /// @dev Reserved storage gap for future upgrades
    uint256[47] internal __gap;

}
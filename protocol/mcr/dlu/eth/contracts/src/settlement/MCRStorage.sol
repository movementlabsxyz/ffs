// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import "forge-std/console.sol";
import {MovementStaking, IMovementStaking} from "../staking/MovementStaking.sol";

/**
 * @title MCRStorage
 * @notice Storage contract for Multi Commit Rollup (MCR) data structures
 */
contract MCRStorage {

    /// @notice Reference to the staking contract
    IMovementStaking public stakingContract;

    /// @notice The number of blocks that can be submitted ahead of the lastAcceptedCommitmentHeight
    /// @dev This allows for batching without attesters locking down the set by pushing too far ahead
    /// @dev Could be replaced by a 2/3 stake vote on block height to epoch assignment, but would require:
    /// @dev 1. Block heights have a non-decreasing mapping to epochs
    /// @dev 2. Votes get accumulated reasonably near the end of the epoch
    /// @dev Using a race with tolerance is simpler and satisfies these constraints
    uint256 public leadingCommitmentTolerance;

    /// @notice Tracks the last accepted block height
    /// @dev Used to ensure blocks are submitted in order and handle staking effectively
    uint256 public lastAcceptedCommitmentHeight;

    /**
     * @notice Structure representing a block around a commitment
     * @dev Height 0 represents uncommitted; all other values are legitimate heights
     */
    struct Commitment {
        uint256 height;
        bytes32 commitmentValue;
        bytes32 blockId;
    }

    /// @notice Maps each block height to its corresponding epoch
    mapping(uint256 commitmentHeight => uint256 epoch) public commitmentHeightEpochAssignments;

    /// @notice Tracks block commitments from each attester for each block height
    mapping(uint256 commitmentHeight => mapping(address attester => Commitment)) public commitments;

    /// @notice Tracks total stake accumulated for each commitment at each block height
    mapping(uint256 commitmentHeight => mapping(bytes32 commitement => uint256 stake)) public commitmentStakes;

    /// @notice Maps block height to accepted block commitment
    mapping(uint256 commitmentHeight => Commitment) public acceptedCommitments;

    /// @notice Whether open attestation is enabled
    bool public openAttestationEnabled;

    /// @notice Versioned mapping of accepted block commitments
    mapping(uint256 => mapping(uint256 commitmentHeight => Commitment)) public versionedAcceptedCommitments;
    
    /// @notice Current version for accepted block commitments
    uint256 public acceptedCommitmentsVersion;

    /// @dev Reserved storage gap for future upgrades
    uint256[47] internal __gap;

}
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {MovementStaking, IMovementStaking} from "../staking/MovementStaking.sol";

contract PCPStorage {

    IMovementStaking public stakingContract;

    // The MOVE token address, which is the primary custodian for rewards in the staking contract
    address public moveTokenAddress;

    // the number of commitments that can be submitted ahead of the lastPostconfirmedCommitmentHeight
    // this allows for things like batching to take place without some attesters locking down the attester set by pushing too far ahead
    // ? this could be replaced by a 2/3 stake vote on the commitment height to epoch assignment
    // ? however, this protocol becomes more complex as you to take steps to ensure that...
    // ? 1. commitment heights have a non-decreasing mapping to epochs
    // ? 2. Votes get accumulated reasonable near the end of the epoch (i.e., your vote is cast for the epoch you vote fore and the next)
    // ? if howevever, you simply allow a race with the tolerance below, both of these are satisfied without the added complexity
    // TODO the above explanation is not clear and needs to be rephrased or further explained
    // TODO unless this is clarified or becomes relevant in the future, this comment should be removed
    uint256 public leadingCommitmentTolerance;

    // track the last postconfirmed commitment height, so that we can require commitments are submitted in order and handle staking effectively
    uint256 public lastPostconfirmedCommitmentHeight;

    /// Postconfirmer term time in seconds. The postconfirmer remains the same for postconfirmerDuration period.
    // The Postconfirmer term can be minimal, but it should not be too small as the postconfirmer should have some time 
    // to prepare and post L1-transactions that will start the validation of attestations.
    uint256 public postconfirmerDuration;

    /// @notice Minimum time that must pass before a commitment can be postconfirmed
    uint256 public minCommitmentAgeForPostconfirmation;

    /// @notice Max time the postconfirmer can be non-reactive to an honest commitment
    uint256 public postconfirmerPrivilegeDuration;

    // TODO i added these param descriptions. are these correct?
    /// Struct to store block commitment details
    /// @param height The height of the commitment
    /// @param commitmentValue The hash of the commitment value
    /// @param commitmentId The unique identifier of the commitment (hash of the commitment value)
    struct Commitment {
        // currently, to simplify the api, we'll say 0 is uncommitted all other numbers are legitimate heights
        uint256 height;
        bytes32 commitmentValue;
        bytes32 commitmentId;
    }

    // map each commitment height to an epoch
    mapping(uint256 commitmentHeight => uint256 epoch) public commitmentHeightAssignedEpoch;

    // track each commitment from each attester for each commitment height
    mapping(uint256 commitmentHeight => mapping(address attester => Commitment)) public commitments;

    // track the total stake accumulate for each commitment for each commitment height
    mapping(uint256 commitmentHeight => mapping(bytes32 commitment => uint256 stake)) public commitmentStake;

    // track when each commitment was first seen for each commitment height
    mapping(uint256 commitmentHeight => mapping(bytes32 commitment => uint256 timestamp)) public commitmentFirstSeenAt;

    // Track which attester postconfirmed a given commitment height
    mapping(uint256 commitmentHeight => address attester) public postconfirmedBy;

    // Track if postconfirmer postconfirmed a given commitment height 
    // TODO this may be redundant due to one of the mappings below
    mapping(uint256 commitmentHeight => bool) public postconfirmedByPostconfirmer;

    // Track the L1Block height when a commitment height was postconfirmed
    mapping(uint256 commitmentHeight => uint256 L1BlockHeight) public postconfirmedAtL1BlockHeight;

    // TODO: either the L1Block timestamp or L1Block height should be tracked, both are not needed, but keep both until we know which one is not needed
    // Track the L1Block timestamp when a commitment height was postconfirmed
    mapping(uint256 commitmentHeight => uint256 L1BlockTimestamp) public postconfirmedAtL1BlockTimestamp;

    // Track the L1Block height when a commitment height was postconfirmed by the postconfirmer
    mapping(uint256 commitmentHeight => uint256 L1BlockHeight) public postconfirmedAtL1BlockHeightByPostconfirmer;

    // map commitment height to postconfirmed commitment 
    mapping(uint256 commitmentHeight => Commitment) public postconfirmedCommitments;

    // whether we allow open attestation
    bool public openAttestationEnabled;

    // versioned scheme for postconfirmed commitments
    mapping(uint256 => mapping(uint256 commitmentHeight => Commitment)) public versionedPostconfirmedCommitments;
    uint256 public postconfirmedCommitmentsVersion;

    // track reward points for attesters
    mapping(uint256 epoch => mapping(address attester => uint256 points)) public attesterRewardPoints;

    // track reward points for postconfirmers
    mapping(uint256 epoch => mapping(address postconfirmer => uint256 points)) public postconfirmerRewardPoints;

    // track the reward per point for attesters
    uint256 public rewardPerAttestationPoint;

    // track the reward per point for postconfirmers
    uint256 public rewardPerPostconfirmationPoint;

    uint256[45] internal __gap; // Reduced by 1 for new mapping

}
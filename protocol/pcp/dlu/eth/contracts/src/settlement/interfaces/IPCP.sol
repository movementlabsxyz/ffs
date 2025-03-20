// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {PCPStorage} from "../PCPStorage.sol";

interface IPCP {

    event SuperBlockPostconfirmed(
        bytes32 indexed blockHash,
        bytes32 stateCommitment,
        uint256 height
    );
    event SuperBlockCommitmentSubmitted(
        bytes32 indexed blockHash,
        bytes32 stateCommitment,
        uint256 attesterStake
    );
    error UnacceptableSuperBlockCommitment();
    error AttesterAlreadyCommitted();

    /// @notice Gets the epoch duration
    function getEpochDuration() external view returns (uint256);

    /// @notice Gets the postconfirmer duration
    function getPostconfirmerDuration() external view returns (uint256);

    /// @notice Gets the postconfirmer
    function getPostconfirmer() external view returns (address);

    /// @notice submit a superblock commitment
    function submitSuperBlockCommitment(PCPStorage.SuperBlockCommitment memory commitment) external;

    /// @notice get the last postconfirmed superblock height
    function getLastPostconfirmedSuperBlockHeight() external view returns (uint256);

    /// @notice get the accepting epoch
    function getAcceptingEpoch() external view returns (uint256);

    /// @notice get the present epoch
    function getPresentEpoch() external view returns (uint256);

    /// @notice get the postconfirmed commitment for a given height
    function getPostconfirmedCommitment(uint256 height) external view returns (PCPStorage.SuperBlockCommitment memory);

    /// @notice postconfirm superblocks and rollover
    function postconfirmSuperBlocksAndRollover() external;

    /// @notice Sets the accepting epoch to a new value (must be higher than current)
    function setAcceptingEpoch(uint256 newEpoch) external;

    /// @notice The role that allows attesters to submit commitments
    function TRUSTED_ATTESTER() external pure returns (bytes32);

    /// @notice The role that allows the commitment admin to set the accepting epoch
    function COMMITMENT_ADMIN() external pure returns (bytes32);
}
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface IMCR {
    event CommitmentAccepted(
        bytes32 indexed blockHash,
        bytes32 stateCommitment,
        uint256 height
    );
    event CommitmentSubmitted(
        bytes32 indexed blockHash,
        bytes32 stateCommitment,
        uint256 attesterStake
    );
    error UnacceptableCommitment();
    error AttesterAlreadyCommitted();
}
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {IMCR} from "./IMCR.sol";
import {MovementStaking, IMovementStaking} from "../../staking/MovementStaking.sol";
import {MCRStorage} from "../MCRStorage.sol";

/**
 * @title IMcrReward
 * @notice Interface for rewarding attesters in the MCR system
 * @dev This interface is designed to be called via delegatecall from the MCR contract
 *      to ensure msg.sender remains the MCR contract when interacting with MovementStaking
 */
interface IMcrReward {
    /**
     * @notice Reward attesters for a successful block commitment
     * @dev Called during _acceptCommitment in the MCR contract
     * @param commitmentHeight The height of the accepted block
     * @param commitment The accepted block commitment hash
     * @param id The unique identifier of the accepted block
     * @param attester The attester who submitted the accepted commitment
     * @return success Whether the reward distribution was successful
     */
    function rewardCommitment(
        uint256 commitmentHeight,
        bytes32 commitment,
        bytes32 id,
        address attester
    ) external returns (bool success);

    /**
     * @notice Reward attesters during epoch rollover
     * @dev Called during epoch rollover in the MCR contract
     * @param previousEpoch The epoch that just completed
     * @param newEpoch The new epoch that is starting
     * @return success Whether the reward distribution was successful
     */
    function rewardEpochRollover(
        uint256 previousEpoch,
        uint256 newEpoch
    ) external returns (bool success);
} 
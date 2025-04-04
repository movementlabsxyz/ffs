// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "forge-std/console.sol";
import {MovementStaking, IMovementStaking} from "../staking/MovementStaking.sol";
import {MCRStorage} from "./MCRStorage.sol";
import {BaseSettlement} from "./settlement/BaseSettlement.sol";
import {IMCR} from "./interfaces/IMCR.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import {IMcrReward} from "./interfaces/IMcrReward.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

/**
 * @title MCR - Multi Commit Rollup
 * @notice Contract for handling block commitments and consensus in a multi-validator environment
 */
contract MCR is Initializable, BaseSettlement, MCRStorage, IMCR {

    /// @notice The role for commitment admin
    bytes32 public constant COMMITMENT_ADMIN = keccak256("COMMITMENT_ADMIN");

    /// @notice The role for trusted attester
    bytes32 public constant TRUSTED_ATTESTER = keccak256("TRUSTED_ATTESTER");

    /// @notice The reward contract
    /// @dev Default implementation is McrARO (Asymptotic Reward One)
    IMcrReward public rewardContract;

    /**
     * @notice Initializes the MCR contract
     * @param _stakingContract The staking contract to use
     * @param _lastAcceptedCommitmentHeight The last accepted block height
     * @param _leadingCommitmentTolerance The tolerance for leading blocks
     * @param _epochDuration The duration of each epoch
     * @param _custodians Array of custodian addresses
     */
    function initialize(
        IMovementStaking _stakingContract,
        uint256 _lastAcceptedCommitmentHeight,
        uint256 _leadingCommitmentTolerance,
        uint256 _epochDuration,
        address[] memory _custodians
    ) public initializer {
        __BaseSettlement_init_unchained();
        stakingContract = _stakingContract;
        leadingCommitmentTolerance = _leadingCommitmentTolerance;
        lastAcceptedCommitmentHeight = _lastAcceptedCommitmentHeight;
        stakingContract.registerDomain(_epochDuration, _custodians);
        grantCommitmentAdmin(msg.sender);
        grantTrustedAttester(msg.sender);
    }

    /**
     * @notice Grants commitment admin role to an account
     * @param account Address to grant the role to
     */
    function grantCommitmentAdmin(address account) public {
        console.log("grantCommitmentAdmin called by:", msg.sender);
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ADD_COMMITMENT_ADMIN_IS_ADMIN_ONLY"
        );
        grantRole(COMMITMENT_ADMIN, account);
        console.log("CommitmentAdmin role granted to:", account);

    }

    /**
     * @notice Grants commitment admin role to multiple accounts in batch
     * @param accounts Array of addresses to grant the role to
     */
    function batchGrantCommitmentAdmin(address[] memory accounts) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ADD_COMMITMENT_ADMIN_IS_ADMIN_ONLY"
        );
        for (uint256 i = 0; i < accounts.length; i++) {
            grantRole(COMMITMENT_ADMIN, accounts[i]);
        }
    }

    /**
     * @notice Creates a new block commitment structure
     * @param height Block height
     * @param commitment Commitment hash
     * @param blockId Unique identifier for the block
     * @return Commitment memory
     */
    function createCommitment(
        uint256 height,
        bytes32 commitment,
        bytes32 blockId
    ) public pure returns (Commitment memory) {
        return Commitment(height, commitment, blockId);
    }

    /**
     * @notice Calculates the maximum tolerable block height based on tolerance
     * @return uint256 Maximum tolerable block height
     */
    function getMaxTolerableBlockHeight() public view returns (uint256) {
        return lastAcceptedCommitmentHeight + leadingCommitmentTolerance;
    }

    /**
     * @notice Gets the epoch for the current block time
     * @return uint256 Current epoch by block time
     */
    function getEpochByBlockTime() public view returns (uint256) {
        return stakingContract.getEpochByBlockTime(address(this));
    }

    /**
     * @notice Gets the current epoch up to which blocks have been accepted
     * @return uint256 Current epoch
     */
    function getCurrentEpoch() public view returns (uint256) {
        return stakingContract.getCurrentEpoch(address(this));
    }

    /**
     * @notice Gets the next epoch
     * @return uint256 Next epoch
     */
    function getNextEpoch() public view returns (uint256) {
        return stakingContract.getNextEpoch(address(this));
    }

    /**
     * @notice Gets the stake for a given attester at a specific epoch
     * @param epoch The epoch to check
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Stake amount
     */
    function getStakeAtEpoch(
        uint256 epoch,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return
            stakingContract.getStakeAtEpoch(
                address(this),
                epoch,
                custodian,
                attester
            );
    }

    /**
     * @notice Computes the total stake across all custodians for an attester at a specific epoch
     * @dev TODO: Consider memoizing this function for efficiency
     * @param epoch The epoch to check
     * @param attester The attester address
     * @return uint256 Total stake amount
     */
    function computeAllStakeAtEpoch(
        uint256 epoch,
        address attester
    ) public view returns (uint256) {
        address[] memory custodians = stakingContract.getCustodiansByDomain(
            address(this)
        );
        uint256 totalStake = 0;
        for (uint256 i = 0; i < custodians.length; i++) {
            // Each custodian currently has a weight of 1
            totalStake += getStakeAtEpoch(epoch, custodians[i], attester);
        }
        return totalStake;
    }

    /**
     * @notice Gets the stake for a given attester at the current epoch
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Current epoch stake
     */
    function getCurrentEpochStake(
        address custodian,
        address attester
    ) public view returns (uint256) {
        return getStakeAtEpoch(getCurrentEpoch(), custodian, attester);
    }

    /**
     * @notice Computes the total stake across all custodians for an attester at the current epoch
     * @param attester The attester address
     * @return uint256 Total current epoch stake
     */
    function computeAllCurrentEpochStake(
        address attester
    ) public view returns (uint256) {
        return computeAllStakeAtEpoch(getCurrentEpoch(), attester);
    }

    /**
     * @notice Gets the total stake for a given epoch and custodian
     * @param epoch The epoch to check
     * @param custodian The custodian address
     * @return uint256 Total stake for epoch
     */
    function getTotalStakeForEpoch(
        uint256 epoch,
        address custodian
    ) public view returns (uint256) {
        return
            stakingContract.getTotalStakeForEpoch(
                address(this),
                epoch,
                custodian
            );
    }

    /**
     * @notice Accepts the genesis ceremony
     * @dev Only callable by admin
     */
    function acceptGenesisCeremony() public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ACCEPT_GENESIS_CEREMONY_IS_ADMIN_ONLY"
        );
        stakingContract.acceptGenesisCeremony();
    }

    /**
     * @notice Computes the total stake across all custodians for a specific epoch
     * @param epoch The epoch to check
     * @return uint256 Total stake for the epoch
     */
    function computeAllTotalStakeForEpoch(
        uint256 epoch
    ) public view returns (uint256) {
        address[] memory custodians = stakingContract.getCustodiansByDomain(
            address(this)
        );
        uint256 totalStake = 0;
        for (uint256 i = 0; i < custodians.length; i++) {
            // Each custodian currently has a weight of 1
            totalStake += getTotalStakeForEpoch(epoch, custodians[i]);
        }
        return totalStake;
    }

    /**
     * @notice Gets the total stake for a custodian at the current epoch
     * @param custodian The custodian address
     * @return uint256 Total stake for current epoch
     */
    function getTotalStakeForCurrentEpoch(
        address custodian
    ) public view returns (uint256) {
        return getTotalStakeForEpoch(getCurrentEpoch(), custodian);
    }

    /**
     * @notice Computes the total stake across all custodians for the current epoch
     * @return uint256 Total stake for current epoch
     */
    function computeAllTotalStakeForCurrentEpoch()
        public
        view
        returns (uint256)
    {
        return computeAllTotalStakeForEpoch(getCurrentEpoch());
    }

    /**
     * @notice Gets a validator's commitment at a specific block height
     * @param height Block height
     * @param attester Attester address
     * @return Commitment memory
     */
    function getValidatorCommitmentAtBlockHeight(
        uint256 height,
        address attester
    ) public view returns (Commitment memory) {
        return commitments[height][attester];
    }

    /**
     * @notice Sets the accepted commitment at a given block height
     * @param commitment The block commitment to set
     */
    function setAcceptedCommitmentAtBlockHeight(Commitment memory commitment) public {
        require(
            hasRole(COMMITMENT_ADMIN, msg.sender),
            "SET_LAST_ACCEPTED_COMMITMENT_AT_HEIGHT_IS_COMMITMENT_ADMIN_ONLY"
        );
        versionedAcceptedCommitments[acceptedCommitmentsVersion][commitment.height] = commitment;  
    }

    /**
     * @notice Sets the last accepted block height
     * @param height New last accepted block height
     */
    function setLastAcceptedBlockHeight(uint256 height) public {
        require(
            hasRole(COMMITMENT_ADMIN, msg.sender),
            "SET_LAST_ACCEPTED_BLOCK_HEIGHT_IS_COMMITMENT_ADMIN_ONLY"
        );
        lastAcceptedCommitmentHeight = height;
    }

    /**
     * @notice Forces the latest attestation by setting the block height
     * @dev Only safe when running with a single validator as it does not zero out follow-on commitments
     * @param commitment The block commitment to force
     */
    function forceLatestCommitment(Commitment memory commitment) public {
        console.log("forceLatestCommitment called by:", msg.sender);
        require(
            hasRole(COMMITMENT_ADMIN, msg.sender),
            "FORCE_LATEST_COMMITMENT_IS_COMMITMENT_ADMIN_ONLY"
        );

        // Increment the acceptedCommitmentsVersion (effectively removing all other accepted blocks)
        acceptedCommitmentsVersion += 1;
        versionedAcceptedCommitments[acceptedCommitmentsVersion][commitment.height] = commitment;
        lastAcceptedCommitmentHeight = commitment.height; 
    }

    /**
     * @notice Gets the accepted commitment at a specific block height
     * @param height Block height
     * @return Commitment memory
     */
    function getAcceptedCommitmentAtBlockHeight(uint256 height) public view returns (Commitment memory) {
        return versionedAcceptedCommitments[acceptedCommitmentsVersion][height];
    }

    /**
     * @notice Gets all attesters for this domain
     * @return address[] memory Array of attester addresses
     */
    function getAttesters() public view returns (address[] memory) {
        return stakingContract.getAttestersByDomain(address(this));
    }

    /**
     * @notice Submits a block commitment for a specific attester
     * @dev Internal function used by public submission methods
     * @param attester The attester submitting the commitment
     * @param commitment The block commitment being submitted
     */
    function submitCommitmentForAttester(
        address attester,
        Commitment memory commitment
    ) internal {
        emit DebugCheckpoint("Start submitCommitmentForAttester");
        emit DebugValues("Values", attester, commitment.height, lastAcceptedCommitmentHeight, leadingCommitmentTolerance);
        // Attester has already committed to a block at this height
        if (commitments[commitment.height][attester].height != 0)
            revert AttesterAlreadyCommitted();

        // We allow commitments to already accepted blocks to support lagging attesters
        // If uncommented, this would prevent commitments to already accepted blocks:
        // if (lastAcceptedCommitmentHeight > commitment.height) revert AlreadyAcceptedBlock();
        
        emit DebugCheckpoint("Before existing commitment check");
        uint256 existingCommitmentHeight = commitments[commitment.height][attester].height;
        emit DebugUint("Existing commitment height", existingCommitmentHeight);
        
        if (commitments[commitment.height][attester].height != 0) {
            emit DebugCheckpoint("Reverting: AttesterAlreadyCommitted - commitment exists");
            revert AttesterAlreadyCommitted();
        }
        emit DebugCheckpoint("After existing commitment check");
        
        emit DebugCheckpoint("Before tolerance check");
        if (lastAcceptedCommitmentHeight + leadingCommitmentTolerance < commitment.height) {
            emit DebugCheckpoint("Reverting: Height exceeds tolerance");
            revert AttesterAlreadyCommitted();
        }
        emit DebugCheckpoint("After tolerance check");

        emit DebugCheckpoint("Before epoch assignment");
        // Assign the block height to the current epoch if it hasn't been assigned yet
        if (commitmentHeightEpochAssignments[commitment.height] == 0) {
            commitmentHeightEpochAssignments[
                commitment.height
            ] = getEpochByBlockTime();
        }
        emit DebugCheckpoint("After epoch assignment");

        // Register the attester's commitment
        commitments[commitment.height][attester] = commitment;

        // Increment the commitment count by stake
        uint256 allCurrentEpochStake = computeAllCurrentEpochStake(attester);
        commitmentStakes[commitment.height][
            commitment.commitmentValue
        ] += allCurrentEpochStake;

        emit CommitmentSubmitted(
            commitment.blockId,
            commitment.commitmentValue,
            allCurrentEpochStake
        );

        // Keep ticking through to find accepted blocks
        // This allows for batching to be successful:
        // We can commit to blocks out to the tolerance point then accept them in order
        // Note: this could become costly for whoever submits the last block
        // Rewards should be managed accordingly
        while (tickOnBlockHeight(lastAcceptedCommitmentHeight + 1)) {}
    }

    /**
     * @notice Processes a specific block height to check for consensus
     * @dev Returns true if a commitment was accepted at this height
     * @param commitmentHeight The block height to process
     * @return bool True if a commitment was accepted
     */
    function tickOnBlockHeight(uint256 commitmentHeight) internal returns (bool) {
        // Get the epoch assigned to the block height
        uint256 blockEpoch = commitmentHeightEpochAssignments[commitmentHeight];

        // If the current epoch is behind, roll it over until we catch up
        // This is fine as long as we process blocks in order and the block-to-epoch 
        // assignment is non-decreasing
        while (getCurrentEpoch() < blockEpoch) {
            rollOverEpoch();
        }

        // We could track seen commitments in a set, but since our operations
        // are very cheap, the set would actually add overhead
        uint256 supermajority = (2 * computeAllTotalStakeForEpoch(blockEpoch)) /
            3;
        address[] memory attesters = getAttesters();

        // Iterate over the attester set
        for (uint256 i = 0; i < attesters.length; i++) {
            address attester = attesters[i];

            // Get the commitment for this attester at the block height
            Commitment memory commitment = commitments[commitmentHeight][
                attester
            ];

            // Check the total stake on the commitment
            uint256 totalStakeOnCommitment = commitmentStakes[
                commitment.height
            ][commitment.commitmentValue];

            if (totalStakeOnCommitment > supermajority) {
                // Accept the block commitment (may trigger epoch rollover)
                _acceptCommitment(commitment);

                // We found a commitment that was accepted
                return true;
            }
        }

        return false;
    }

    /**
     * @notice Grants trusted attester role to an account
     * @param attester Address to grant the role to
     */
    function grantTrustedAttester(address attester) public onlyRole(COMMITMENT_ADMIN) {
        grantRole(TRUSTED_ATTESTER, attester);
    }

    /**
     * @notice Grants trusted attester role to multiple accounts in batch
     * @param attesters Array of addresses to grant the role to
     */
    function batchGrantTrustedAttester(address[] memory attesters) public onlyRole(COMMITMENT_ADMIN) {
        for (uint256 i = 0; i < attesters.length; i++) {
            grantRole(TRUSTED_ATTESTER, attesters[i]);
        }

    }

    /**
     * @notice Enables or disables open attestation
     * @param enabled Boolean indicating if open attestation should be enabled
     */
    function setOpenAttestationEnabled(bool enabled) public onlyRole(COMMITMENT_ADMIN) {
        openAttestationEnabled = enabled;
    }

    /**
     * @notice Submits a single block commitment
     * @param commitment The block commitment to submit
     */
    function submitCommitment(Commitment memory commitment) public {
        emit DebugSubmitCommitment(
            msg.sender,
            hasRole(TRUSTED_ATTESTER, msg.sender),
            openAttestationEnabled
        );
        emit DebugCheckpoint("Before authorization check");
        require(
            openAttestationEnabled || hasRole(TRUSTED_ATTESTER, msg.sender),
            "UNAUTHORIZED_BLOCK_COMMITMENT"
        );
        emit DebugCheckpoint("After authorization check");
        submitCommitmentForAttester(msg.sender, commitment);
    }

    /**
     * @notice Submits multiple block commitments in batch
     * @param commitments Array of block commitments to submit
     */
    function submitBatchCommitment(Commitment[] memory commitments) public {
        require(
            openAttestationEnabled || hasRole(TRUSTED_ATTESTER, msg.sender),
            "UNAUTHORIZED_BLOCK_COMMITMENT"
        );
        for (uint256 i = 0; i < commitments.length; i++) {
            submitCommitmentForAttester(msg.sender, commitments[i]);
        }
    }

    /**
     * @notice Accepts a block commitment
     * @dev This shares recursion with tickOnBlockHeight, so it should be reentrant
     * @param commitment The block commitment to accept
     */
    function _acceptCommitment(
        Commitment memory commitment
    ) internal {
        uint256 currentEpoch = getCurrentEpoch();
        
        // Block commitment must be in the current epoch to be accepted
        // If not, this indicates a bug in the protocol
        if (commitmentHeightEpochAssignments[commitment.height] != currentEpoch)
            revert UnacceptableCommitment();

        // Set accepted block commitment
        versionedAcceptedCommitments[acceptedCommitmentsVersion][commitment.height] = commitment;

        // Set last accepted block height
        lastAcceptedCommitmentHeight = commitment.height;

        // Slash minority attesters with respect to the accepted block commitment
        slashMinority(commitment);

        // Emit the block accepted event
        emit BlockAccepted(
            commitment.blockId,
            commitment.commitmentValue,
            commitment.height
        );

        // Distribute rewards for the block commitment if reward contract is set
        if (address(rewardContract) != address(0)) {
            // Find the attester who made this commitment
            address[] memory attesters = stakingContract.getAttestersByDomain(address(this));
            for (uint256 i = 0; i < attesters.length; i++) {
                address attester = attesters[i];
                if (commitments[commitment.height][attester].commitmentValue == commitment.commitmentValue) {
                    // Use delegatecall to maintain MCR as msg.sender for the reward call
                    (bool _success, ) = address(rewardContract).delegatecall(
                        abi.encodeWithSelector(
                            IMcrReward.rewardCommitment.selector,
                            commitment.height,
                            commitment.commitmentValue,
                            commitment.blockId,
                            attester
                        )
                    );
                    // silence unused variable warning
                    _success;
                    // We don't handle the success case specially as rewards are optional
                    break;
                }
            }
        }

        // If the timestamp epoch is greater than the current epoch, roll over the epoch
        if (getEpochByBlockTime() > currentEpoch) {
            rollOverEpoch();
        }
    }

    /**
     * @notice Slashes minority attesters who committed to different blocks
     * @dev Currently a placeholder for future implementation
     * @param commitment The accepted block commitment
     */
    function slashMinority(Commitment memory commitment) internal {
        // Future implementation:
        // stakingContract.slash(custodians, attesters, amounts, refundAmounts);
    }

    /**
     * @notice Rolls over to the next epoch
     * @dev Non-reentrant because there is no need to reenter this function.
     *      It should be called iteratively. Marked on the internal method to
     *      simplify risks from complex calling patterns. This also calls an external contract.
     */
    function rollOverEpoch() internal {
        uint256 currentEpoch = getCurrentEpoch();
        
        // Call the staking contract to roll over the epoch
        stakingContract.rollOverEpoch();
        
        // Distribute epoch rewards if reward contract is set
        if (address(rewardContract) != address(0)) {
            // Use delegatecall to maintain MCR as msg.sender for the reward call
            (bool _success, ) = address(rewardContract).delegatecall(
                abi.encodeWithSelector(
                    IMcrReward.rewardEpochRollover.selector,
                    currentEpoch,
                    currentEpoch + 1
                )
            );
            // silence unused variable warning
            _success;
            // We don't handle the success case specially as rewards are optional
        }
    }

    /**
     * @notice Sets the reward contract
     * @dev Only callable by admin
     * @param _rewardContract The contract that implements IMcrReward
     */
    function setRewardContract(IMcrReward _rewardContract) external onlyRole(DEFAULT_ADMIN_ROLE) {
        rewardContract = _rewardContract;
    }

    event DebugSubmitCommitment(
        address caller,
        bool hasTrustedAttesterRole,
        bool openAttestationEnabled
    );

    event DebugSubmitCommitmentForAttester(
        address caller,
        address attester,
        uint256 height,
        uint256 existingHeight
    );

    event DebugCheckpoint(string message);
    
    event DebugValues(string message, address attester, uint256 height, uint256 lastAccepted, uint256 tolerance);
    event DebugUint(string message, uint256 value);
    
    function toString(uint256 value) internal pure returns (string memory) {
        if (value == 0) return "0";
        uint256 temp = value;
        uint256 digits;
        while (temp != 0) {
            digits++;
            temp /= 10;
        }
        bytes memory buffer = new bytes(digits);
        while (value != 0) {
            digits -= 1;
            buffer[digits] = bytes1(uint8(48 + uint256(value % 10)));
            value /= 10;
        }
        return string(buffer);
    }

    /**
     * @notice Allows users to stake tokens for the MCR domain
     * @param amount The amount to stake
     */
    function stake(uint256 amount) external {
        // Get the token from the staking contract
        IERC20 token = stakingContract.getToken();

        // approve the staking contract to spend the tokens
        token.approve(address(stakingContract), amount);
        
        // Call stakeFor on the staking contract to stake for the user
        stakingContract.stakeFor(msg.sender, token, amount);
    }

    /**
     * @notice Allows users to unstake tokens from the MCR domain
     * @param amount The amount to unstake
     */
    function unstake(uint256 amount) external {
        // Get the token from the staking contract
        IERC20 token = stakingContract.getToken();

        // Call unstake on the staking contract
        stakingContract.unstakeFor(msg.sender, address(token), amount);
    }
}

// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {EnumerableSet} from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {MovementStaking, IMovementStaking} from "../staking/MovementStaking.sol";
import {PCPStorage} from "./PCPStorage.sol";
import {BaseSettlement} from "./settlement/BaseSettlement.sol";
import {IPCP} from "./interfaces/IPCP.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

contract PCP is Initializable, BaseSettlement, PCPStorage, IPCP {

    // A role for setting commitments
    bytes32 public constant COMMITMENT_ADMIN = keccak256("COMMITMENT_ADMIN");

    // Trusted attesters admin
    bytes32 public constant TRUSTED_ATTESTER = keccak256("TRUSTED_ATTESTER");

    /// @notice Error thrown when postconfirmer term is greater than 256 blocks
    error PostconfirmerDurationTooLong();

    /// @notice Error thrown when postconfirmer term is too large for epoch duration
    error PostconfirmerDurationTooLongForEpoch();

    /// @notice Error thrown when minimum commitment age is greater than epoch duration
    error minCommitmentAgeForPostconfirmationTooLong();

    /// @notice Error thrown when maximum postconfirmer non-reactivity time is greater than epoch duration
    error postconfirmerPrivilegeDurationTooLong();

    // ----------------------------------------------------------------
    // -------- 1. Role & Permission Management -----------------------
    // ----------------------------------------------------------------

    function grantCommitmentAdmin(address account) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ADD_COMMITMENT_ADMIN_IS_ADMIN_ONLY"
        );
        grantRole(COMMITMENT_ADMIN, account);
    }

    function batchGrantCommitmentAdmin(address[] memory accounts) public {
        require(
            hasRole(DEFAULT_ADMIN_ROLE, msg.sender),
            "ADD_COMMITMENT_ADMIN_IS_ADMIN_ONLY"
        );
        for (uint256 i = 0; i < accounts.length; i++) {
            grantRole(COMMITMENT_ADMIN, accounts[i]);
        }
    }

    function grantTrustedAttester(address attester) public onlyRole(COMMITMENT_ADMIN) {
        grantRole(TRUSTED_ATTESTER, attester);
    }

    function batchGrantTrustedAttester(address[] memory attesters) public onlyRole(COMMITMENT_ADMIN) {
        for (uint256 i = 0; i < attesters.length; i++) {
            grantRole(TRUSTED_ATTESTER, attesters[i]);
        }
    }

    // ----------------------------------------------------------------
    // -------- 2. Contract Initialization & Configuration ------------
    // ----------------------------------------------------------------

    function initialize(
        IMovementStaking _stakingContract,
        uint256 _lastPostconfirmedCommitmentHeight,
        uint256 _leadingCommitmentTolerance,
        uint256 _epochDuration, // in time units
        address[] memory _custodians,
        uint256 _postconfirmerDuration, // in time units
        address _moveTokenAddress  // the primary custodian for rewards in the staking contract
    ) public initializer {
        __BaseSettlement_init_unchained();
        stakingContract = _stakingContract;
        leadingCommitmentTolerance = _leadingCommitmentTolerance;
        lastPostconfirmedCommitmentHeight = _lastPostconfirmedCommitmentHeight;
        stakingContract.registerDomain(_epochDuration, _custodians);
        grantCommitmentAdmin(msg.sender);
        grantTrustedAttester(msg.sender);
        postconfirmerDuration = _postconfirmerDuration;
        moveTokenAddress = _moveTokenAddress;

        // Set default values to 1/10th of epoch duration
        // NOTE since epochduration divided by 10 may not be an exact integer, the start and end of these windows may drift within an epoch over time.
        // NOTE Consequently to remain on the safe side, these values should remain a small fraction of the epoch duration. 
        // NOTE If they are small at most only the last fraction within an epoch will behave differently.
        // TODO Examine the effects of the above.
        minCommitmentAgeForPostconfirmation = _epochDuration / 10;
        postconfirmerPrivilegeDuration = _epochDuration / 10;
        rewardPerAttestationPoint = 1;
        rewardPerPostconfirmationPoint = 1;
    }

    /// @notice Accepts the genesis ceremony.
    function acceptGenesisCeremony() public {
        require(hasRole(DEFAULT_ADMIN_ROLE, msg.sender), "ACCEPT_GENESIS_CEREMONY_IS_ADMIN_ONLY");
        stakingContract.acceptGenesisCeremony();
    }

    /// @notice Sets the postconfirmer term duration, must be less than epoch duration
    /// @param _postconfirmerDuration New postconfirmer term duration in time units
    function setPostconfirmerDuration(uint256 _postconfirmerDuration) public onlyRole(COMMITMENT_ADMIN) {
        // Ensure postconfirmer term is sufficiently small compared to epoch duration
        uint256 epochDuration = stakingContract.getEpochDuration(address(this));

        // TODO If we would use block heights instead of timestamps we could handle everything much smoother.
        if (2 * _postconfirmerDuration >= epochDuration ) {
            revert PostconfirmerDurationTooLongForEpoch();
        }
        postconfirmerDuration = _postconfirmerDuration;
    }

    function getPostconfirmerDuration() public view returns (uint256) {
        return postconfirmerDuration;
    }

    /// @notice Sets the maximum time the postconfirmer can be non-reactive to an honest commitment
    /// @param _postconfirmerPrivilegeDuration  maximum time the postconfirmer is permitted to be non-reactive to an honest commitment
    function setPostconfirmerPrivilegeDuration(uint256 _postconfirmerPrivilegeDuration) public onlyRole(COMMITMENT_ADMIN) {
        // Ensure max privilege time is not too large
        if (_postconfirmerPrivilegeDuration >= stakingContract.getEpochDuration(address(this)) - getMinCommitmentAgeForPostconfirmation()) {
            revert postconfirmerPrivilegeDurationTooLong();
        }
        postconfirmerPrivilegeDuration = _postconfirmerPrivilegeDuration;
    }

    /// @notice Gets the maximum time the postconfirmer can be non-reactive to an honest commitment
    /// @return The maximum time the postconfirmer can be non-reactive to an honest commitment
    function getPostconfirmerPrivilegeDuration() public view returns (uint256) {
        return postconfirmerPrivilegeDuration;
    }

    /// @notice Sets the minimum time that must pass before a commitment can be postconfirmed
    /// @param _minCommitmentAgeForPostconfirmation New minimum commitment age 
    // TODO we also require a check when setting the epoch length that it is larger than the min commitment age
    // TODO we need to set these values such that it works for postconfirmer Term and postconfirmerPrivilegeDuration, etc... there are many constraints here.
    function setMinCommitmentAgeForPostconfirmation(uint256 _minCommitmentAgeForPostconfirmation) public onlyRole(COMMITMENT_ADMIN) {
        // Ensure min age is less than epoch duration to allow postconfirmation within same epoch
        if (_minCommitmentAgeForPostconfirmation >= stakingContract.getEpochDuration(address(this)) - getPostconfirmerPrivilegeDuration()) {
            revert minCommitmentAgeForPostconfirmationTooLong();
        }
        minCommitmentAgeForPostconfirmation = _minCommitmentAgeForPostconfirmation;
    }

    function getMinCommitmentAgeForPostconfirmation() public view returns (uint256) {
        return minCommitmentAgeForPostconfirmation;
    }

    function setOpenAttestationEnabled(bool enabled) public onlyRole(COMMITMENT_ADMIN) {
        openAttestationEnabled = enabled;
    }

    // ----------------------------------------------------------------
    // -------- 3. Epoch & Time Management ---------------------------
    // ----------------------------------------------------------------

    /// @notice Gets the epoch duration
    function getEpochDuration() public view returns (uint256) {
        return stakingContract.getEpochDuration(address(this));
    }

    /// @notice Gets the time at which the current epoch started
    function getEpochStartTime() public view returns (uint256) {
        uint256 currentTime = block.timestamp;
        return currentTime - (currentTime % stakingContract.getEpochDuration(address(this)));
    }

    // gets the present epoch
    function getPresentEpoch() public view returns (uint256) {
        return stakingContract.getEpochByL1BlockTime(address(this));
    }

    // gets the accepting epoch
    function getAcceptingEpoch() public view returns (uint256) {
        return stakingContract.getAcceptingEpoch(address(this));
    }

    // gets the next accepting epoch (unless we are at genesis)
    function getNextAcceptingEpochWithException() public view returns (uint256) {
        return stakingContract.getNextAcceptingEpochWithException(address(this));
    }

    /// @notice Gets the time at which the current postconfirmer's term started
    function getPostconfirmerStartTime() public view returns (uint256) {
        uint256 currentTime = block.timestamp;
        // We reset the times to match the start of epochs. This ensures every epoch has the same setup.
        uint256 currentTimeCorrected = currentTime % stakingContract.getEpochDuration(address(this));
        return currentTimeCorrected - (currentTimeCorrected % postconfirmerDuration);
    }

    /// @notice Determines the postconfirmer in the accepting epoch using L1 block hash as a source of randomness
    // At the border between epochs this is not ideal as getPostconfirmer works on blocks and epochs works with time. 
    // Thus we must consider the edge cases where the postconfirmer is only active for a short time.
    function getPostconfirmer() public view returns (address) {
        // TODO unless we swap with everything, including epochs, we must use block.timestamp. 
        // However, to get easy access to L1 randomness we need to know the block at which the postconfirmer started, which is expensive (unless we count in blocks instead of time)
        // TODO as long as we do not swap to block.number, the randomness below is precictable.
        uint256 randSeed1 = getPostconfirmerStartTime();
        uint256 randSeed2 = getEpochStartTime();
        address[] memory attesters = stakingContract.getStakedAttestersForAcceptingEpoch(address(this));
        if (attesters.length == 0) {
            return address(0);
        }
        uint256 postconfirmerIndex = uint256(keccak256(abi.encodePacked(randSeed1, randSeed2))) % attesters.length; // randomize the order of the attesters
        return attesters[postconfirmerIndex];
    }

    /// @notice Sets the accepting epoch to a new value (must be higher than current)
    /// @param newEpoch The new accepting epoch value
    function setAcceptingEpoch(uint256 newEpoch) external onlyRole(COMMITMENT_ADMIN) {
        // the domain which is the pcp contract must make the call to set the accepting epoch
        stakingContract.setAcceptingEpoch(address(this), newEpoch);
    }

    // ----------------------------------------------------------------
    // -------- 4. Commitment Management ----------
    // ----------------------------------------------------------------

    // creates a commitment
    function createCommitment(
        uint256 height,
        bytes32 commitmentValue,
        bytes32 commitmentId
    ) public pure returns (Commitment memory) {
        return Commitment(height, commitmentValue, commitmentId);
    }

    /// @dev submits a commitment for an attester.
    function submitCommitmentForAttester(
        address attester,
        Commitment memory commitment
    ) internal {
        // Attester has already committed to a commitment at this height
        if (commitments[commitment.height][attester].height != 0)
            revert AttesterAlreadyCommitted();

        // note: do no uncomment the below, we want to allow this in case we have lagging attesters
        // Attester has committed to an already postconfirmed commitment
        // if ( lastPostconfirmedCommitmentHeight > commitment.height) revert AlreadyAcceptedCommitment();
        // Attester has committed to a commitment too far ahead of the last postconfirmed commitment
        if (lastPostconfirmedCommitmentHeight + leadingCommitmentTolerance < commitment.height) {
            revert AttesterAlreadyCommitted();
        }

        // assign the commitment height to the present epoch if it hasn't been assigned yet
        // since any attester can submit a comittment for a commitment height, the epoch assignment could differ 
        // from when the commitment gets actually postconfirmed. This is limited by by leadingCommitmentTolerance
        if (commitmentHeightAssignedEpoch[commitment.height] == 0) {
            commitmentHeightAssignedEpoch[commitment.height] = getPresentEpoch();
        }

        // register the attester's commitment
        commitments[commitment.height][attester] = commitment;
        
        // Record first seen timestamp if not already set
        TrySetCommitmentFirstSeenAt(commitment.height, commitment.commitmentValue, block.timestamp);

        // increment the commitment count by stake
        uint256 attesterStakeForAcceptingEpoch = getAttesterStakeForAcceptingEpoch(attester);
        commitmentStake[commitment.height][commitment.commitmentValue] += attesterStakeForAcceptingEpoch;

        emit CommitmentSubmitted(
            commitment.commitmentId,
            commitment.commitmentValue,
            attesterStakeForAcceptingEpoch
        );
    }
    function submitCommitment(Commitment memory commitment) external {
        require(
            openAttestationEnabled || hasRole(TRUSTED_ATTESTER, msg.sender),
            "UNAUTHORIZED_COMMITMENT"
        );
        submitCommitmentForAttester(msg.sender, commitment);
    }

    function submitBatchCommitment(Commitment[] memory commitments) public {
        require(
            openAttestationEnabled || hasRole(TRUSTED_ATTESTER, msg.sender),
            "UNAUTHORIZED_COMMITMENT"
        );
        for (uint256 i = 0; i < commitments.length; i++) {
            submitCommitmentForAttester(msg.sender, commitments[i]);
        }
    }
    
    function getValidatorCommitmentAtCommitmentHeight(
        uint256 height,
        address attester
    ) public view returns (Commitment memory) {
        return commitments[height][attester];
    }

    // gets the max tolerable commitment height
    function getMaxTolerableCommitmentHeight() public view returns (uint256) {
        return lastPostconfirmedCommitmentHeight + leadingCommitmentTolerance;
    }
    /// @notice Gets the commitment submitted by an attester for a given height
    function getCommitmentByAttester(uint256 height, address attester) public view returns (Commitment memory) {
        return commitments[height][attester];
    }

    /// @notice Gets the epoch assigned to a superblock height
    function getCommitmentHeightAssignedEpoch(uint256 height) public view returns (uint256) {
        return commitmentHeightAssignedEpoch[height];
    }

    // TODO use this to limit the postconfirmations on new commits ( we need to give time to attesters to submit their commitments )
    /// @notice get the timestamp when a commitment was first seen
    function getCommitmentFirstSeenAt(Commitment memory commitment) public view returns (uint256) {
        return commitmentFirstSeenAt[commitment.height][commitment.commitmentValue];
    }

    /// @notice Sets the timestamp when a commitment was first seen
    function TrySetCommitmentFirstSeenAt(uint256 height, bytes32 commitmentValue, uint256 timestamp) internal {
        if (commitmentFirstSeenAt[height][commitmentValue] != 0) {
            // do not set if already set
            return;
        } else if (timestamp == 0) {
            // no need to set if timestamp is 0. This if may be redundant though.
            return;
        }
        commitmentFirstSeenAt[height][commitmentValue] = timestamp;
    }

    // ----------------------------------------------------------------
    // -------- 5. Postconfirmation and Rollover Management ----------
    // ----------------------------------------------------------------

    /// @notice Gets the height of the last postconfirmed superblock
    function getLastPostconfirmedCommitmentHeight() public view returns (uint256) {
        return lastPostconfirmedCommitmentHeight;
    }

    function postconfirmCommitmentsAndRollover() public {
        postconfirmAndRolloverWithAttester(msg.sender);
    }

    /// @notice The current postconfirmer can postconfirm a commitment height, given there is a supermajority of stake on a commitment
    /// @notice If the current postconfirmer is live, we should not accept postconfirmations from voluntary attesters
    // TODO: this will be improved, such that voluntary attesters can postconfirm but will not be rewarded before the liveness period has ended
    /// @notice If the current postconfirmer is not live, we should accept postconfirmations from any attester
    // TODO: this will be improved, such that the first voluntary attester to do sowill be rewarded
    function postconfirmAndRolloverWithAttester(address /* attester */) internal {

        // keep ticking through postconfirmations and rollovers as long as the postconfirmer is permitted to do
        // ! rewards need to be 
        // ! - at least the cost for gas cost of postconfirmation
        // ! - reward the postconfirmer well to incentivize postconfirmation at every height
        while (attemptPostconfirmOrRollover(lastPostconfirmedCommitmentHeight + 1)) {
        }
    }

    // Sets the postconfirmed commitment at a given commitment height
    function setPostconfirmedCommitmentAtBlockHeight(Commitment memory commitment) public {
        require(
            hasRole(COMMITMENT_ADMIN, msg.sender),
            "SET_LAST_POSTCONFIRMED_COMMITMENT_AT_HEIGHT_IS_COMMITMENT_ADMIN_ONLY"
        );
        versionedPostconfirmedCommitments[postconfirmedCommitmentsVersion][commitment.height] = commitment;  
    }

    // Forces the latest attestation by setting the commitment height
    // Note: this only safe when we are running with a single validator as it does not zero out follow-on commitments.
    function forceLatestCommitment(Commitment memory commitment) public {
        require(
            hasRole(COMMITMENT_ADMIN, msg.sender),
            "FORCE_LATEST_COMMITMENT_IS_COMMITMENT_ADMIN_ONLY"
        );
        setPostconfirmedCommitmentAtBlockHeight(commitment);
    }

    function getPostconfirmedCommitment(uint256 height) public view returns (Commitment memory) {
        return versionedPostconfirmedCommitments[postconfirmedCommitmentsVersion][height];
    }
    /// @dev Postconfirms a commitment.
    /// @dev This function and attemptPostconfirmOrRollover() could call each other recursively, so we must ensure it's safe from re-entrancy
    function _postconfirmCommitment(Commitment memory commitment, address attester) internal {
        uint256 currentAcceptingEpoch = getAcceptingEpoch();
        
        // get the epoch for the commitment
        // Commitment is not in the current epoch, it cannot be postconfirmed. 
        // TODO: double check liveness conditions for the following critera
        if (commitmentHeightAssignedEpoch[commitment.height] != currentAcceptingEpoch) {
            revert UnacceptableCommitment();
        }

        // ensure that the lastPostconfirmedCommitmentHeight is exactly the commitment height - 1
        if (lastPostconfirmedCommitmentHeight != commitment.height - 1) {
            revert UnacceptableCommitment();
        }

        // Record reward points for all attesters who committed to the winning commitment
        address[] memory attesters = getStakedAttestersForAcceptingEpoch();
        for (uint256 i = 0; i < attesters.length; i++) {
            if (commitments[commitment.height][attesters[i]].commitmentValue == commitment.commitmentValue) {
                attesterRewardPoints[currentAcceptingEpoch][attesters[i]]++;
            }
        }

        // Award points to postconfirmer
        if (!isWithinPostconfirmerPrivilegeDuration(commitment)) { 
            // if we are outside the privilege window, for the postconfirmer reward anyone who postconfirms
            postconfirmerRewardPoints[currentAcceptingEpoch][attester] += 1;
        } else {
            // if we are within the privilege window, only award points to the postconfirmer
            // TODO optimization: even if the height has been volunteer postconfirmed we need to allow that that postconfirmer gets rewards, 
            // TODO otherwise weak postconfirmers may could get played (rich volunteer postconfirmers pay the fees and poor postconfirmers never get any reward) 
            // TODO but check if this is really required game theoretically.
            if (getPostconfirmer() == attester) {
                postconfirmerRewardPoints[currentAcceptingEpoch][attester] += 1;
            }
        }

        versionedPostconfirmedCommitments[postconfirmedCommitmentsVersion][commitment.height] = commitment;
        lastPostconfirmedCommitmentHeight = commitment.height;
        postconfirmedBy[commitment.height] = attester;
        postconfirmedAtL1BlockHeight[commitment.height] = block.number;
        postconfirmedAtL1BlockTimestamp[commitment.height] = block.timestamp;

        // emit the commitment postconfirmed event
        emit CommitmentPostconfirmed(
            commitment.commitmentId,
            commitment.commitmentValue,
            commitment.height
        );
    }

    /// @dev nonReentrant because there is no need to reenter this function. It should be called iteratively. 
    /// @dev Marked on the internal method to simplify risks from complex calling patterns. This also calls an external contract.
    function rollOverEpoch() internal {
        // Get all attesters who earned points in the current epoch
        uint256 acceptingEpoch = getAcceptingEpoch();
        address[] memory attesters = getStakedAttestersForAcceptingEpoch();
        
        // reward
        for (uint256 i = 0; i < attesters.length; i++) {
            if (attesterRewardPoints[acceptingEpoch][attesters[i]] > 0) {
                // TODO: make this configurable and set it on instance creation
                uint256 reward = attesterRewardPoints[acceptingEpoch][attesters[i]] * rewardPerAttestationPoint * getAttesterStakeForAcceptingEpoch(attesters[i]);
                // the staking contract is the custodian
                // rewards are currently paid out from the pcp domain
                stakingContract.rewardFromDomain(attesters[i], reward, moveTokenAddress);
                // TODO : check if we really have to keep attesterRewardPoints per epoch, or whether we could simply delete the points here for a given attester.
            }

            // Add postconfirmation rewards
            if (postconfirmerRewardPoints[acceptingEpoch][attesters[i]] > 0) {
                uint256 reward = postconfirmerRewardPoints[acceptingEpoch][attesters[i]] * rewardPerPostconfirmationPoint * getAttesterStakeForAcceptingEpoch(attesters[i]);
                stakingContract.rewardFromDomain(attesters[i], reward, moveTokenAddress);
                // TODO : check if we really have to keep postconfirmerRewardPoints per epoch, or whether we could simply delete the points here for a given postconfirmer.
                // TODO also the postconfirmer list is super short. typically for a given height only the postconfirmer and at most the postconfirmer and a volunteer postconfirmer.
                // TODO So this can be heavily optimized.
            }
        }

        stakingContract.rollOverEpoch();
    }

    /// @notice Checks, for a given commitment, if the current L1 block time is within the postconfirmer's privilege window
    /// @dev The postconfirmer's privilege window is the time period when only the postconfirmer will get rewarded for postconfirmation
    function isWithinPostconfirmerPrivilegeDuration(Commitment memory commitment) public view returns (bool) {
        if (getCommitmentFirstSeenAt(commitment) == 0) {
            return false;
        }
        // based on the first timestamp for the commitment we can determine if the postconfirmer has been live sufficiently recently
        // use getCommitmentFirstSeenAt, and the mappings
        if (getCommitmentFirstSeenAt(commitment) 
            + getMinCommitmentAgeForPostconfirmation() 
            + getPostconfirmerPrivilegeDuration() 
            < block.timestamp) {
            return false;
        }
        return true;
    }

    /// @dev it is possible if the accepting epoch is behind the presentEpoch that heights dont obtain enough votes in the assigned epoch. 
    /// @dev Moreover, due to the leadingCommitmentTolerance, the assigned epoch for a height could be ahead of the actual epoch. 
    /// @dev solution is to move to the next epoch and count votes there
    function attemptPostconfirmOrRollover(uint256 commitmentHeight) internal returns (bool) {
        uint256 commitmentEpoch = commitmentHeightAssignedEpoch[commitmentHeight];
        if (getLastPostconfirmedCommitmentHeight() == 0) {
            // if there is no postconfirmed superblock we are at genesis
        } else {
            // ensure that the commitment height is equal or above the lastPostconfirmedCommitmentHeight
            uint256 previousCommitmentEpoch = commitmentHeightAssignedEpoch[commitmentHeight-1];
            if (commitmentEpoch < previousCommitmentEpoch  )  {
                address[] memory stakedAttesters = getStakedAttestersForAcceptingEpoch();
                // if there is at least one commitment at this commitment height, we need to update once
                for (uint256 i = 0; i < stakedAttesters.length; i++) {
                    if (commitments[commitmentHeight][stakedAttesters[i]].height != 0) {
                        commitmentHeightAssignedEpoch[commitmentHeight] = previousCommitmentEpoch;
                        break;
                    }
                }
                commitmentEpoch = previousCommitmentEpoch;
            }
        }

        // if the accepting epoch is far behind the commitmentEpoch (which is determined by commitments measured in L1 block time), then the protocol was not live for a while
        // We keep rolling over the epoch (i.e. update stakes) until we catch up with the present epoch
        while (getAcceptingEpoch() < commitmentEpoch) {
            // TODO only permit rollover after some liveness criteria for the postconfirmer, as this is related to the reward model (rollovers should be rewarded)
            rollOverEpoch();
        }

        // TODO only permit postconfirmation after some liveness criteria for the postconfirmer, as this is related to the reward model (postconfirmation should be rewarded)

        uint256 supermajority = (2 * getTotalStake(commitmentEpoch)) / 3 + 1;
        address[] memory attesters = getStakedAttestersForAcceptingEpoch();

        // iterate over the attester set
        // TODO: randomize the order in which we check the attesters, which helps against spam of commitments. 
        // TODO: it may be more elegant to go through the commitments rather than the attesters..
        bool successfulPostconfirmation = false;
        for (uint256 i = 0; i < attesters.length; i++) {
            address attester = attesters[i];
            Commitment memory commitment = commitments[commitmentHeight][attester];
            // check if the commitment has committed to the correct commitment height
            // TODO: possibly this is not needed and we can remove the height from the commitment?
            if (commitment.height != commitmentHeight) continue;

            // check the total stake on the commitment
            uint256 totalStakeOnCommitment = commitmentStake[commitment.height][commitment.commitmentValue];

            if (totalStakeOnCommitment >= supermajority) {
                // Check if enough time has passed since commitment was first seen
                // if not enough time has passed, then no postconfirmation at this height can yet happen
                uint256 firstSeen = getCommitmentFirstSeenAt(commitment);
                // we should jump out of the for loop entirely
                if (block.timestamp < firstSeen + minCommitmentAgeForPostconfirmation) break;

                _postconfirmCommitment(commitment, msg.sender);
                successfulPostconfirmation = true;

                // TODO: for rewards we have to run through all the attesters, as we need to acknowledge that they get rewards. 

                // TODO: if the attester is the current postconfirmer, we need to record that the postconfirmer has shown liveness. 
                // TODO: this liveness needs to be discoverable by isWithinPostconfirmerPrivilegeDuration()

                return true;
            }
        }
        // if there was no supermajority for any commitment at that height it means that the attesters were not sufficiently live
        // we rollover the epoch to give the next attesters a chance
        if (!successfulPostconfirmation && getPresentEpoch() > getAcceptingEpoch()) {
            rollOverEpoch();
            return true; // we have to retry the postconfirmation at the next epoch again
        }
        return false;
    }

    // ----------------------------------------------------------------
    // -------- 6. Stake, Rewards & Slashing Mechanisms --------------
    // ----------------------------------------------------------------

    /// @notice Gets the stake for a given tuple (custodian, attester) at a given epoch
    function getStake(
        uint256 epoch,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return
            stakingContract.getStake(
                address(this),
                epoch,
                custodian,
                attester
            );
    }

    /// @notice Gets the stake for a given tuple (custodian, attester) at the accepting epoch
    function getStakeForAcceptingEpoch(
        address custodian,
        address attester
    ) public view returns (uint256) {
        return getStake(getAcceptingEpoch(), custodian, attester);
    }

    /// @notice Gets the stake for a given attester at a given epoch
    // TODO: memorize this (<-- ? as in create a mapping?)
    function getAttesterStake(
        uint256 epoch,
        address attester
    ) public view returns (uint256) {
        address[] memory custodians = stakingContract.getRegisteredCustodians(
            address(this)
        );
        uint256 totalStake = 0;
        for (uint256 i = 0; i < custodians.length; i++) {
            // for now, each custodian has weight of 1
            totalStake += getStake(epoch, custodians[i], attester);
        }
        return totalStake;
    }

    /// @notice Gets the stake for a given attester at the accepting epoch
    function getAttesterStakeForAcceptingEpoch(
        address attester
    ) public view returns (uint256) {
        return getAttesterStake(getAcceptingEpoch(), attester);
    }

    /// @notice Gets the stake for a given custodian for a given epoch
    function getCustodianStake(
        uint256 epoch,
        address custodian
    ) public view returns (uint256) {
        return
            stakingContract.getCustodianStake(
                address(this), // domain
                epoch,
                custodian
            );
    }

    function getTotalStake(
        uint256 epoch
    ) public view returns (uint256) {
        // we can either use the attesterStake or the custodianStake
        // the sums of attesterStake and custodianStake should equal the same value
        address[] memory custodians = stakingContract.getRegisteredCustodians(
            address(this)
        );
        uint256 totalStake = 0;
        for (uint256 i = 0; i < custodians.length; i++) {
            // for now, each custodian has weight of 1
            totalStake += getCustodianStake(epoch, custodians[i]);
        }
        return totalStake;
    }

    // gets the total stake for the current epoch for a given custodian
    function getCustodianStakeForAcceptingEpoch(
        address custodian
    ) public view returns (uint256) {
        return getCustodianStake(getAcceptingEpoch(), custodian);
    }

    function getTotalStakeForAcceptingEpoch()
        public
        view
        returns (uint256)
    {
        return getTotalStake(getAcceptingEpoch());
    }

    function setRewardPerAttestationPoint(uint256 rewardPerPoint) public onlyRole(COMMITMENT_ADMIN) {
        rewardPerAttestationPoint = rewardPerPoint;
    }

    function setRewardPerPostconfirmationPoint(uint256 rewardPerPoint) public onlyRole(COMMITMENT_ADMIN) {
        rewardPerPostconfirmationPoint = rewardPerPoint;
    }

    /// @notice Gets the reward points for an attester in a given epoch
    function getAttesterRewardPoints(uint256 epoch, address attester) public view returns (uint256) {
        return attesterRewardPoints[epoch][attester];
    }

        /// @notice Gets the reward points for a postconfirmer in a given epoch
    function getPostconfirmerRewardPoints(uint256 epoch, address postconfirmer) public view returns (uint256) {
        return postconfirmerRewardPoints[epoch][postconfirmer];
    }

    /// @notice Gets the attesters who have stake in the current accepting epoch
    function getStakedAttestersForAcceptingEpoch() public view returns (address[] memory) {
        return stakingContract.getStakedAttestersForAcceptingEpoch(address(this)); 
    }

    function isCommitted(uint256 height) external view returns (bool) {
        return commitments[height][msg.sender].height != 0;
    }

    function isPostconfirmed(uint256 height) external view returns (bool) {
        return versionedPostconfirmedCommitments[postconfirmedCommitmentsVersion][height].height != 0;
    }

}

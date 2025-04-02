// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "forge-std/console.sol";
import {BaseStaking} from "./base/BaseStaking.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {ICustodianToken} from "../token/custodian/CustodianToken.sol";
import {Math} from "@openzeppelin/contracts/utils/math/Math.sol";
import {MovementStakingStorage, EnumerableSet} from "./MovementStakingStorage.sol";
import {IMovementStaking} from "./interfaces/IMovementStaking.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";

/**
 * @title MovementStaking
 * @notice Contract that manages staking and unstaking of tokens across multiple domains
 * @dev Implements epoch-based staking with support for multiple custodians per domain
 */
contract MovementStaking is
    MovementStakingStorage,
    IMovementStaking,
    BaseStaking,
    ReentrancyGuard
{
    using EnumerableSet for EnumerableSet.AddressSet;

    /// @notice Mapping of domain addresses to their epoch durations
    mapping(address => uint256) internal epochDurations;

    /**
     * @notice Initializes the staking contract
     * @param _token The token to be staked
     */
    function initialize(IERC20 _token) public initializer {
        __BaseStaking_init_unchained();
        token = _token;
    }

    /**
     * @notice Returns the token being staked
     * @return The token being staked
     */
    function getToken() external view returns (IERC20) {
        return token;
    }

    /**
     * @notice Registers a domain with the staking contract
     * @param epochDuration Duration of each epoch for this domain
     * @param custodians Array of custodian addresses for this domain
     */
    function registerDomain(
        uint256 epochDuration,
        address[] calldata custodians
    ) external nonReentrant {
        address domain = msg.sender;
        epochDurationByDomain[domain] = epochDuration;

        for (uint256 i = 0; i < custodians.length; i++) {
            custodiansByDomain[domain].add(custodians[i]);
        }
    }

    /**
     * @notice Gets all custodians for a specific domain
     * @dev TODO: Consider improving the API to allow domains to interpret custodians as they see fit
     * @param domain The domain address
     * @return Array of custodian addresses
     */
    function getCustodiansByDomain(
        address domain
    ) public view returns (address[] memory) {
        address[] memory custodians = new address[](
            custodiansByDomain[domain].length()
        );
        for (uint256 i = 0; i < custodiansByDomain[domain].length(); i++) {
            custodians[i] = custodiansByDomain[domain].at(i);
        }
        return custodians;
    }

    /**
     * @notice Gets all attesters for a specific domain
     * @param domain The domain address
     * @return Array of attester addresses
     */
    function getAttestersByDomain(
        address domain
    ) public view returns (address[] memory) {
        address[] memory attesters = new address[](
            attestersByDomain[domain].length()
        );
        for (uint256 i = 0; i < attestersByDomain[domain].length(); i++) {
            attesters[i] = attestersByDomain[domain].at(i);
        }
        return attesters;
    }

    /**
     * @notice Accepts the genesis ceremony for a domain
     * @dev Rolls over from genesis (epoch 0) to current epoch and sets initial stakes
     */
    function acceptGenesisCeremony() public nonReentrant {
        address domain = msg.sender;
        if (domainGenesisAccepted[domain]) revert GenesisAlreadyAccepted();
        domainGenesisAccepted[domain] = true;
        
        // Roll over from 0 (genesis) to current epoch by block time
        currentEpochByDomain[domain] = getEpochByBlockTime(domain);

        for (uint256 i = 0; i < attestersByDomain[domain].length(); i++) {
            address attester = attestersByDomain[domain].at(i);

            for (uint256 j = 0; j < custodiansByDomain[domain].length(); j++) {
                address custodian = custodiansByDomain[domain].at(j);

                // Get the genesis stake for the attester
                uint256 attesterStake = getStakeAtEpoch(
                    domain,
                    0,
                    custodian,
                    attester
                );

                // Roll over the genesis stake to the current epoch
                _addStake(
                    domain,
                    getCurrentEpoch(domain),
                    custodian,
                    attester,
                    attesterStake
                );
            }
        }
    }

    /**
     * @notice Adds stake for an attester
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @param amount The amount to stake
     */
    function _addStake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        epochStakesByDomain[domain][epoch][custodian][attester] += amount;
        epochTotalStakeByDomain[domain][epoch][custodian] += amount;
    }

    /**
     * @notice Removes stake from an attester
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @param amount The amount to remove
     */
    function _removeStake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        epochStakesByDomain[domain][epoch][custodian][attester] -= amount;
        epochTotalStakeByDomain[domain][epoch][custodian] -= amount;
    }

    /**
     * @notice Adds an unstake request for an attester
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @param amount The amount to unstake
     */
    function _addUnstake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        epochUnstakesByDomain[domain][epoch][custodian][attester] += amount;
    }

    /**
     * @notice Removes an unstake request for an attester
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @param amount The amount to remove from unstake
     */
    function _removeUnstake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        epochUnstakesByDomain[domain][epoch][custodian][attester] -= amount;
    }

    /**
     * @notice Sets the unstake amount for an attester
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @param amount The amount to set
     */
    function _setUnstake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        epochUnstakesByDomain[domain][epoch][custodian][attester] = amount;
    }

    /**
     * @notice Gets the epoch for the current block time
     * @param domain The domain address
     * @return uint256 Current epoch based on block time
     */
    function getEpochByBlockTime(address domain) public view returns (uint256) {
        return block.timestamp / epochDurationByDomain[domain];
    }

    /**
     * @notice Gets the current epoch up to which blocks have been accepted
     * @param domain The domain address
     * @return uint256 Current epoch
     */
    function getCurrentEpoch(address domain) public view returns (uint256) {
        return currentEpochByDomain[domain];
    }

    /**
     * @notice Gets the next epoch
     * @param domain The domain address
     * @return uint256 Next epoch 
     */
    function getNextEpoch(address domain) public view returns (uint256) {
        return getCurrentEpoch(domain) == 0 ? 0 : getCurrentEpoch(domain) + 1;
    }

    /**
     * @notice Gets the next epoch based on block time
     * @param domain The domain address
     * @return uint256 Next epoch by block time
     */
    function getNextEpochByBlockTime(
        address domain
    ) public view returns (uint256) {
        return
            getCurrentEpoch(domain) == 0 ? 0 : getEpochByBlockTime(domain) + 1;
    }

    /**
     * @notice Gets the stake for a given attester at a specific epoch
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Stake amount
     */
    function getStakeAtEpoch(
        address domain,
        uint256 epoch,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return epochStakesByDomain[domain][epoch][custodian][attester];
    }

    /**
     * @notice Gets the stake for a given attester at the current epoch
     * @param domain The domain address
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Current epoch stake
     */
    function getCurrentEpochStake(
        address domain,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return
            getStakeAtEpoch(
                domain,
                getCurrentEpoch(domain),
                custodian,
                attester
            );
    }

    /**
     * @notice Gets the unstake amount for a given attester at a specific epoch
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Unstake amount
     */
    function getUnstakeAtEpoch(
        address domain,
        uint256 epoch,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return epochUnstakesByDomain[domain][epoch][custodian][attester];
    }

    /**
     * @notice Gets the unstake amount for a given attester at the current epoch
     * @param domain The domain address
     * @param custodian The custodian address
     * @param attester The attester address
     * @return uint256 Current epoch unstake amount
     */
    function getCurrentEpochUnstake(
        address domain,
        address custodian,
        address attester
    ) public view returns (uint256) {
        return
            getUnstakeAtEpoch(
                domain,
                getCurrentEpoch(domain),
                custodian,
                attester
            );
    }

    /**
     * @notice Gets the total stake for a given epoch and custodian
     * @param domain The domain address
     * @param epoch The epoch number
     * @param custodian The custodian address
     * @return uint256 Total stake for the epoch
     */
    function getTotalStakeForEpoch(
        address domain,
        uint256 epoch,
        address custodian
    ) public view returns (uint256) {
        return epochTotalStakeByDomain[domain][epoch][custodian];
    }

    /**
     * @notice Gets the total stake for the current epoch and custodian
     * @param domain The domain address
     * @param custodian The custodian address
     * @return uint256 Total stake for current epoch
     */
    function getTotalStakeForCurrentEpoch(
        address domain,
        address custodian
    ) public view returns (uint256) {
        return
            getTotalStakeForEpoch(domain, getCurrentEpoch(domain), custodian);
    }

    /**
     * @notice Internal helper for staking tokens
     * @param domain The domain to stake for
     * @param attester The attester to stake for
     * @param custodian The custodian token contract
     * @param amount The amount to stake
     * @param from The address to transfer tokens from
     */
    function _stake(
        address domain,
        address attester,
        IERC20 custodian,
        uint256 amount,
        address from
    ) internal {
        // Add the attester to the list of attesters
        attestersByDomain[domain].add(attester);

        // Check the balance of the token before transfer
        uint256 balanceBefore = token.balanceOf(address(this));

        // Transfer the stake to the contract
        custodian.transferFrom(from, address(this), amount);

        // Require that the balance of the actual token has increased by the amount
        if (token.balanceOf(address(this)) != balanceBefore + amount)
            revert CustodianTransferAmountMismatch();

        // Set the attester to stake for the next epoch
        _addStake(
            domain,
            getNextEpochByBlockTime(domain),
            address(custodian),
            attester,
            amount
        );

        // Emit stake event
        emit AttesterStaked(
            domain,
            getNextEpoch(domain),
            address(custodian),
            attester,
            amount
        );
    }

    /**
     * @notice Internal helper for unstaking tokens
     * @param domain The domain to unstake from
     * @param attester The attester to unstake for
     * @param custodian The custodian address
     * @param amount The amount to unstake
     */
    function _unstake(
        address domain,
        address attester,
        address custodian,
        uint256 amount
    ) internal {
        // Mark tokens for unstaking in the next epoch
        _addUnstake(
            domain,
            getNextEpochByBlockTime(domain),
            custodian,
            attester,
            amount
        );

        emit AttesterUnstaked(
            domain,
            getNextEpoch(domain),
            custodian,
            attester,
            amount
        );
    }

    /**
     * @notice Stakes tokens for the next epoch
     * @param domain The domain to stake for
     * @param custodian The custodian token contract
     * @param amount The amount to stake
     */
    function stake(
        address domain,
        IERC20 custodian,
        uint256 amount
    ) external onlyRole(WHITELIST_ROLE) nonReentrant {

        // approve the staking contract to spend the tokens
        custodian.approve(address(this), amount);
        _stake(domain, msg.sender, custodian, amount, msg.sender);
    }

    /**
     * @notice Allows a domain to stake tokens for a user under its own domain
     * @param user The user to stake for
     * @param custodian The custodian token contract
     * @param amount The amount to stake
     */
    function stakeFor(address user, IERC20 custodian, uint256 amount) external {
        address domain = msg.sender;
        require(epochDurationByDomain[domain] > 0, "DOMAIN_NOT_REGISTERED");
        _stake(domain, user, custodian, amount, domain);
    }

    /**
     * @notice Initiates unstaking of tokens for the next epoch
     * @dev This doesn't actually unstake immediately, but marks tokens for unstaking in the next epoch
     * @param domain The domain to unstake from
     * @param custodian The custodian address
     * @param amount The amount to unstake
     */
    function unstake(
        address domain,
        address custodian,
        uint256 amount
    ) external onlyRole(WHITELIST_ROLE) nonReentrant {
        _unstake(domain, msg.sender, custodian, amount);
    }

    /**
     * @notice Allows a domain to unstake tokens for a user under its own domain
     * @param user The user to unstake for
     * @param custodian The custodian address
     * @param amount The amount to unstake
     */
    function unstakeFor(address user, address custodian, uint256 amount) external {
        address domain = msg.sender;
        require(epochDurationByDomain[domain] > 0, "DOMAIN_NOT_REGISTERED");
        _unstake(domain, user, custodian, amount);
    }

    /**
     * @notice Rolls over staking and unstaking for a specific attester
     * @param domain The domain address
     * @param epochNumber The current epoch number
     * @param custodian The custodian address
     * @param attester The attester address
     */
    function _rollOverAttester(
        address domain,
        uint256 epochNumber,
        address custodian,
        address attester
    ) internal {
        // Calculate the amount of stake to roll over: stake[currentEpoch] - unstake[nextEpoch]
        uint256 stakeAmount = getStakeAtEpoch(
            domain,
            epochNumber,
            custodian,
            attester
        );
        uint256 unstakeAmount = getUnstakeAtEpoch(
            domain,
            epochNumber + 1,
            custodian,
            attester
        );
        if (unstakeAmount > stakeAmount) {
            unstakeAmount = stakeAmount;
        }
        uint256 remainder = stakeAmount - unstakeAmount;

        _addStake(domain, epochNumber + 1, custodian, attester, remainder);

        // Process unstaking payout
        // @dev This is the only place unstaking happens - no risk of double payout
        // as long as rollOverAttester is only called once per epoch
        _payAttester(address(this), attester, custodian, unstakeAmount);

        emit AttesterEpochRolledOver(
            attester,
            epochNumber,
            custodian,
            stakeAmount,
            unstakeAmount
        );
    }

    /**
     * @notice Rolls over an epoch for a domain
     * @param domain The domain address
     * @param epochNumber The epoch number to roll over
     */
    function _rollOverEpoch(address domain, uint256 epochNumber) internal {
        // Process all attesters and custodians
        // @dev Complexity could be reduced by mapping attesters to their token and custodian
        for (uint256 i = 0; i < attestersByDomain[domain].length(); i++) {
            address attester = attestersByDomain[domain].at(i);

            for (uint256 j = 0; j < custodiansByDomain[domain].length(); j++) {
                address custodian = custodiansByDomain[domain].at(j);

                _rollOverAttester(domain, epochNumber, custodian, attester);
            }
        }

        // Increment the current epoch
        currentEpochByDomain[domain] = epochNumber + 1;

        emit EpochRolledOver(domain, epochNumber);
    }

    /**
     * @notice Public function to roll over the current epoch
     */
    function rollOverEpoch() external {
        _rollOverEpoch(msg.sender, getCurrentEpoch(msg.sender));
    }

    /**
     * @notice Slashes an attester's stake
     * @param domain The domain of the attester
     * @param epoch The epoch in which the slash is attempted
     * @param custodian The custodian of the token
     * @param attester The attester to slash
     * @param amount The amount to slash
     */
    function _slashStake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester,
        uint256 amount
    ) internal {
        // Slash will always target the current epoch
        uint256 targetEpoch = epoch;
        uint256 stakeForEpoch = getStakeAtEpoch(
            domain,
            targetEpoch,
            custodian,
            attester
        );

        // Deduct the amount from the attester's stake, accounting for underflow
        if (stakeForEpoch < amount) {
            _removeStake(
                domain,
                targetEpoch,
                custodian,
                attester,
                stakeForEpoch
            );
        } else {
            _removeStake(domain, targetEpoch, custodian, attester, amount);
        }
    }

    /**
     * @notice Slashes an attester's unstake request
     * @param domain The domain of the attester
     * @param epoch The epoch in which the slash is attempted
     * @param custodian The custodian of the token
     * @param attester The attester to slash
     */
    function _slashUnstake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester
    ) internal {
        // Unstake slash targets the next epoch
        uint256 stakeForEpoch = getStakeAtEpoch(
            domain,
            epoch,
            custodian,
            attester
        );
        uint256 targetEpoch = epoch + 1;
        uint256 unstakeForEpoch = getUnstakeAtEpoch(
            domain,
            targetEpoch,
            custodian,
            attester
        );

        if (unstakeForEpoch > stakeForEpoch) {
            // If attester is trying to unstake more than is staked,
            // set the unstake to the maximum possible amount
            _setUnstake(
                domain,
                targetEpoch,
                custodian,
                attester,
                stakeForEpoch
            );
        }
    }

    /**
     * @notice Slashes multiple attesters
     * @param custodians Array of custodian addresses
     * @param attesters Array of attester addresses
     * @param amounts Array of amounts to slash
     * @param refundAmounts Array of refund amounts
     */
    function slash(
        address[] calldata custodians,
        address[] calldata attesters,
        uint256[] calldata amounts,
        uint256[] calldata refundAmounts
    ) public nonReentrant {
        for (uint256 i = 0; i < attesters.length; i++) {
            // Calculate refund as minimum of stake balance, slash amount, and refund amount
            // This prevents a Domain from having this contract pay more than has been staked
            uint256 refundAmount = Math.min(
                getStakeAtEpoch(
                    msg.sender,
                    getCurrentEpoch(attesters[i]),
                    custodians[i],
                    attesters[i]
                ),
                Math.min(amounts[i], refundAmounts[i])
            );
            _payAttester(
                address(this), // Contract is paying the attester
                attesters[i],
                custodians[i],
                refundAmount
            );

            // Slash both stake and unstake to reduce attester weight and prevent withdrawals
            _slashStake(
                msg.sender,
                getCurrentEpoch(msg.sender),
                custodians[i],
                attesters[i],
                amounts[i]
            );

            _slashUnstake(
                msg.sender,
                getCurrentEpoch(msg.sender),
                custodians[i],
                attesters[i]
            );
        }
    }

    /**
     * @notice Pays an attester with tokens
     * @param from The address paying the attester
     * @param attester The attester receiving payment
     * @param custodian The custodian address
     * @param amount The amount to pay
     */
    function _payAttester(
        address from,
        address attester,
        address custodian,
        uint256 amount
    ) internal {
        if (from == address(this)) {
            // Contract is paying the attester
            if (address(token) == custodian) {
                // Direct token transfer if no custodian
                token.transfer(attester, amount);
            } else {
                // Approve custodian to spend base token
                token.approve(custodian, amount);

                // Purchase custodial token for the attester
                ICustodianToken(custodian).buyCustodialToken(attester, amount);
            }
        } else {
            // External address is paying the attester
            // @dev Domain could reward the attester directly; this provides Ricardian clarity
            if (address(token) == custodian) {
                // Direct token transfer if no custodian
                token.transferFrom(from, attester, amount);
            } else {
                // Purchase custodial token for the attester
                ICustodianToken(custodian).buyCustodialTokenFrom(
                    from,
                    attester,
                    amount
                );
            }
        }
    }

    /**
     * @notice Rewards multiple attesters
     * @dev Could be used to automatically add to attesters' stake with a restake policy
     * @param attesters Array of attester addresses
     * @param amounts Array of reward amounts
     * @param custodians Array of custodian addresses
     */
    function reward(
        address[] calldata attesters,
        uint256[] calldata amounts,
        address[] calldata custodians
    ) public nonReentrant {
        for (uint256 i = 0; i < attesters.length; i++) {
            // Pay the attester
            _payAttester(msg.sender, attesters[i], custodians[i], amounts[i]);
        }
    }

    /**
     * @notice Adds an address to the whitelist
     * @param addr The address to whitelist
     */
    function whitelistAddress(
        address addr
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        grantRole(WHITELIST_ROLE, addr);
    }

    /**
     * @notice Removes an address from the whitelist
     * @param addr The address to remove
     */
    function removeAddressFromWhitelist(
        address addr
    ) external onlyRole(DEFAULT_ADMIN_ROLE) {
        revokeRole(WHITELIST_ROLE, addr);
    }
}

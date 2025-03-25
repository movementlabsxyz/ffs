// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;
import "forge-std/console.sol";
import { SafeERC20 } from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import { IERC20 } from "@openzeppelin/contracts/interfaces/IERC20.sol";
import { EnumerableSet } from "@openzeppelin/contracts/utils/structs/EnumerableSet.sol";

/**
 * @title MovementStakingStorage
 * @notice Storage contract for the Movement staking system
 * @dev Contains state variables and mappings related to staking, unstaking, and epoch management
 */
contract MovementStakingStorage {

    using SafeERC20 for IERC20;
    using EnumerableSet for EnumerableSet.AddressSet;

    /// @notice The token used for staking
    IERC20 public token;

    /// @notice Mapping of domain to epoch duration
    mapping(address domain => uint256 epochDuration) public epochDurationByDomain;
    
    /// @notice Mapping of domain to current epoch
    mapping(address domain => uint256 currentEpoch) public currentEpochByDomain;
    
    /// @notice Mapping of domain to set of attesters
    mapping(address domain => EnumerableSet.AddressSet attester) internal attestersByDomain;
    
    /// @notice Mapping of domain to set of custodians
    mapping(address domain => EnumerableSet.AddressSet custodian) internal custodiansByDomain;

    /// @notice Records of stake by address per epoch for each domain and custodian
    mapping(address domain => 
        mapping(uint256 epoch => 
            mapping(address custodian => 
                mapping(address attester => uint256 stake)))) public epochStakesByDomain;

    /// @notice Records of unstake requests by address per epoch for each domain and custodian
    mapping(address domain => 
        mapping(uint256 epoch => 
            mapping(address custodian =>
                mapping(address attester => uint256 stake))))  public epochUnstakesByDomain;

    /// @notice Total stake for an epoch, computed at rollover time
    mapping(address domain =>
        mapping(uint256 epoch =>
            mapping(address attester => uint256 stake))) public epochTotalStakeByDomain;

    /// @notice Tracks which domains have accepted genesis ceremony
    mapping(address domain => bool) public domainGenesisAccepted;

    /// @notice Role required for staking and unstaking operations
    bytes32 public constant WHITELIST_ROLE = keccak256("WHITELIST_ROLE");
}
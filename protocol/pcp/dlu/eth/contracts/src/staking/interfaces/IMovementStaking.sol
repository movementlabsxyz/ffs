pragma solidity ^0.8.13;

import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";

// canonical order: domain, epoch, custodian, attester, stake =? decas
interface IMovementStaking {
    function registerDomain(
        uint256 epochDuration,
        address[] calldata custodians
    ) external;
    function acceptGenesisCeremony() external;
    function getEpochByL1BlockTime(address) external view returns (uint256);
    function getAcceptingEpoch(address) external view returns (uint256);
    function getNextAcceptingEpochWithException(address) external view returns (uint256);
    function getNextPresentEpochWithException(address) external view returns (uint256);
    function getStake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester
    ) external view returns (uint256);
    function getStakeForAcceptingEpoch(
        address domain,
        address custodian,
        address attester
    ) external view returns (uint256);
    function getUnstake(
        address domain,
        uint256 epoch,
        address custodian,
        address attester
    ) external view returns (uint256);
    function getUnstakeForAcceptingEpoch(
        address domain,
        address custodian,
        address attester
    ) external view returns (uint256);
    function getCustodianStake(
        address domain,
        uint256 epoch,
        address custodian
    ) external view returns (uint256);
    function getCustodianStakeForAcceptingEpoch(
        address domain,
        address custodian
    ) external view returns (uint256);
    function stake(address domain, IERC20 custodian, uint256 amount) external;
    function unstake(
        address domain,
        address custodian,
        uint256 amount
    ) external;
    function getRegisteredCustodians(
        address domain
    ) external view returns (address[] memory);
    function getRegisteredAttesters(
        address domain
    ) external view returns (address[] memory);
    function rollOverEpoch() external;
    function slash(
        address[] calldata custodians,
        address[] calldata attesters,
        uint256[] calldata amounts,
        uint256[] calldata refundAmounts
    ) external;

    function whitelistAddress(address addr) external;
    function removeAddressFromWhitelist(address addr) external;

    event AttesterStaked(
        address indexed domain,
        uint256 indexed epoch,
        address indexed custodian,
        address attester,
        uint256 stake
    );

    event AttesterUnstaked(
        address indexed domain,
        uint256 indexed epoch,
        address indexed custodian,
        address attester,
        uint256 stake
    );

    event AttesterEpochRolledOver(
        address indexed attester,
        uint256 indexed epoch,
        address indexed custodian,
        uint256 stake,
        uint256 unstake
    );

    event EpochRolledOver(address indexed domain, uint256 epoch);
    
    error StakeExceedsGenesisStake();
    error CustodianTransferAmountMismatch();
    error GenesisAlreadyAccepted();

    function getStakedAttestersForAcceptingEpoch(address domain) external view returns (address[] memory);
    function computeAllStakeForAcceptingEpoch(address attester) external view returns (uint256);
    function getAttesterStakeForAcceptingEpoch(address domain, address attester) external view returns (uint256);

    function rewardFromDomain(address attester, uint256 amount, address custodian) external;
    function rewardArray(address[] calldata attesters, uint256[] calldata amounts, address[] calldata custodians) external;

    function getEpochDuration(address domain) external view returns (uint256);

    function setAcceptingEpoch(address domain, uint256 newEpoch) external;
}

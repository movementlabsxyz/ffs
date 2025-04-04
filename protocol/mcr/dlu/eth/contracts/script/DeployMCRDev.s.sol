// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Script.sol";
import "../src/token/MOVEToken.sol";
import "../src/staking/MovementStaking.sol";
import "../src/settlement/MCR.sol";
import "../src/settlement/McrARO.sol";
import {IMintableToken, MintableToken} from "../src/token/base/MintableToken.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ITransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {IERC20} from "@openzeppelin/contracts/interfaces/IERC20.sol";
import {IMcrReward} from "../src/settlement/interfaces/IMcrReward.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import "forge-std/console.sol";

/**
 * @title DeployMCRDev
 * @notice Script for deploying the MCR ecosystem with configurable parameters
 * @dev Supports both fresh deployments and upgrades to existing deployments
 */
contract DeployMCRDev is Script {
    /**
     * @notice Deployment configuration for MCR ecosystem
     * @dev This struct contains all configurable parameters for deployment
     */
    struct DeployConfig {
        // Admin configuration
        address contractAdmin;        // Admin address for deployed contracts
        
        // Token configuration
        string tokenName;
        string tokenSymbol;
        uint256 initialTokenMint;
        
        // Staking configuration
        address[] custodians;
        
        // MCR configuration
        uint256 initialBlockHeight;
        uint256 leadingCommitmentTolerance;
        uint256 epochDuration;
        
        // Reward configuration
        uint8 rewardOption;              // 0=none, 1=deploy ARO, 2=existing
        address existingRewardContract;  // Only used if rewardOption=2
        
        // Existing contracts (for upgrades)
        address existingProxyAdmin;      // If set, will use this instead of deploying new
        address existingMoveTokenProxy;  // If set, will upgrade this instead of deploying new
        address existingStakingProxy;    // If set, will upgrade this instead of deploying new
        address existingMcrProxy;        // If set, will upgrade this instead of deploying new
        address existingAroProxy;        // If set, will upgrade this instead of deploying new// If true, will nullify the proxies
    }
    
    /**
     * @notice Default run function required by forge script
     */
    function run() public returns (DeployedAddresses memory) {
        // Default configuration for Anvil deployment
        string memory defaultConfig = vm.envString("DEPLOY_CONFIG");
        return run(defaultConfig);
    }
    
    /**
     * @notice Main deployment function that accepts a JSON string
     * @param jsonConfig JSON string containing deployment configuration
     * @return addresses Struct containing addresses of deployed contracts
     */
    function run(string memory jsonConfig) public returns (DeployedAddresses memory) {
        // Parse JSON string into DeployConfig struct
        DeployConfig memory config = parseConfig(jsonConfig);
        
        // Call the implementation function with parsed config
        return runWithConfig(config);
    }
    
    /**
     * @notice Parse JSON string into DeployConfig struct
     * @param jsonConfig JSON string containing deployment configuration
     * @return config Parsed DeployConfig struct
     */
    function parseConfig(string memory jsonConfig) internal pure returns (DeployConfig memory config) {
        // Use forge-std's JSON parsing utilities
        address contractAdmin = vm.parseJsonAddress(jsonConfig, ".contractAdmin");
        string memory tokenName = vm.parseJsonString(jsonConfig, ".tokenName");
        string memory tokenSymbol = vm.parseJsonString(jsonConfig, ".tokenSymbol");
        uint256 initialTokenMint = vm.parseJsonUint(jsonConfig, ".initialTokenMint");
        address[] memory custodians = vm.parseJsonAddressArray(jsonConfig, ".custodians");
        uint256 initialBlockHeight = vm.parseJsonUint(jsonConfig, ".initialBlockHeight");
        uint256 leadingCommitmentTolerance = vm.parseJsonUint(jsonConfig, ".leadingCommitmentTolerance");
        uint256 epochDuration = vm.parseJsonUint(jsonConfig, ".epochDuration");
        uint8 rewardOption = uint8(vm.parseJsonUint(jsonConfig, ".rewardOption"));
        address existingRewardContract = vm.parseJsonAddress(jsonConfig, ".existingRewardContract");
        address existingProxyAdmin = vm.parseJsonAddress(jsonConfig, ".existingProxyAdmin");
        address existingMoveTokenProxy = vm.parseJsonAddress(jsonConfig, ".existingMoveTokenProxy");
        address existingStakingProxy = vm.parseJsonAddress(jsonConfig, ".existingStakingProxy");
        address existingMcrProxy = vm.parseJsonAddress(jsonConfig, ".existingMcrProxy");
        address existingAroProxy = vm.parseJsonAddress(jsonConfig, ".existingAroProxy");

        return DeployConfig({
            contractAdmin: contractAdmin,
            tokenName: tokenName,
            tokenSymbol: tokenSymbol,
            initialTokenMint: initialTokenMint,
            custodians: custodians,
            initialBlockHeight: initialBlockHeight,
            leadingCommitmentTolerance: leadingCommitmentTolerance,
            epochDuration: epochDuration,
            rewardOption: rewardOption,
            existingRewardContract: existingRewardContract,
            existingProxyAdmin: existingProxyAdmin,
            existingMoveTokenProxy: existingMoveTokenProxy,
            existingStakingProxy: existingStakingProxy,
            existingMcrProxy: existingMcrProxy,
            existingAroProxy: existingAroProxy
        });
    }
    
    /**
     * @notice Implementation of the deployment logic
     * @param config All configuration parameters for deployment
     * @return addresses Struct containing addresses of deployed contracts
     */
    function runWithConfig(DeployConfig memory config) internal returns (DeployedAddresses memory addresses) {
        // Validate config
        if (config.rewardOption == 2) {
            require(config.existingRewardContract != address(0), "Existing reward contract address must be provided when rewardOption = 2");
        }

        vm.startBroadcast();
        vm.recordLogs();

        // Handle ProxyAdmin - use existing or deploy new
        ProxyAdmin proxyAdmin;
        if (config.existingProxyAdmin != address(0)) {
            proxyAdmin = ProxyAdmin(config.existingProxyAdmin);
            console.log("JSONL using_existing_proxy_admin = %s", address(proxyAdmin));
            console.log("JSONL proxy_admin = %s", address(proxyAdmin));
        } else {
            proxyAdmin = new ProxyAdmin(config.contractAdmin);
            console.log("JSONL proxy_admin = %s", address(proxyAdmin));
        }
        addresses.proxyAdmin = address(proxyAdmin);

        // Deploy or use existing implementations
        
        // Token Implementation
        MintableToken moveTokenImplementation = new MintableToken();
        console.log("JSONL move_token_implementation = %s", address(moveTokenImplementation));
        addresses.moveTokenImplementation = address(moveTokenImplementation);
        
        // Staking Implementation
        MovementStaking stakingImplementation = new MovementStaking();
        console.log("JSONL staking_implementation = %s", address(stakingImplementation));
        addresses.stakingImplementation = address(stakingImplementation);
        
        // MCR Implementation
        MCR mcrImplementation = new MCR();
        console.log("JSONL mcr_implementation = %s", address(mcrImplementation));
        addresses.mcrImplementation = address(mcrImplementation);

        // Handle Token Proxy - use existing or deploy new
        TransparentUpgradeableProxy moveTokenProxy;
        if (config.existingMoveTokenProxy != address(0)) {
            moveTokenProxy = TransparentUpgradeableProxy(payable(config.existingMoveTokenProxy));
            
            // Upgrade the implementation
            proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(moveTokenProxy)), address(moveTokenImplementation), "");
            console.log("JSONL token_proxy = %s", address(moveTokenProxy));
        } else {
            // Deploy the Move Token Proxy
            bytes memory moveTokenData = abi.encodeCall(
                MintableToken.initialize, 
                (config.tokenName, config.tokenSymbol)
            );
            moveTokenProxy = new TransparentUpgradeableProxy(
                address(moveTokenImplementation), address(proxyAdmin), moveTokenData
            );
            console.log("JSONL token_proxy = %s", address(moveTokenProxy));
        }
        addresses.moveTokenProxy = address(moveTokenProxy);

        // Handle Staking Proxy - use existing or deploy new
        TransparentUpgradeableProxy movementStakingProxy;
        if (config.existingStakingProxy != address(0)) {
            movementStakingProxy = TransparentUpgradeableProxy(payable(config.existingStakingProxy));
            
            // Upgrade the implementation
            proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(movementStakingProxy)), address(stakingImplementation), "");
            console.log("JSONL staking_proxy = %s", address(movementStakingProxy));
        } else {
            // Deploy the Movement Staking Proxy
            bytes memory movementStakingData =
                abi.encodeCall(MovementStaking.initialize, IMintableToken(address(moveTokenProxy)));
            movementStakingProxy = new TransparentUpgradeableProxy(
                address(stakingImplementation), address(proxyAdmin), movementStakingData
            );
            console.log("JSONL staking_proxy = %s", address(movementStakingProxy));
        }
        addresses.movementStakingProxy = address(movementStakingProxy);

        // Handle MCR Proxy - use existing or deploy new
        TransparentUpgradeableProxy mcrProxy;
        if (config.existingMcrProxy != address(0)) {
            mcrProxy = TransparentUpgradeableProxy(payable(config.existingMcrProxy));
            
            // Upgrade the implementation
            proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(mcrProxy)), address(mcrImplementation), "");
            console.log("JSONL mcr_proxy = %s", address(mcrProxy));
        } else {
            // Deploy the MCR Proxy
            bytes memory mcrData = abi.encodeCall(
                MCR.initialize, 
                (
                    IMovementStaking(address(movementStakingProxy)), 
                    config.initialBlockHeight, 
                    config.leadingCommitmentTolerance, 
                    config.epochDuration, 
                    config.custodians
                )
            );
            mcrProxy = new TransparentUpgradeableProxy(
                address(mcrImplementation), address(proxyAdmin), mcrData
            );
            console.log("JSONL mcr_proxy = %s", address(mcrProxy));
        }
        addresses.mcrProxy = address(mcrProxy);
        
        // Handle reward contract setup based on rewardOption
        if (config.rewardOption == 1) {
            // Deploy McrARO implementation
            McrARO aroImplementation = new McrARO();
            console.log("JSONL aro_implementation = %s", address(aroImplementation));
            addresses.aroImplementation = address(aroImplementation);
            
            // Handle ARO Proxy - use existing or deploy new
            TransparentUpgradeableProxy aroProxy;
            if (config.existingAroProxy != address(0)) {
                aroProxy = TransparentUpgradeableProxy(payable(config.existingAroProxy));
                
                // Upgrade the implementation
                proxyAdmin.upgradeAndCall(ITransparentUpgradeableProxy(address(aroProxy)), address(aroImplementation), "");
                console.log("JSONL aro_proxy = %s", address(aroProxy));
            } else {
                // Deploy the ARO Proxy
                bytes memory aroData = abi.encodeCall(McrARO.initializeRewardConfig, ());
                aroProxy = new TransparentUpgradeableProxy(
                    address(aroImplementation), address(proxyAdmin), aroData
                );
                console.log("JSONL aro_proxy = %s", address(aroProxy));
            }
            addresses.aroProxy = address(aroProxy);
            
            // Link MCR to the ARO reward contract
            MCR(address(mcrProxy)).setRewardContract(IMcrReward(address(aroProxy)));
            console.log("JSONL reward_contract_set = true");
            console.log("JSONL reward_contract_address = %s", address(aroProxy));
            console.log("JSONL reward_contract_type = %s", "McrARO");
            console.log("JSONL reward_proxy = %s", address(aroProxy));
        } else if (config.rewardOption == 2) {
            // Link MCR to the existing reward contract
            MCR(address(mcrProxy)).setRewardContract(IMcrReward(config.existingRewardContract));
            console.log("JSONL reward_contract_set = true");
            console.log("JSONL reward_contract_address = %s", config.existingRewardContract);
            console.log("JSONL reward_contract_type = %s", "External");
            console.log("JSONL reward_proxy = %s", config.existingRewardContract);
        } else {
            // No reward contract
            console.log("JSONL reward_contract_set = false");
            console.log("JSONL reward_proxy = %s", address(0));
        }

        // Only do these steps for fresh deployments
        if (config.existingMcrProxy == address(0)) {
            // Grant commitment admin
            MCR mcr = MCR(address(mcrProxy));
            mcr.grantCommitmentAdmin(config.contractAdmin);
            mcr.grantCommitmentAdmin(msg.sender);
            console.log("JSONL granted_commitment_admin = %s", config.contractAdmin);

            // Verify custodian setup
            uint256 custodianEpochDuration = MovementStaking(address(movementStakingProxy)).epochDurationByDomain(address(mcrProxy));
            console.log("JSONL mcr_custodian_epoch_duration = %d", custodianEpochDuration);

            // Mint tokens
            MintableToken moveToken = MintableToken(address(moveTokenProxy));
            if (config.initialTokenMint > 0) {
                moveToken.mint(config.contractAdmin, config.initialTokenMint);
                console.log("JSONL minted_tokens = { \"recipient\" : \"%s\", \"amount\" : \"%d\" }", config.contractAdmin, config.initialTokenMint);
            }

            // Grant minter role
            moveToken.grantMinterRole(msg.sender);
            moveToken.grantMinterRole(config.contractAdmin);
            console.log("JSONL granted_minter_role = %s", config.contractAdmin);

            moveToken.grantMinterRole(address(movementStakingProxy));
            // console.log("JSONL granted_minter_role = %s", address(movementStakingProxy));
        } else {
            console.log("JSONL skipping_initialization = true");
        }

        vm.stopBroadcast();
        
        return addresses;
    }
    
    /**
     * @notice Upgrades an existing deployment
     * @param config Configuration including contractAdmin and existing addresses to upgrade
     * @return addresses Updated contract addresses
     */
    function upgrade(DeployConfig memory config) public returns (DeployedAddresses memory) {
        return runWithConfig(config);
    }
    
    /**
     * @notice Creates a default configuration
     * @return config Default deployment configuration
     */
    function getDefaultConfig() internal pure returns (DeployConfig memory config) {
        // Default token config
        config.tokenName = "Move Token";
        config.tokenSymbol = "MOVE";
        config.initialTokenMint = 100000 ether;
        
        // Default MCR config
        config.initialBlockHeight = 0;
        config.leadingCommitmentTolerance = 10;
        config.epochDuration = 4 seconds;
        
        // Default custodians - will be overridden when deployed
        config.custodians = new address[](1);
        
        // Default reward config
        config.rewardOption = 0; // No reward by default
        config.existingRewardContract = address(0);
        
        // Default existing contracts
        config.existingProxyAdmin = address(0);
        config.existingMoveTokenProxy = address(0);
        config.existingStakingProxy = address(0);
        config.existingMcrProxy = address(0);
        config.existingAroProxy = address(0);
        
        return config;
    }
    
    /**
     * @notice Struct to return all deployed contract addresses
     */
    struct DeployedAddresses {
        address proxyAdmin;
        address moveTokenImplementation;
        address stakingImplementation;
        address mcrImplementation;
        address moveTokenProxy;
        address movementStakingProxy;
        address mcrProxy;
        address aroImplementation;
        address aroProxy;
    }
}
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {MCR} from "../src/settlement/MCR.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import { Helper } from "./helpers/Helper.sol";

contract MCRDeployer is Helper {

    function run() external virtual {
        
        // load config and deployments data
        _loadExternalData();

        uint256 signer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(signer);

        // Deploy CREATE3Factory, Safes and Timelock if not deployed
        _deployDependencies();

        deployment.mcrAdmin == ZERO && deployment.mcr == ZERO && deployment.move != ZERO && deployment.staking != ZERO ?
            _deployMCR() : deployment.mcrAdmin != ZERO && deployment.mcr != ZERO ?
                _upgradeMCR() : revert("MCR: both admin and proxy should be registered");

        vm.stopBroadcast();

        // Only write to file if chainid is not running a foundry local chain
        if (vm.isContext(VmSafe.ForgeContext.ScriptBroadcast)) {
                _writeDeployments();
        }
    }

    // •☽────✧˖°˖DANGER ZONE˖°˖✧────☾•
// Modifications to the following functions have to be throughly tested

    function _deployMCR() internal {
        console.log("MCR: deploying");
        MCR mcrImplementation = new MCR();
        vm.recordLogs();
        mcrProxy = new TransparentUpgradeableProxy(
            address(mcrImplementation),
            address(timelock),
            abi.encodeWithSignature(
                mcrSignature,
                address(stakingProxy),
                128,
                100 ether,
                100 ether, 
                config.signersLabs
            )
        );
        console.log("MCR deployment records:");
        console.log("proxy", address(mcrProxy));
        deployment.mcr = address(mcrProxy);
        deployment.mcrAdmin = _storeAdminDeployment();
    }

    function _upgradeMCR() internal {
        console.log("MCR: upgrading");
        MCR newMCRImplementation = new MCR();
        _checkBytecodeDifference(address(newMCRImplementation), deployment.mcr);
        bytes memory data = abi.encodeWithSignature(
            "schedule(address,uint256,bytes,bytes32,bytes32,uint256)",
            address(deployment.mcrAdmin),
            0,
            abi.encodeWithSignature(
                "upgradeAndCall(address,address,bytes)",
                address(mcrProxy),
                address(newMCRImplementation),
                ""
            ),
            bytes32(0),
            bytes32(0),
            config.minDelay
        );
        _proposeUpgrade(data, "mcr.json");
    }

}

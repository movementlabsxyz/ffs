pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {PCP} from "../src/settlement/PCP.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";
import { Helper } from "./helpers/Helper.sol";

contract PCPDeployer is Helper {

    function run() external virtual {
        
        // load config and deployments data
        _loadExternalData();

        uint256 signer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(signer);

        // Deploy CREATE3Factory, Safes and Timelock if not deployed
        _deployDependencies();

        deployment.pcpAdmin == ZERO && deployment.pcp == ZERO && deployment.move != ZERO && deployment.staking != ZERO ?
            _deployPCP() : deployment.pcpAdmin != ZERO && deployment.pcp != ZERO ?
                _upgradePCP() : revert("PCP: both admin and proxy should be registered");

        vm.stopBroadcast();

        // Only write to file if chainid is not running a foundry local chain
        if (vm.isContext(VmSafe.ForgeContext.ScriptBroadcast)) {
                _writeDeployments();
        }
    }

    // •☽────✧˖°˖DANGER ZONE˖°˖✧────☾•
// Modifications to the following functions have to be throughly tested

    function _deployPCP() internal {
        console.log("PCP: deploying");
        PCP pcpImplementation = new PCP();
        vm.recordLogs();
        pcpProxy = new TransparentUpgradeableProxy(
            address(pcpImplementation),
            address(timelock),
            abi.encodeWithSignature(
                pcpSignature,
                address(stakingProxy),
                128,
                100 ether,
                100 ether, 
                config.signersLabs
            )
        );
        console.log("PCP deployment records:");
        console.log("proxy", address(pcpProxy));
        deployment.pcp = address(pcpProxy);
        deployment.pcpAdmin = _storeAdminDeployment();
    }

    function _upgradePCP() internal {
        console.log("PCP: upgrading");
        PCP newPCPImplementation = new PCP();
        _checkBytecodeDifference(address(newPCPImplementation), deployment.pcp);
        bytes memory data = abi.encodeWithSignature(
            "schedule(address,uint256,bytes,bytes32,bytes32,uint256)",
            address(deployment.pcpAdmin),
            0,
            abi.encodeWithSignature(
                "upgradeAndCall(address,address,bytes)",
                address(pcpProxy),
                address(newPCPImplementation),
                ""
            ),
            bytes32(0),
            bytes32(0),
            config.minDelay
        );
        _proposeUpgrade(data, "pcp.json");
    }

}

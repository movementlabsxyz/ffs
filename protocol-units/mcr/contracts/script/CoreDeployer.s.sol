pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {MOVEToken} from "../src/token/MOVEToken.sol";
import { Helper } from "./helpers/Helper.sol";
import { MCRDeployer } from "./MCRDeployer.s.sol";
import { MovementStakingDeployer } from "./MovementStakingDeployer.s.sol";
import { StlMoveDeployer } from "./StlMoveDeployer.s.sol";
import { MOVETokenDeployer } from "./MOVETokenDeployer.s.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {TimelockController} from "@openzeppelin/contracts/governance/TimelockController.sol";

contract CoreDeployer is MCRDeployer, MovementStakingDeployer, StlMoveDeployer, MOVETokenDeployer {

    function run() external override(MCRDeployer, MovementStakingDeployer, StlMoveDeployer, MOVETokenDeployer) {

        // load config and deployments data
        _loadExternalData();

        uint256 signer = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(signer);

        // Deploy CREATE3Factory, Safes and Timelock if not deployed
        _deployDependencies();
        
        // Deploy or upgrade contracts conditionally
        deployment.moveAdmin == ZERO && deployment.move == ZERO ?
            _deployMove() : deployment.moveAdmin != ZERO && deployment.move != ZERO ?
                // if move is already deployed, upgrade it
                _upgradeMove() : revert("MOVE: both admin and proxy should be registered");

        // requires move to be deployed
        deployment.stakingAdmin == ZERO && deployment.staking == ZERO && deployment.move != ZERO ?
            _deployStaking() : deployment.stakingAdmin != ZERO && deployment.staking != ZERO ?
                // if staking is already deployed, upgrade it
                _upgradeStaking() : revert("STAKING: both admin and proxy should be registered");

        // requires move to be deployed
        deployment.stlMoveAdmin == ZERO && deployment.stlMove == ZERO && deployment.move != ZERO ?
            _deployStlMove() : deployment.stlMoveAdmin != ZERO && deployment.stlMove != ZERO ?
                // if stlMove is already deployed, upgrade it
                _upgradeStlMove() : revert("STL: both admin and proxy should be registered");

        // requires staking and move to be deployed
        deployment.mcrAdmin == ZERO && deployment.mcr == ZERO && deployment.move != ZERO && deployment.staking != ZERO ?
            _deployMCR() : deployment.mcrAdmin != ZERO && deployment.mcr != ZERO ?
                // if mcr is already deployed, upgrade it
                _upgradeMCR() : revert("MCR: both admin and proxy should be registered");

        // Only write to file if chainid is not running a foundry local chain and if broadcasting
        if (block.chainid == foundryChainId) {
            _allowSameContract();
            _upgradeMove();
            _upgradeStaking();
            _upgradeStlMove();
            _upgradeMCR();
        } else {
            if (vm.isContext(VmSafe.ForgeContext.ScriptBroadcast)) {
                _writeDeployments();
            }
        }

        vm.stopBroadcast();
    }
}

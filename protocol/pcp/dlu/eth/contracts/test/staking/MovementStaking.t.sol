// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/staking/MovementStaking.sol";
import "../../src/token/MOVETokenDev.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";

contract MovementStakingTest is Test {
    bytes32 public constant WHITELIST_ROLE = keccak256("WHITELIST_ROLE");
    address public multisig = address(this);
    MOVETokenDev public moveToken;
    MovementStaking public staking;

    function setUp() public {
        MOVETokenDev moveTokenImpl = new MOVETokenDev();
        TransparentUpgradeableProxy moveProxy = new TransparentUpgradeableProxy(
            address(moveTokenImpl),
            address(this),
            abi.encodeWithSignature("initialize(address)", multisig)
        );

        MovementStaking stakingImpl = new MovementStaking();
        TransparentUpgradeableProxy stakingProxy = new TransparentUpgradeableProxy(
            address(stakingImpl),
            address(this),
            abi.encodeWithSignature("initialize(address)", address(moveProxy))
        );
        moveToken = MOVETokenDev(address(moveProxy));
        staking = MovementStaking(address(stakingProxy));
    }

    function testCannotInitializeTwice() public {
        // Attempt to initialize again should fail
        vm.expectRevert(bytes4(0xf92ee8a9));
        staking.initialize(moveToken);
    }

    function testRegister() public {

        // Register a new domain
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        assertEq(staking.getAcceptingEpoch(domain), 0);
    }

    function testWhitelist() public {

        // Our whitelister
        address whitelister = vm.addr(1);
        // Whitelist them
        staking.whitelistAddress(whitelister);
        assertEq(staking.hasRole(WHITELIST_ROLE, whitelister), true);
        // Remove them from the whitelist
        staking.removeAddressFromWhitelist(whitelister);
        assertEq(staking.hasRole(WHITELIST_ROLE, whitelister), false);
        // As a whitelister let's see if I can whitelist myself
        vm.prank(whitelister);
        vm.expectRevert();
        staking.whitelistAddress(whitelister);
    }

    function testSimpleStaker() public {
        // Register a new staker
        address payable domain = payable(vm.addr(1));
        
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);

        vm.prank(domain);
        staking.registerDomain(3600 seconds, custodians);
        assertEq(staking.getEpochDuration(domain), 3600 seconds, "Epoch duration not set correctly");

        // stake at the domain
        address payable staker = payable(vm.addr(2));

        staking.whitelistAddress(staker);
        moveToken.mint(staker, 100);

        vm.prank(staker);
        moveToken.approve(address(staking), 100);

        vm.prank(staker);
        staking.stake(domain, moveToken, 100);

        assertEq(moveToken.balanceOf(staker), 0);
        assertEq(staking.getStake(domain, 0, address(moveToken), staker), 100);
    }

    function testSimpleGenesisCeremony() public {
        // Register a new staker
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);
        assertEq(staking.getEpochDuration(domain), 1 seconds, "Epoch duration not set correctly");

        // genesis ceremony
        address payable staker = payable(vm.addr(2));
        staking.whitelistAddress(staker);
        moveToken.mint(staker, 100);
        vm.prank(staker);
        moveToken.approve(address(staking), 100);
        vm.prank(staker);
        staking.stake(domain, moveToken, 100);
        vm.prank(domain);
        staking.acceptGenesisCeremony();

        assertNotEq(staking.currentAcceptingEpochByDomain(domain), 0);
        assertEq(staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker), 100);

        vm.expectRevert(IMovementStaking.GenesisAlreadyAccepted.selector);
        vm.prank(domain);
        staking.acceptGenesisCeremony();
    }

    function testSimpleRolloverEpoch() public {
       

        // Register a new staker
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        // genesis ceremony
        address payable staker = payable(vm.addr(2));
        staking.whitelistAddress(staker);
        moveToken.mint(staker, 100);
        staking.whitelistAddress(staker);
        vm.prank(staker);
        moveToken.approve(address(staking), 100);
        vm.prank(staker);
        staking.stake(domain, moveToken, 100);
        vm.prank(domain);
        staking.acceptGenesisCeremony();

        // rollover epoch
        for (uint256 i = 0; i < 10; i++) {
            vm.warp((i + 1) * 1 seconds);
            uint256 epochBefore = staking.getAcceptingEpoch(domain);
            vm.prank(domain);
            staking.rollOverEpoch();
            uint256 epochAfter = staking.getAcceptingEpoch(domain);
            assertEq(epochAfter, epochBefore + 1);
            assertEq(staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker), 100);
        }
    }

    function testUnstakeRolloverEpoch() public {
       

        // Register a new staker
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        // genesis ceremony
        address payable staker = payable(vm.addr(2));
        staking.whitelistAddress(staker);
        moveToken.mint(staker, 100);
        vm.prank(staker);
        moveToken.approve(address(staking), 100);
        vm.prank(staker);
        staking.stake(domain, moveToken, 100);
        vm.prank(domain);
        staking.acceptGenesisCeremony();

        for (uint256 i = 0; i < 10; i++) {
            vm.warp((i + 1) * 1 seconds);
            uint256 epochBefore = staking.getAcceptingEpoch(domain);

            // unstake
            vm.prank(staker);
            staking.unstake(domain, address(moveToken), 10);
            assertEq(staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker), 100 - (i * 10));
            assertEq(moveToken.balanceOf(staker), i * 10);

            // roll over
            vm.prank(domain);
            staking.rollOverEpoch();
            uint256 epochAfter = staking.getAcceptingEpoch(domain);
            assertEq(epochAfter, epochBefore + 1);
        }
    }

    function testUnstakeAndStakeRolloverEpoch() public {
       

        // Register a new staker
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        // genesis ceremony
        address payable staker = payable(vm.addr(2));
        staking.whitelistAddress(staker);
        moveToken.mint(staker, 150);
        vm.prank(staker);
        moveToken.approve(address(staking), 100);
        vm.prank(staker);
        staking.stake(domain, moveToken, 100);
        vm.prank(domain);
        staking.acceptGenesisCeremony();

        for (uint256 i = 0; i < 10; i++) {
            vm.warp((i + 1) * 1 seconds);
            uint256 epochBefore = staking.getAcceptingEpoch(domain);

            // unstake
            vm.prank(staker);
            staking.unstake(domain, address(moveToken), 10);

            // stake
            vm.prank(staker);
            moveToken.approve(address(staking), 5);
            vm.prank(staker);
            staking.stake(domain, moveToken, 5);

            // check stake
            assertEq(staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker), (100 - (i * 10)) + (i * 5));
            assertEq(moveToken.balanceOf(staker), (50 - (i + 1) * 5) + (i * 10));

            // roll over
            vm.prank(domain);
            staking.rollOverEpoch();
            uint256 epochAfter = staking.getAcceptingEpoch(domain);
            assertEq(epochAfter, epochBefore + 1);
        }
    }

    function testUnstakeStakeAndSlashRolloverEpoch() public {
       

        // Register a new staker
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        // genesis ceremony
        address payable staker = payable(vm.addr(2));
        staking.whitelistAddress(staker);
        moveToken.mint(staker, 150);
        vm.prank(staker);
        moveToken.approve(address(staking), 100);
        vm.prank(staker);
        staking.stake(domain, moveToken, 100);
        vm.prank(domain);
        staking.acceptGenesisCeremony();

        for (uint256 i = 0; i < 5; i++) {
            vm.warp((i + 1) * 1 seconds);
            uint256 epochBefore = staking.getAcceptingEpoch(domain);

            // unstake
            vm.prank(staker);
            staking.unstake(domain, address(moveToken), 10);

            // stake
            vm.prank(staker);
            moveToken.approve(address(staking), 5);
            vm.prank(staker);
            staking.stake(domain, moveToken, 5);

            // check stake
            assertEq(
                staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker), (100 - (i * 10)) + (i * 5) - (i * 1)
            );
            assertEq(moveToken.balanceOf(staker), (50 - (i + 1) * 5) + (i * 10));

            // slash
            vm.prank(domain);
            address[] memory custodians1 = new address[](1);
            custodians1[0] = address(moveToken);
            address[] memory attesters1 = new address[](1);
            attesters1[0] = staker;
            uint256[] memory amounts1 = new uint256[](1);
            amounts1[0] = 1;
            uint256[] memory refundAmounts1 = new uint256[](1);
            refundAmounts1[0] = 0;
            staking.slash(custodians1, attesters1, amounts1, refundAmounts1);

            // slash immediately takes effect
            assertEq(
                staking.getStakeForAcceptingEpoch(domain, address(moveToken), staker),
                (100 - (i * 10)) + (i * 5) - ((i + 1) * 1)
            );

            // roll over
            vm.prank(domain);
            staking.rollOverEpoch();
            uint256 epochAfter = staking.getAcceptingEpoch(domain);
            assertEq(epochAfter, epochBefore + 1);
        }
    }

    function testHalbornReward() public {

        // Register a domain
        address payable domain = payable(vm.addr(1));
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        vm.prank(domain);
        staking.registerDomain(1 seconds, custodians);

        // Alice stakes 1000 tokens
        address payable alice = payable(vm.addr(2));
        staking.whitelistAddress(alice);
        moveToken.mint(alice, 1000);
        vm.prank(alice);
        moveToken.approve(address(staking), 1000);
        vm.prank(alice);
        staking.stake(domain, moveToken, 1000);

        // Bob stakes 100 tokens
        address payable bob = payable(vm.addr(3));
        staking.whitelistAddress(bob);
        moveToken.mint(bob, 100);
        vm.prank(bob);
        moveToken.approve(address(staking), 100);
        vm.prank(bob);
        staking.stake(domain, moveToken, 100);

        // Assertions on stakes and balances
        assertEq(moveToken.balanceOf(alice), 0);
        assertEq(moveToken.balanceOf(bob), 0);
        assertEq(moveToken.balanceOf(address(staking)), 1100);
        assertEq(staking.getCustodianStake(domain, 0, address(moveToken)), 1100);
        assertEq(staking.getStake(domain, 0, address(moveToken), alice), 1000);
        assertEq(staking.getStake(domain, 0, address(moveToken), bob), 100);

        // Charlie calls reward with himself only to steal tokens
        address charlie = vm.addr(4);
        address[] memory attesters = new address[](1);
        attesters[0] = charlie;
        uint256[] memory amounts = new uint256[](1);
        amounts[0] = 1000;
        vm.prank(charlie);
        vm.expectRevert(
            abi.encodeWithSignature(
                "ERC20InsufficientAllowance(address,uint256,uint256)",
                address(staking), // should be called by the staking contract
                0,
                1000
            )
        );
        staking.rewardArray(attesters, amounts, custodians);
    }

    function testRewardSingleAttester() public {
        // Register a domain
        address domain = address(this);
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        staking.registerDomain(7200 seconds, custodians);
        // Setup domain to pay rewards
        // moveToken.mint(domain, 100);  // Domain has already funds
        moveToken.approve(address(staking), 100);  // Domain approves staking

        
        // Alice stakes 1000 tokens
        address payable alice = payable(vm.addr(2));
        staking.whitelistAddress(alice);
        moveToken.mint(alice, 1000);
        vm.prank(alice);
        moveToken.approve(address(staking), 1000);
        vm.prank(alice);
        staking.stake(domain, moveToken, 1000);
        
        // Assertions on stakes and balances
        assertEq(moveToken.balanceOf(alice), 0, "Alice should have 0 tokens");
        assertEq(moveToken.balanceOf(address(staking)), 1000, "Staking contract should have 1000 tokens");
        assertEq(staking.getCustodianStake(domain, 0, address(moveToken)), 1000, "Custodian stake should be 1000");
        assertEq(staking.getStake(domain, 0, address(moveToken), alice), 1000, "Alice stake should be 1000");
                
        // Reward alice
        console.log("This is moveToken:", address(moveToken));
        console.log("This is staking:", address(staking));
        console.log("This is alice:", alice);
        console.log("This is domain:", domain);
        vm.prank(domain);  // Domain calls reward
        staking.rewardFromDomain(alice, 100, address(moveToken));
        
        // Verify reward was received
        assertEq(moveToken.balanceOf(alice), 100, "Alice should have received 100 tokens");
    }

    function testSetAcceptingEpoch() public {
        // Setup
        address domain = makeAddr("domain");
        vm.startPrank(domain);
        
        // Register domain
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        staking.registerDomain(10, custodians);
        staking.acceptGenesisCeremony();
        
        // Test setting accepting epoch
        uint256 currentEpoch = staking.getAcceptingEpoch(domain);
        uint256 epochDuration = staking.getEpochDuration(domain);
        vm.warp(block.timestamp + 10 * epochDuration);
        uint256 presentEpoch = staking.getEpochByL1BlockTime(domain);
        
        // Should succeed when newEpoch is between current and present
        uint256 newEpoch = currentEpoch + 1;
        vm.assume(newEpoch <= presentEpoch);
        staking.setAcceptingEpoch(domain, newEpoch);
        assertEq(staking.getAcceptingEpoch(domain), newEpoch);
        
        // Should fail when caller is not domain
        vm.stopPrank();
        vm.expectRevert("UNAUTHORIZED");
        staking.setAcceptingEpoch(domain, newEpoch + 1);
        
        // Should fail when newEpoch > present epoch
        vm.startPrank(domain);
        vm.expectRevert("NEW_EPOCH_MUST_BE_LESS_THAN_PRESENT_EPOCH");
        staking.setAcceptingEpoch(domain, presentEpoch + 1);
        
        // Should fail when newEpoch <= current epoch
        vm.expectRevert("NEW_EPOCH_MUST_BE_HIGHER_THAN_CURRENT_EPOCH");
        staking.setAcceptingEpoch(domain, newEpoch);
    }

}




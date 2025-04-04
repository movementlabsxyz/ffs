// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "forge-std/Test.sol";
import "../../src/staking/MovementStaking.sol";
import "../../src/token/MOVETokenDev.sol";
import "../../src/settlement/PCP.sol";
import "../../src/settlement/PCPStorage.sol";
import "../../src/settlement/interfaces/IPCP.sol";
import {TransparentUpgradeableProxy} from "@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol";
import {ProxyAdmin} from "@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol";
import "@openzeppelin/contracts/utils/Strings.sol";

contract PCPTest is Test {
    MOVETokenDev public moveToken;
    MovementStaking public staking;
    PCP public pcp;
    ProxyAdmin public admin;
    string public moveSignature = "initialize(address)";
    string public stakingSignature = "initialize(address)";
    string public pcpSignature = "initialize(address,uint256,uint256,uint256,address[],uint256,address)";
    uint256 epochDuration = 7200 seconds;
    uint256 postconfirmerDuration = epochDuration/4;
    bytes32 honestCommitmentTemplate = keccak256(abi.encodePacked(uint256(1), uint256(2), uint256(3)));
    bytes32 honestBlockIdTemplate = keccak256(abi.encodePacked(uint256(1), uint256(2), uint256(3)));
    bytes32 dishonestCommitmentTemplate = keccak256(abi.encodePacked(uint256(3), uint256(2), uint256(1)));
    bytes32 dishonestBlockIdTemplate = keccak256(abi.encodePacked(uint256(3), uint256(2), uint256(1)));
    
    // make an honest commitment
    function makeHonestCommitment(uint256 height) internal view returns (PCPStorage.SuperCommitment memory) {
        return PCPStorage.SuperCommitment({
            height: height,
            commitment: honestCommitmentTemplate,
            blockId: honestBlockIdTemplate
        });
    }
       
    // make a dishonest commitment
    function makeDishonestCommitment(uint256 height) internal view returns (PCPStorage.SuperCommitment memory) {
        return PCPStorage.SuperCommitment({
            height: height,
            commitment: dishonestCommitmentTemplate,
            blockId: dishonestBlockIdTemplate
        });
    }


    // ----------------------------------------------------------------
    // -------- Helper functions --------------------------------------
    // ----------------------------------------------------------------

    function setUp() public {
        MOVETokenDev moveTokenImplementation = new MOVETokenDev();
        MovementStaking stakingImplementation = new MovementStaking();
        PCP pcpImplementation = new PCP();

        // Contract PCPTest is the admin
        admin = new ProxyAdmin(address(this));

        // Deploy proxies
        bytes memory initData = abi.encodeWithSignature(moveSignature, address(this));
        TransparentUpgradeableProxy moveProxy = new TransparentUpgradeableProxy(
            address(moveTokenImplementation),
            address(admin),
            initData
        );
        // Set up the moveToken variable to interact with the proxy
        moveToken = MOVETokenDev(address(moveProxy));

        bytes memory stakingInitData = abi.encodeWithSignature(stakingSignature, IMintableToken(address(moveProxy)));
        TransparentUpgradeableProxy stakingProxy = new TransparentUpgradeableProxy(
            address(stakingImplementation),
            address(admin),
            stakingInitData
        );
        // Set up the staking variable to interact with the proxy
        staking = MovementStaking(address(stakingProxy));

        address[] memory custodians = new address[](1);
        // TODO while this works it is hard to access that this is the moveToken. We should not rely on the custodian array
        custodians[0] = address(moveProxy);  

        bytes memory pcpInitData = abi.encodeWithSignature(
            pcpSignature, 
            stakingProxy,               // _stakingContract, address of staking contract
            0,                          // _lastPostconfirmedSuperBlockHeight, start from genesis
            5,                          // _leadingSuperBlockTolerance, max blocks ahead of last confirmed
            epochDuration,              // _epochDuration, how long an epoch lasts, constant stakes in that time
            custodians,                 // _custodians, array with moveProxy address
            postconfirmerDuration,      // _postconfirmerDuration, how long an postconfirmer serves
            // TODO can we replace the following line with the moveToken address?
            address(moveProxy)          // _moveTokenAddress, the primary custodian for rewards in the staking contract
        );
        TransparentUpgradeableProxy pcpProxy = new TransparentUpgradeableProxy(
            address(pcpImplementation),
            address(admin),
            pcpInitData
        );

        pcp = PCP(address(pcpProxy));
        pcp.setOpenAttestationEnabled(true);

        assertEq(staking.getEpochDuration(address(pcp)), epochDuration, "Epoch duration not set correctly");
        // set the min commitment age for postconfirmation to 0 to make the tests easier
        pcp.setMinCommitmentAgeForPostconfirmation(0);
        assertEq(pcp.getMinCommitmentAgeForPostconfirmation(), 0, "The default min commitment age for tests is set to 0");
        // set the max postconfirmer non-reactivity time to 0 to make the tests easier
        pcp.setPostconfirmerPrivilegeDuration(0);
        assertEq(pcp.getPostconfirmerPrivilegeDuration(), 0, "The default max postconfirmer non-reactivity time for tests is set to 0");
    }

    // Helper function to setup genesis with 1 attester and their stake
    function setupGenesisWithOneAttester(uint256 stakeAmount) internal returns (address attester) {
        moveToken.mint(address(pcp), stakeAmount*100); // PCP needs tokens to pay rewards
        // PCP needs to approve staking contract to spend its tokens
        vm.prank(address(pcp));
        moveToken.approve(address(staking), type(uint256).max);
        
        attester = payable(vm.addr(1));
        staking.whitelistAddress(attester);
        moveToken.mint(attester, stakeAmount);
        vm.prank(attester);
        moveToken.approve(address(staking), stakeAmount);
        vm.prank(attester);
        staking.stake(address(pcp), moveToken, stakeAmount);
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), attester), stakeAmount);
        assertEq(pcp.getTotalStakeForAcceptingEpoch(), stakeAmount);

        // TODO check why the registering did not work in the setup function
        // setup the epoch duration
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        staking.registerDomain(epochDuration, custodians);

        // TODO this seems odd that we need to do this here.. check for correctnes of this approach
        pcp.grantRole(pcp.DEFAULT_ADMIN_ROLE(), address(pcp));

        // attempt genesis when L1 chain has already advanced into the future
        // vm.warp(3*epochDuration);

        // End genesis ceremony
        vm.prank(address(pcp));
        pcp.acceptGenesisCeremony();

        // Verify stakes
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), attester), stakeAmount, "Alice's stake not correct");
        assertEq(pcp.getTotalStakeForAcceptingEpoch(), stakeAmount, "Total stake not correct");
    }


    // Helper function to setup genesis with 3 attesters and their stakes
    function setupGenesisWithThreeAttesters(
        uint256 aliceStakeAmount,
        uint256 bobStakeAmount, 
        uint256 carolStakeAmount
    ) internal returns (address alice, address bob, address carol) {
        uint256 totalStakeAmount = aliceStakeAmount + bobStakeAmount + carolStakeAmount;

        moveToken.mint(address(pcp), totalStakeAmount*100); // PCP needs tokens to pay rewards
        // PCP needs to approve staking contract to spend its tokens
        vm.prank(address(pcp));
        moveToken.approve(address(staking), type(uint256).max);

        // Create attesters
        alice = payable(vm.addr(1));
        bob = payable(vm.addr(2));
        carol = payable(vm.addr(3));
        address[] memory attesters = new address[](3);
        attesters[0] = alice;
        attesters[1] = bob;
        attesters[2] = carol;

        // Setup attesters
        for (uint i = 0; i < attesters.length; i++) {
            staking.whitelistAddress(attesters[i]);
            moveToken.mint(attesters[i], totalStakeAmount); // we mint the total stake amount for each attester, just so we have some buffer
            vm.prank(attesters[i]);
            moveToken.approve(address(staking), totalStakeAmount);
        }

        // Stake
        vm.prank(alice);
        staking.stake(address(pcp), moveToken, aliceStakeAmount);
        vm.prank(bob);
        staking.stake(address(pcp), moveToken, bobStakeAmount);
        vm.prank(carol);
        staking.stake(address(pcp), moveToken, carolStakeAmount);

        // Verify stakes
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), alice), aliceStakeAmount, "Alice's stake not correct");
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), bob), bobStakeAmount, "Bob's stake not correct");
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), carol), carolStakeAmount, "Carol's stake not correct");
        assertEq(pcp.getTotalStakeForAcceptingEpoch(), totalStakeAmount, "Total stake not correct");

        // TODO check why the registering did not work in the setup function
        // setup the epoch duration
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        staking.registerDomain(epochDuration, custodians);

        // TODO this seems odd that we need to do this here.. check for correctnes of this approach
        pcp.grantRole(pcp.DEFAULT_ADMIN_ROLE(), address(pcp));

        // attempt genesis when L1 chain has already advanced into the future
        // vm.warp(3*epochDuration);

        // End genesis ceremony
        vm.prank(address(pcp));
        pcp.acceptGenesisCeremony();

        // Verify stakes
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), alice), aliceStakeAmount, "Alice's stake not correct");
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), bob), bobStakeAmount, "Bob's stake not correct");
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), carol), carolStakeAmount, "Carol's stake not correct");
        assertEq(pcp.getTotalStakeForAcceptingEpoch(), totalStakeAmount, "Total stake not correct");
    } 

    /// @notice Helper function to setup a new signer with staking
    /// @param seed used to generate signer address
    /// @param stakeAmount Amount of tokens to stake
    /// @return newStakedAttester Address of the newly setup signer
    function newStakedAttester(uint256 seed, uint256 stakeAmount) internal returns (address) {
        address payable newAttester = payable(vm.addr(seed));
        staking.whitelistAddress(newAttester);
        moveToken.mint(newAttester, stakeAmount * 3);  // Mint 3x for flexibility    
        vm.prank(newAttester);
        moveToken.approve(address(staking), stakeAmount);
        vm.prank(newAttester);
        staking.stake(address(pcp), moveToken, stakeAmount);        
        assert(pcp.getStakeForAcceptingEpoch(address(moveToken), newAttester) == stakeAmount);

        return newAttester;
    }

    // we need this function to print the commitment in a readable format, e.g. for logging purposes
    function commitmentToHexString(bytes32 commitment) public pure returns (string memory) {
        bytes memory alphabet = "0123456789abcdef";
        bytes memory str = new bytes(2 + 32 * 2);
        str[0] = "0";
        str[1] = "x";
        for (uint i = 0; i < 32; i++) {
            str[2+i*2] = alphabet[uint8(commitment[i] >> 4)];
            str[2+i*2+1] = alphabet[uint8(commitment[i] & 0x0f)];
        }
        return string(str);
    }

    // this function checks if the honest attesters have a supermajority of the stake
    function logStakeInfo(address[] memory _honestAttesters, address[] memory _dishonestAttesters) internal view returns (bool) {
        // calculate the honest attesters stake
        uint256 honestStake = 0;
        for (uint256 k = 0; k < _honestAttesters.length; k++) {
            honestStake += pcp.getStakeForAcceptingEpoch(address(moveToken), _honestAttesters[k]);
        }

        // calculate the dishonest attesters stake
        uint256 dishonestStake = 0;
        for (uint256 k = 0; k < _dishonestAttesters.length; k++) {
            dishonestStake += pcp.getStakeForAcceptingEpoch(address(moveToken), _dishonestAttesters[k]);
        }
        
        uint256 supermajorityStake = 2 * (honestStake + dishonestStake) / 3 + 1;
        return honestStake >= supermajorityStake;
    }

    // remove an attester from the attesters array
    function removeAttester(address attester, address[] storage attesters, uint256 attesterStake) internal {
        vm.prank(attester);
        staking.unstake(address(pcp), address(moveToken), attesterStake);
        
        // Find and remove attester from array using swap and pop
        for (uint i = 0; i < attesters.length; i++) {
            if (attesters[i] == attester) {
                attesters[i] = attesters[attesters.length - 1];
                attesters.pop();
                break;
            }
        }
    }

    // ----------------------------------------------------------------
    // -------- General tests ----------------------------------------
    // ----------------------------------------------------------------

    function testCannotInitializeTwice() public {
        address[] memory custodians = new address[](1);
        custodians[0] = address(moveToken);
        // Attempt to initialize again should fail
        vm.expectRevert(bytes4(0xf92ee8a9));
        pcp.initialize(staking, 0, 5, 10 seconds, custodians,120 seconds, address(moveToken));
    }

    function testSetAcceptingEpochOnlyDomain() public {
        address alice = setupGenesisWithOneAttester(1000);
        vm.warp(pcp.getEpochDuration()*2);

        // Try to set accepting epoch from a non-domain address
        vm.prank(alice);
        assertEq(pcp.hasRole(pcp.COMMITMENT_ADMIN(), alice), false);
        vm.prank(alice);
        vm.expectRevert("UNAUTHORIZED");
        staking.setAcceptingEpoch(address(pcp), 1);
        console.log("Unauthorized attempt failed as expected");

        // Ensure the PCP contract has the COMMITMENT_ADMIN role
        uint256 presentEpoch = pcp.getPresentEpoch();
        assertEq(pcp.hasRole(pcp.COMMITMENT_ADMIN(), address(this)), true);
        pcp.grantRole(pcp.COMMITMENT_ADMIN(), address(this));
        // check that pcp has the COMMITMENT_ADMIN role
        assertEq(pcp.hasRole(pcp.COMMITMENT_ADMIN(), address(this)), true);
        pcp.setAcceptingEpoch(presentEpoch - 1);
        assertEq(staking.getAcceptingEpoch(address(pcp)), presentEpoch - 1);
    }

    /// @notice Test that an attester cannot submit multiple commitments for the same height
    function testAttesterCannotCommitTwice() public {
        // three well-funded signers
        (, , address carol) = setupGenesisWithThreeAttesters(1, 1, 1);

        // carol will be dishonest
        vm.prank(carol);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));

        // carol will try to sign again
        vm.prank(carol);
        vm.expectRevert(IPCP.AttesterAlreadyCommitted.selector);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));
    }

    /// @notice Test that honest supermajority succeeds despite dishonest attesters
    function testHonestSupermajoritySucceeds() public {
        // Setup with alice+bob having supermajority (67%)
        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(2, 1, 1);

        // Dishonest carol submits first
        vm.prank(carol);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));

        // Honest majority submits
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(bob); 
        pcp.submitSuperCommitment(makeHonestCommitment(1));

        // Trigger postconfirmation with majority
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();

        // Verify honest commitment was postconfirmed
        PCPStorage.SuperCommitment memory retrievedCommitment = pcp.getPostconfirmedCommitment(1);
        assertEq(retrievedCommitment.commitment, honestCommitmentTemplate);
        assertEq(retrievedCommitment.blockId, honestBlockIdTemplate);
        assertEq(retrievedCommitment.height, 1);
    }


    /// @notice Test that no postconfirmation happens when stakes are equal
    function testNoPostconfirmationWithEqualStakes() public {
        // Setup with equal stakes (no possible supermajority)
        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(1, 1, 1);

        // Honnest commitments
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(bob);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        // Dishonest commitment
        vm.prank(carol);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));

        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 0, "Height should not advance - Alice");
        // Verify no commitment was postconfirmed
        PCPStorage.SuperCommitment memory retrievedCommitment = pcp.getPostconfirmedCommitment(1);
        assertEq(retrievedCommitment.height, 0, "No commitment should be postconfirmed");
        assertEq(retrievedCommitment.commitment, bytes32(0), "No commitment should be postconfirmed");
    }

    /// @notice Test that rollover handling works with dishonesty
    function testRolloverHandlingWithDishonesty() public {
        uint256 L1BlockTimeStart = 30 * epochDuration; // TODO why though?
        vm.warp(L1BlockTimeStart);

        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(2, 1, 1);

        // dishonest carol
        vm.prank(carol);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));

        // honest majority
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(bob);
        pcp.submitSuperCommitment(makeHonestCommitment(1));

        // now we move to next epoch
        vm.warp(L1BlockTimeStart + epochDuration);

        // postconfirm and rollover
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();

        // check that roll over happened
        assertEq(pcp.getAcceptingEpoch(), pcp.getPresentEpoch());
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), alice), 2);
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), bob), 1);
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), carol), 1);
        PCPStorage.SuperCommitment memory retrievedCommitment = pcp.getPostconfirmedCommitment(1);
        assert(retrievedCommitment.commitment == honestCommitmentTemplate);
        assert(retrievedCommitment.blockId == honestBlockIdTemplate);
        assert(retrievedCommitment.height == 1);
    }

    // State variable (at contract level)
    // dynamic array defined as state variable to permit to use push
    address[] honestAttesters = new address[](0);
    address[] dishonestAttesters = new address[](0);

    /// @notice Tests the PCP system's resilience with changing Attester sets by:
    /// 1. Starting with honest majority (2/3 honest, 1/3 dishonest)
    /// 2. Adding new attester periodically
    /// 3. Removing attester periodically
    /// 4. Verifying honest commitments prevail over 50 reorganizations
    // TODO i am not convinced we need such a complicated unit test here. Consider what this is trying to achieve and break it up.
    function testChangingAttesterSet() public {
        // TODO explain why we need to pause gas metering here
        vm.pauseGasMetering();
        uint256 attesterStake = 1; 
        uint256 L1BlockTimeStart = 30 * epochDuration; // TODO why though?
        uint256 L1BlockTime = L1BlockTimeStart;
        vm.warp(L1BlockTime);
        uint256 changingAttesterSetEvents = 10; // number of times we change the attester set
        uint256 commitmentHeights = 1; // number of commitments after each change event

        // alice needs to have attesterStake + 1 so we reach supermajority
        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(attesterStake+1, attesterStake, attesterStake);
        moveToken.mint(address(pcp), 100); // PCP needs tokens to pay rewards

        // honest attesters
        honestAttesters.push(alice);
        honestAttesters.push(bob);

        // dishonest attesters
        dishonestAttesters.push(carol);

        for (uint256 i = 0; i < changingAttesterSetEvents; i++) {
            for (uint256 j = 0; j < commitmentHeights; j++) {
                uint256 superBlockHeightNow = i * commitmentHeights + j + 1;

                L1BlockTime += epochDuration;
                vm.warp(L1BlockTime);
                // alice triggers rollover
                vm.prank(alice);
                pcp.postconfirmSuperBlocksAndRollover();

                // get the assigned epoch for the superblock height
                // commit roughly half of dishones attesters 
                PCPStorage.SuperCommitment memory dishonestCommitment = makeDishonestCommitment(superBlockHeightNow);
                for (uint256 k = 0; k < dishonestAttesters.length / 2; k++) {
                    vm.prank(dishonestAttesters[k]);
                    pcp.submitSuperCommitment(dishonestCommitment);
                }

                // commit honestly
                PCPStorage.SuperCommitment memory honestCommitment = makeHonestCommitment(superBlockHeightNow);
                for (uint256 k = 0; k < honestAttesters.length; k++) {
                    vm.prank(honestAttesters[k]);
                    pcp.submitSuperCommitment(honestCommitment);
                }

                // TODO: The following does not serve any purpose, as enough attesters are already committed
                // commit dishonestly the rest
                // for (uint256 k = dishonestAttesters.length / 2; k < dishonestAttesters.length; k++) {
                //     vm.prank(dishonestAttesters[k]);
                //     pcp.submitSuperCommitment(dishonestCommitment);
                // }

                vm.prank(alice);
                pcp.postconfirmSuperBlocksAndRollover();

                PCPStorage.SuperCommitment memory retrievedCommitment = pcp.getPostconfirmedCommitment(superBlockHeightNow);
                assert(retrievedCommitment.commitment == honestCommitment.commitment);
                assert(retrievedCommitment.blockId == honestCommitment.blockId);
                assert(retrievedCommitment.height == superBlockHeightNow);

            }

            uint256 honestStakedAttesterLength = honestAttesters.length;
            uint256 dishonestStakedAttesterLength = dishonestAttesters.length;

            // TODO replace the below with this function call
            // address newAttester = newStakedAttester(4 + i, attesterStake); // TODO why 4 not 3?

            // add a new attester
            address payable newAttester = payable(vm.addr(4 + i));
            
            staking.whitelistAddress(newAttester);
            moveToken.mint(newAttester, 3*attesterStake);
            vm.prank(newAttester);
            moveToken.approve(address(staking), attesterStake);
            vm.prank(newAttester);
            staking.stake(address(pcp), moveToken, attesterStake);

            L1BlockTime += epochDuration;
            vm.warp(L1BlockTime);

            // Force rollover by having alice (who has majority stake) call postconfirmSuperBlocksAndRollover
            vm.prank(alice);  // alice has attesterStake+1 from setup
            pcp.postconfirmSuperBlocksAndRollover();
            // confirm that the new attester has stake
            assert(pcp.getStakeForAcceptingEpoch(address(moveToken), newAttester) == attesterStake);

            // push every third signer to dishonest attesters. If pushed earlier we fail a super majority test.
            if (i % 3 == 2) {
                dishonestAttesters.push(newAttester);
                assert(dishonestAttesters.length == dishonestStakedAttesterLength + 1);
            } else {
                honestAttesters.push(newAttester);
                assert(honestAttesters.length == honestStakedAttesterLength + 1);
            }

            // TODO explain here why we do the following
            if (i % 5 == 4) {
                // removeAttester(dishonestAttesters[0], dishonestAttesters, attesterStake);
            }
            // TODO only having this but not the above is a more complex interesting scenario that would fail the line as we rollover in the postconfirmation:  
            // assert(retrievedCommitment.commitment == honestCommitment.commitment); (above)
            // this is interesting but it requires moving this upwards in the code and maybe not applying both
            if (i % 8 == 7) {
                // remove an honest attester
                // removeAttester(honestAttesters[0], honestAttesters, attesterStake);
            }

            assert(logStakeInfo(honestAttesters, dishonestAttesters));

            // L1BlockTime += 5;
            // vm.warp(L1BlockTime);
            // assert the time here
            assertEq(L1BlockTime, L1BlockTimeStart + (i+1) * (commitmentHeights + 1) * epochDuration);
        }
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), changingAttesterSetEvents * commitmentHeights);
    }

    function testForcedAttestation() public {
        vm.pauseGasMetering();

        uint256 blockTime = 300;
        vm.warp(blockTime);

        // default signer should be able to force commitment
        PCPStorage.SuperCommitment memory forcedCommitment = makeDishonestCommitment(1);
        pcp.forceLatestCommitment(forcedCommitment);

        // get the latest commitment
        PCPStorage.SuperCommitment memory retrievedCommitment = pcp.getPostconfirmedCommitment(1);
        assertEq(retrievedCommitment.blockId, forcedCommitment.blockId);
        assertEq(retrievedCommitment.commitment, forcedCommitment.commitment);
        assertEq(retrievedCommitment.height, forcedCommitment.height);

        // create an unauthorized signer
        address payable alice = payable(vm.addr(1));

        // try to force a different commitment with unauthorized user
        PCPStorage.SuperCommitment memory badForcedCommitment = makeHonestCommitment(1);
        
        // Alice should not have COMMITMENT_ADMIN role
        assertEq(pcp.hasRole(pcp.COMMITMENT_ADMIN(), alice), false);
        
        vm.prank(alice);
        vm.expectRevert("FORCE_LATEST_COMMITMENT_IS_COMMITMENT_ADMIN_ONLY");
        pcp.forceLatestCommitment(badForcedCommitment);
    }

    /// @notice Test that a confirmation and postconfirmation by single attester works
    function testSimplePostconfirmation() public {
        // Setup - single attester
        address payable alice = payable(vm.addr(1));
        staking.whitelistAddress(alice);
        moveToken.mint(alice, 100);
        
        // Stake
        vm.prank(alice);
        moveToken.approve(address(staking), 100);
        vm.prank(alice);
        staking.stake(address(pcp), moveToken, 100);
        
        // End genesis ceremony
        // vm.prank(address(pcp)); // TODO is this needed?
        pcp.acceptGenesisCeremony();
        
        // confirm current superblock height
        uint256 currentHeight = pcp.getLastPostconfirmedSuperBlockHeight();

        // Create and submit commitment
        uint256 targetHeight = 1;
        PCPStorage.SuperCommitment memory commitment = PCPStorage.SuperCommitment({
            height: targetHeight,
            commitment: keccak256(abi.encodePacked(uint256(1))),
            blockId: keccak256(abi.encodePacked(uint256(1)))
        });

        // Submit commitment
        vm.prank(alice);
        pcp.submitSuperCommitment(commitment);
        
        // Verify commitment was stored
        PCPStorage.SuperCommitment memory stored = pcp.getCommitmentByAttester(targetHeight, alice);
        assert(stored.commitment == commitment.commitment);
        
        // Attempt postconfirmation
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        
        // Verify postconfirmation worked
        PCPStorage.SuperCommitment memory postconfirmed = pcp.getPostconfirmedCommitment(targetHeight);
        assert(postconfirmed.commitment == commitment.commitment);

        // confirm current superblock height
        uint256 currentHeightNew = pcp.getLastPostconfirmedSuperBlockHeight();
        assertEq(currentHeightNew, currentHeight + 1);

    }


    /// @notice Test that a confirmation and postconfirmation by single attester works if they have majority stake
    function testPostconfirmationWithMajorityStake() public {
        // Setup with alice having majority
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(34, 33, 33);
        
        // Create commitment for height 1
        uint256 targetHeight = 1;
        
        PCPStorage.SuperCommitment memory commitment = makeHonestCommitment(targetHeight);

        // Submit commitments
        vm.prank(alice);
        pcp.submitSuperCommitment(commitment);
        vm.prank(bob);
        pcp.submitSuperCommitment(commitment);

        // Verify commitments were stored
        PCPStorage.SuperCommitment memory aliceCommitment = pcp.getCommitmentByAttester(targetHeight, alice);
        PCPStorage.SuperCommitment memory bobCommitment = pcp.getCommitmentByAttester(targetHeight, bob);
        assert(aliceCommitment.commitment == commitment.commitment);
        assert(bobCommitment.commitment == commitment.commitment);

        // Verify postconfirmer state
        assert(pcp.isWithinPostconfirmerPrivilegeDuration(commitment));
        assertEq(pcp.getSuperBlockHeightAssignedEpoch(targetHeight), pcp.getAcceptingEpoch());

        // Attempt postconfirmation
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();

        // Verify postconfirmation
        PCPStorage.SuperCommitment memory postconfirmed = pcp.getPostconfirmedCommitment(targetHeight);
        assert(postconfirmed.commitment == commitment.commitment);
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), targetHeight);
    }

    /// @notice Test that a confirmation and postconfirmation by single attester fails if they have majority stake
    function testPostconfirmationWithoutMajorityStake() public {
        // Setup with no one having majority
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(33, 33, 34);
        
        // Create commitment for height 1
        uint256 targetHeight = 1;
        
        PCPStorage.SuperCommitment memory commitment = makeHonestCommitment(targetHeight);

        // Submit commitments
        vm.prank(alice);
        pcp.submitSuperCommitment(commitment);
        vm.prank(bob);
        pcp.submitSuperCommitment(commitment);

        // Verify commitments were stored
        PCPStorage.SuperCommitment memory aliceCommitment = pcp.getCommitmentByAttester(targetHeight, alice);
        PCPStorage.SuperCommitment memory bobCommitment = pcp.getCommitmentByAttester(targetHeight, bob);
        assert(aliceCommitment.commitment == commitment.commitment);
        assert(bobCommitment.commitment == commitment.commitment);

        // Verify postconfirmer state
        assert(pcp.isWithinPostconfirmerPrivilegeDuration(commitment));
        assertEq(pcp.getSuperBlockHeightAssignedEpoch(targetHeight), pcp.getAcceptingEpoch());

        // Attempt postconfirmation - this should fail because there's no supermajority
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();

        // Verify height hasn't changed (postconfirmation didn't succeed)
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 0);
    }

    /// @notice Test that stake activation and postconfirmation works away from the Genesis. 
    /// TODO at genesis this behaves different and we should test this, specifically. unstake and stake are directly applied to epoch 0 until it is rolled over
    function testStakeActivationAndPostconfirmation() public {
        // Setup initial attesters with equal stakes, but Carol hasn't staked yet
        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(1, 1, 0);

        // Create commitment for height 1 by the only stable attester
        PCPStorage.SuperCommitment memory commitment = makeHonestCommitment(1);
        vm.prank(bob);
        pcp.submitSuperCommitment(commitment);

        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 0, "Last postconfirmed superblock height should be 0, as no supermajority was reached (2/3 < threshold)");
        assertEq(pcp.getAcceptingEpoch(),0, "Accepting epoch should be 0");

        vm.warp(epochDuration);
        assertEq(pcp.getPresentEpoch(),1, "Present epoch should be 1");
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(),1, "Accepting epoch should be 1");

        vm.prank(carol);
        staking.stake(address(pcp), moveToken, 1);
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), carol), 0, "Carol's stake is still 0.");
        // Alice unstakes so her commitment is not counted in the next accepting epoch
        vm.prank(alice);
        staking.unstake(address(pcp), address(moveToken), 1);
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), alice), 1, "Alice's stake should still be 1");
        assertEq(staking.getUnstake(address(pcp), 2, address(moveToken), alice), 1, "Alice's unstake in epoch 2 should be 1");

        // Warp to next epoch 
        vm.warp(2*epochDuration);
        assertEq(pcp.getPresentEpoch(), 2, "Present epoch should be 2");
        assertEq(pcp.getAcceptingEpoch(), 1, "Accepting epoch should be 1");

        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 2, "Accepting epoch should be 2");

        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), carol), 1, "Carol's stake should already be active");
        assertEq(pcp.getStakeForAcceptingEpoch(address(moveToken), alice), 0, "Alice's stake should be 0");
        assertEq(moveToken.balanceOf(alice), 2, "Alice's balance should be 2");

        // Carol commits to height 1
        vm.prank(carol);
        pcp.submitSuperCommitment(commitment);

        // perform postconfirmation
        vm.prank(carol);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1, "Last postconfirmed superblock height should be 1, as supermajority was reached (2/2 > threshold)");
    }

    function testSetMinCommitmentAge() public {
        // Set min commitment age to a too long value
        vm.expectRevert(PCP.minCommitmentAgeForPostconfirmationTooLong.selector);
        pcp.setMinCommitmentAgeForPostconfirmation(epochDuration);

        // Set min commitment age to 1/10 of epochDuration
        uint256 minAge = epochDuration/10;
        pcp.setMinCommitmentAgeForPostconfirmation(minAge);
        assertEq(pcp.minCommitmentAgeForPostconfirmation(), minAge, "Min commitment age should be updated to 1/10 of epochDuration");
    }

    function testMinCommitmentAge() public {
        // Setup with Alice having supermajority stake
        address alice = setupGenesisWithOneAttester(1);
        assertEq(pcp.getMinCommitmentAgeForPostconfirmation(), 0, "The unset min commitment age should be 0");
        uint256 minAge = 1 minutes;
        pcp.setMinCommitmentAgeForPostconfirmation(minAge);
        assertEq(pcp.getMinCommitmentAgeForPostconfirmation(), minAge, "Min commitment age should be updated to 1 minutes");
        
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 0, "Immediate postconfirmation should fail.");
        
        vm.warp(block.timestamp + minAge);  // note that time starts at 1, not 0
        // Now postconfirmation should succeed
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1);
    }


    // ----------------------------------------------------------------
    // -------- Postconfirmer tests --------------------------------------
    // ----------------------------------------------------------------

    /// @notice Test that getPostconfirmerStartTime correctly calculates term start times
    function testPostconfirmerStartTime() public {
        // Test at block 0
        assertEq(block.timestamp, 1, "Current time should be 1"); // TODO why is it 1? and not 0?
        assertEq(postconfirmerDuration, pcp.getPostconfirmerDuration(), "Postconfirmer term should be correctly set");
        assertEq(pcp.getPostconfirmerStartTime(), 0, "Postconfirmer term should start at (1) time 0");

        // Test at half an postconfirmer term
        vm.warp(postconfirmerDuration-1);
        assertEq(pcp.getPostconfirmerStartTime(), 0, "Postconfirmer term should start at (2) time 0");

        // Test at an postconfirmer term boundary
        vm.warp(postconfirmerDuration);
        assertEq(pcp.getPostconfirmerStartTime(), postconfirmerDuration, "Postconfirmer term should start at (3) time postconfirmerDuration");

        // Test at an postconfirmer term boundary
        vm.warp(postconfirmerDuration+1);
        assertEq(pcp.getPostconfirmerStartTime(), postconfirmerDuration, "Postconfirmer term should start at (4) time postconfirmerDuration");

        // Test at 1.5 postconfirmer terms
        vm.warp(2 * postconfirmerDuration );
        assertEq(pcp.getPostconfirmerStartTime(), 2 * postconfirmerDuration, "Postconfirmer term should start at (5) time 2 * postconfirmerDuration");        
    }

    /// @notice Test setting postconfirmer duration with validation
    function testSetPostconfirmerDuration() public {
        // Check the epoch duration is set correctly
        assertEq(epochDuration, staking.getEpochDuration(address(pcp)));        
        // Test valid duration (less than half epoch duration)
        uint256 validDuration = epochDuration / 2 - 1;
        pcp.setPostconfirmerDuration(validDuration);
        assertEq(pcp.getPostconfirmerDuration(), validDuration, "Duration should be updated to valid value");

        // Test duration too long compared to epoch (>= epochDuration/2)
        uint256 invalidDuration = epochDuration / 2;
        vm.expectRevert(PCP.PostconfirmerDurationTooLongForEpoch.selector);
        pcp.setPostconfirmerDuration(invalidDuration);
        assertEq(pcp.getPostconfirmerDuration(), validDuration, "Duration should remain at previous valid value");

        // Test duration equal to epoch duration (should fail)
        vm.expectRevert(PCP.PostconfirmerDurationTooLongForEpoch.selector);
        pcp.setPostconfirmerDuration(epochDuration);
        assertEq(pcp.getPostconfirmerDuration(), validDuration, "Duration should remain at previous valid value");
    }

    /// @notice Test that getPostconfirmer correctly selects an postconfirmer based on block hash
    function testGetPostconfirmer() public {
        // Setup with three attesters with equal stakes
        (, address bob, address carol) = setupGenesisWithThreeAttesters(1, 1, 1);
        uint256 myPostconfirmerDuration = 13;
        pcp.setPostconfirmerDuration(myPostconfirmerDuration);
        assertEq(myPostconfirmerDuration,pcp.getPostconfirmerDuration(),"Postconfirmer duration not set correctly");

        address initialPostconfirmer = pcp.getPostconfirmer();
        assertEq(initialPostconfirmer, bob, "Postconfirmer should be bob");

        vm.warp(myPostconfirmerDuration-1); 
        assertEq(pcp.getPostconfirmer(), initialPostconfirmer, "Postconfirmer should not change within term");

        // Move two postconfirmer terms (moving one resulted still in bob as postconfirmer with current randomness)
        vm.warp(2*myPostconfirmerDuration); 
        address newPostconfirmer = pcp.getPostconfirmer();
        assertEq(pcp.getPostconfirmerStartTime(),2*myPostconfirmerDuration,"Postconfirmer start time should be myPostconfirmerDuration");
        assertEq(newPostconfirmer, carol, "New postconfirmer should be Carol");
    }


    // ----------------------------------------------------------------
    // -------- Attester reward tests --------------------------------------
    // ----------------------------------------------------------------

    function testAttesterRewardPoints() public {
        // Setup with Alice having supermajority-enabling stake
        (address alice, address bob, address carol) = setupGenesisWithThreeAttesters(2, 1, 1);
        uint256 aliceInitialBalance = moveToken.balanceOf(alice);
        uint256 bobInitialBalance = moveToken.balanceOf(bob);
        uint256 carolInitialBalance = moveToken.balanceOf(carol);
        pcp.setRewardPerPostconfirmationPoint(0);

        // Exit genesis epoch
        vm.warp(epochDuration);
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 1, "Should have exited genesis");

        // Submit commitments for height 1 honestly (Alice and Bob > 2/3)
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(bob);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        vm.prank(carol);
        pcp.submitSuperCommitment(makeDishonestCommitment(1));

        // Check initial reward points
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), alice), 0, "Alice should have no points yet");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), bob), 0, "Bob should have no points yet");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), carol), 0, "Carol should have no points yet");

        // Trigger postconfirmation
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();

        // New reward points
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), alice), 1, "Alice should have 1 points");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), bob), 1, "Bob should have 1 point");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), carol), 0, "Carol should have 0 point");

        // Alice and Carol commit to height 2 honestly (Alice + Carol > 2/3)
        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(2));
        vm.prank(bob);
        pcp.submitSuperCommitment(makeDishonestCommitment(2));
        vm.prank(carol);
        pcp.submitSuperCommitment(makeHonestCommitment(2));

        // Trigger postconfirmation, reward distribution by rolling over to next epoch
        vm.warp(2*epochDuration);
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 2, "Should be in epoch 2");

        // Verify rewards were distributed and points were cleared
        assertEq(pcp.attesterRewardPoints(pcp.getAcceptingEpoch(), alice), 0, "Alice's points should be cleared");
        assertEq(pcp.attesterRewardPoints(pcp.getAcceptingEpoch(), bob), 0, "Bob's points should be cleared");
        assertEq(pcp.attesterRewardPoints(pcp.getAcceptingEpoch(), carol), 0, "Carol's points should be cleared");
        assertEq(moveToken.balanceOf(alice), aliceInitialBalance + pcp.getStakeForAcceptingEpoch(address(moveToken), alice) * 2, "Alice reward not correct.");
        assertEq(moveToken.balanceOf(bob), bobInitialBalance + pcp.getStakeForAcceptingEpoch(address(moveToken), bob), "Bob reward not correct.");
        assertEq(moveToken.balanceOf(carol), carolInitialBalance + pcp.getStakeForAcceptingEpoch(address(moveToken), carol), "Carol reward not correct.");
    }

    /// @notice Test that postconfirmation rewards are distributed correctly when the postconfirmer is live
    function testPostconfirmationRewardsLivePostconfirmer() public {
        uint256 stake = 7;
        // alice has supermajority stake
        uint256 aliceStake = 3*stake;
        uint256 bobStake = stake;
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(aliceStake, bobStake, 0);
        uint256 aliceInitialBalance = moveToken.balanceOf(alice);
        uint256 bobInitialBalance = moveToken.balanceOf(bob);
        // set the max postconfirmer non-reactivity time to 1/4 epochDuration        
        pcp.setPostconfirmerPrivilegeDuration(epochDuration/4);

        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));
        // check that the first seen timestamp is set
        assertGt(pcp.getCommitmentFirstSeenAt(makeHonestCommitment(1)), 0, "Commitment first seen at should be set");

        assertEq(pcp.getPostconfirmer(), bob, "Bob should be the postconfirmer but its not");
        assertEq(pcp.isWithinPostconfirmerPrivilegeDuration(makeHonestCommitment(1)), true, "Postconfirmer should be live");

        // postconfirmer postconfirms while postconfirmer is live
        vm.prank(bob);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 0, "Should be in epoch 0");
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1, "Last postconfirmed superblock height should be 1");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), alice), 1, "Alice should have 1 attester points");
        assertEq(pcp.getPostconfirmerRewardPoints(pcp.getAcceptingEpoch(), bob), 1, "Bob should have 1 postconfirmer points");
        assertEq(moveToken.balanceOf(alice), aliceInitialBalance, "Alice should have not received any rewards yet");
        assertEq(moveToken.balanceOf(bob), bobInitialBalance, "Bob should not have received any rewards yet");

        // warp to next epoch
        vm.warp(epochDuration);
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1);
        assertEq(pcp.getAcceptingEpoch(), 1, "Should be in epoch 1");

        // Verify rewards:
        assertEq(moveToken.balanceOf(alice), aliceInitialBalance + aliceStake, "Alice should have received the rewards");
        assertEq(moveToken.balanceOf(bob), bobInitialBalance + bobStake, "Bob should have received the rewards");
    }

    /// @notice Test that volunteer postconfirmation rewards are not distributed to volunteer postconfirmer when postconfirmer is live
    // TODO once the postconfirmer can get postconfirm points within the postconfirmer privilege window, whether or not the height has previously been postconfirmed, this test should be updated
    function testVolunteerPostconfirmationRewardsLivePostconfirmer() public {
        uint256 aliceStake = 9;
        // alice has supermajority stake
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(aliceStake, 0, 0);
        uint256 aliceInitialBalance = moveToken.balanceOf(alice);
        uint256 bobInitialBalance = moveToken.balanceOf(bob);

        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));

        assertEq(pcp.getPostconfirmer(), alice, "Alice should be the postconfirmer since it is the only staked attester.");
        assertEq(pcp.isWithinPostconfirmerPrivilegeDuration(makeHonestCommitment(1)), true, "Postconfirmer should be live");

        // volunteer postconfirmer postconfirms while postconfirmer is live
        vm.prank(bob);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1);

        // bob should not get any postconfirmer rewards
        assertEq(moveToken.balanceOf(bob), bobInitialBalance, "Bob should not have received any rewards");
        assertEq(pcp.getPostconfirmerRewardPoints(pcp.getAcceptingEpoch(), bob), 0, "Bob should have 0 postconfirmer points");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), alice), 1, "Alice should have 1 attester points");
        assertEq(pcp.getPostconfirmerRewardPoints(pcp.getAcceptingEpoch(), alice), 0, "Alice should have 0 postconfirmer points");

        vm.warp(epochDuration);
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 1, "Should be in epoch 1");

        // alice should get the postconfirmer rewards
        assertEq(moveToken.balanceOf(bob), bobInitialBalance, "Bob should not have received any rewards");
        assertEq(moveToken.balanceOf(alice), aliceInitialBalance + aliceStake, "Alice should have received the attester rewards");
    }

    /// @notice Test that postconfirmation rewards are distributed to volunteer postconfirmer when postconfirmer is not live
    // TODO this test should probably be merged with the above test
    function testVolunteerPostconfirmationRewardsNotLivePostconfirmer() public {
        // alice has supermajority stake
        uint256 stake =13;
        uint256 aliceStake = 3*stake;
        uint256 bobStake = stake;
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(aliceStake, bobStake, 0);
        uint256 aliceInitialBalance = moveToken.balanceOf(alice);
        uint256 bobInitialBalance = moveToken.balanceOf(bob);
        uint256 thisPostconfirmerDuration = pcp.getPostconfirmerDuration();

        // set the time windows
        assertEq(pcp.getMinCommitmentAgeForPostconfirmation(), 0, "Min commitment age should be 0"); 
        uint256 thisPostconfirmerPriviledgeWindow = epochDuration/100;
        pcp.setPostconfirmerPrivilegeDuration(thisPostconfirmerPriviledgeWindow); 
        assertEq(pcp.getPostconfirmerPrivilegeDuration(), thisPostconfirmerPriviledgeWindow, "Max postconfirmer non-reactivity time should be 1/100 epochDuration");        
        assertGt(thisPostconfirmerDuration, thisPostconfirmerPriviledgeWindow, "Postconfirmer term should be greater than thisPostconfirmerPriviledgeWindow");

        vm.prank(alice);
        pcp.submitSuperCommitment(makeHonestCommitment(1));

        assertEq(pcp.getPostconfirmer(), bob, "bob should be the postconfirmer");
        assertEq(pcp.isWithinPostconfirmerPrivilegeDuration(makeHonestCommitment(1)), true, "Postconfirmer should be live");

        // warp out of postconfirmer privilege window
        vm.warp(block.timestamp + thisPostconfirmerPriviledgeWindow + 1 ); // TODO check why + 1 is needed
        assertEq(pcp.isWithinPostconfirmerPrivilegeDuration(makeHonestCommitment(1)), false, "Postconfirmer should not be live");
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 0, "Should be in epoch 0");
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1, "Last postconfirmed superblock height should be 1");
        assertEq(pcp.getAttesterRewardPoints(pcp.getAcceptingEpoch(), alice), 1, "Alice should have 1 attester points");
        assertEq(pcp.getPostconfirmerRewardPoints(pcp.getAcceptingEpoch(), alice), 1, "Alice should have 1 postconfirmer points");

        // warp to next epoch
        vm.warp(epochDuration);
        vm.prank(bob);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getAcceptingEpoch(), 1, "Should be in epoch 1");

        assertEq(moveToken.balanceOf(alice), aliceInitialBalance + aliceStake + aliceStake, "Alice should have received the attester and postconfirmer rewards");
        assertEq(moveToken.balanceOf(bob), bobInitialBalance, "Bob should have received no rewards");
    }
    
    // ----------------------------------------------------------------
    // -------- Postconfirmer reward tests --------------------------------------
    // ----------------------------------------------------------------


    // An postconfirmer that is in place for postconfirmerDuration time should be replaced by a new postconfirmer after their term ended.
    // TODO reward logic is not yet implemented
    function testPostconfirmerRewards() public {
        (address alice, address bob, ) = setupGenesisWithThreeAttesters(1, 1, 0);
        assertEq(pcp.getPostconfirmer(), bob, "Bob should be the postconfirmer");

        // make superBlock commitments
        PCPStorage.SuperCommitment memory initCommitment = makeHonestCommitment(1);
        vm.prank(alice);
        pcp.submitSuperCommitment(initCommitment);
        vm.prank(bob);
        pcp.submitSuperCommitment(initCommitment);

        // bob postconfirms and gets a reward
        vm.prank(bob);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 1);

        // make second superblock commitment
        PCPStorage.SuperCommitment memory secondCommitment = makeHonestCommitment(2);
        vm.prank(alice);
        pcp.submitSuperCommitment(secondCommitment);
        vm.prank(bob);
        pcp.submitSuperCommitment(secondCommitment);

        // alice can postconfirm, but does not get the reward
        // TODO check that bob did not get the reward
        vm.prank(alice);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 2);

        // bob tries to postconfirm, but already done by alice
        // TODO: bob should still get the reward
        vm.prank(bob);
        pcp.postconfirmSuperBlocksAndRollover();
        assertEq(pcp.getLastPostconfirmedSuperBlockHeight(), 2);
    }


}

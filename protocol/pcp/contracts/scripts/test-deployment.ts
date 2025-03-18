import { ethers, type Wallet } from "ethers";
import { ethers as hreEthers } from "hardhat";
import * as fs from 'fs';
import * as path from 'path';
import { IMovementStaking, MCR } from "../typechain-types";  // Make sure types are generated
import { IERC20 } from "@openzeppelin/contracts/token/ERC20/IERC20";  // Add at top with other imports
import { AccessControl } from "@openzeppelin/contracts/access/AccessControl";
import { IMCR } from "../typechain-types";  // Make sure types are generated
import { MCRStorage } from "../typechain-types";  // Add MCRStorage import




// - - - - - - - - - - Main - - - - - - - - - -

async function main() {
  try {
    console.log("\n=== Starting Deployment Verification ===\n");

    // Get contract addresses
    const deployments = JSON.parse(fs.readFileSync(
      path.join(__dirname, '../deployments/local.json'), 
      'utf8'
    ));
    
    // Setup contracts
    const moveToken = await hreEthers.getContractAt("MintableToken", deployments.MoveToken.proxy) as unknown as IERC20;
    const staking = await hreEthers.getContractAt("MovementStaking", deployments.MovementStaking.proxy) as unknown as IMovementStaking & AccessControl;
    const mcrContract = await hreEthers.getContractAt("MCR", deployments.MCR.proxy) as unknown as MCR;
    const mcr = await hreEthers.getContractAt("MCR", deployments.MCR.proxy) as unknown as IMCR;
    const { deployer, staker } = await setupAccounts();
    // await testEthTransfers(staker as unknown as ethers.Wallet);
    
    // Fund staker with MOVE first
    console.log("Funding staker with MOVE tokens...");
    const fundMoveTx = await moveToken.connect(deployer).transfer(staker.address, ethers.parseEther("3"));
    await fundMoveTx.wait();
    console.log("Staker MOVE balance:", ethers.formatEther(await moveToken.balanceOf(staker.address)));
    // await testMoveTransfers(moveToken, staker as unknown as ethers.Wallet);
    
    // Test Staking and MCR
    await testStaking(staking, moveToken, deployments.MCR.proxy, staker as unknown as ethers.Wallet);
    await testMCR(mcr, mcrContract, moveToken, staker as unknown as ethers.Wallet, deployer);
    await testUnstaking(staking, moveToken, deployments.MCR.proxy, staker as unknown as ethers.Wallet, mcr);


    console.log("\n=== Verification Complete ===");
  } catch (error) {
    console.error("Verification failed:", error);
    throw error;
  }
}

// - - - - - - - - - - The Functions - - - - - - - - - -

// This function sets up the deployer and staker accounts
async function setupAccounts() {
  const [deployer] = await hreEthers.getSigners();
  const staker = ethers.Wallet.createRandom().connect(hreEthers.provider);
  console.log("Deployer account:", deployer.address);
  console.log("Staker account:", staker.address);
  
  // Fund staker with ETH
  const fundTx = await deployer.sendTransaction({
    to: staker.address,
    value: ethers.parseEther("10")
  });
  await fundTx.wait();
  
  return { deployer, staker };
}

// This function tests the ETH transfers
async function testEthTransfers(staker: ethers.Wallet) {
  console.log("---------- Testing ETH transfers -------------------");
  const randomAddress = ethers.Wallet.createRandom().address;
  console.log("Staker sends 1 ETH to random address...");
  const tx = await staker.sendTransaction({
    to: randomAddress, 
    value: ethers.parseEther("1")
  });
  await tx.wait();
  
  const balance = await hreEthers.provider.getBalance(randomAddress);
  if (balance !== ethers.parseEther("1")) {
    throw new Error("ETH transfer failed");
  }
  console.log("ETH transfer successful");
}

// This function tests the MOVE token transfers
async function testMoveTransfers(moveToken: IERC20, staker: ethers.Wallet) {
  console.log("---------- Testing MOVE transfers -------------------");
  const randomAddress = ethers.Wallet.createRandom().address;
  console.log("Staker sends 1 MOVE to random address...");
  
  const tx = await moveToken.connect(staker).transfer(randomAddress, ethers.parseEther("1"));
  await tx.wait();
  
  const balance = await moveToken.balanceOf(randomAddress);
  if (balance !== ethers.parseEther("1")) {
    throw new Error("MOVE transfer failed");
  }
  console.log("MOVE transfer successful");
}

// This function tests the staking functionality
async function testStaking(
  staking: IMovementStaking & AccessControl,
  moveToken: IERC20,
  mcrAddress: string,
  staker: ethers.Wallet
) {
  console.log("\n=== Testing Staking Functionality ===");
  const stakeAmount = ethers.parseEther("1");
  
  // Whitelist and verify
  const txWhitelist = await staking.whitelistAddress(staker.address);
  await txWhitelist.wait();
  if (!await staking.hasRole(await staking.WHITELIST_ROLE(), staker.address)) {
    throw new Error("Whitelisting failed");
  }
  
  // Approve and verify
  const txApprove = await moveToken.connect(staker).approve(staking.target, stakeAmount);
  await txApprove.wait();
  if (await moveToken.allowance(staker.address, staking.target) !== stakeAmount) {
    throw new Error("Token approval failed");
  }
  
  // Stake and verify
  const txStake = await staking.connect(staker).stake(mcrAddress, moveToken.target, stakeAmount);
  await txStake.wait();
  if (await staking.getAttesterStakeForAcceptingEpoch(mcrAddress, staker.address) !== stakeAmount) {
    throw new Error("Staking failed");
  }
  
  console.log("Staking successful - amount:", ethers.formatEther(stakeAmount));

  const registeredAttesters = await staking.getRegisteredAttesters(mcrAddress);
  console.log("Registered Attesters:", registeredAttesters);
  const stakedAttesters = await staking.getStakedAttestersForAcceptingEpoch(mcrAddress);
  console.log("Staked Attesters:", stakedAttesters);
}

// This function tests the MCR functionality
async function testMCR(
  mcr: IMCR,
  mcrContract: MCR,
  moveToken: IERC20, 
  staker: ethers.Wallet,
  deployer: ethers.Wallet
) {
  console.log("\n=== Testing MCR Functionality ===");

  // Grant COMMITMENT_ADMIN role to deployer
  const COMMITMENT_ADMIN = await mcrContract.COMMITMENT_ADMIN();
  const txGrantAdminRole = await mcrContract.grantRole(COMMITMENT_ADMIN, deployer.address);
  await txGrantAdminRole.wait();
  console.log("Commitment admin role granted to deployer");

  const TRUSTED_ATTESTER = await mcrContract.TRUSTED_ATTESTER();
  const txGrantRole = await mcrContract.grantRole(TRUSTED_ATTESTER, staker.address);
  await txGrantRole.wait();
  const hasRole = await mcrContract.hasRole(TRUSTED_ATTESTER, staker.address);
  if (!hasRole) {
    throw new Error("TRUSTED_ATTESTER role not granted");
  }
  console.log("Attester role granted to staker: ", hasRole);
  const stakerBalance = await moveToken.balanceOf(staker.address);
  console.log("Staker balance:", ethers.formatEther(stakerBalance));

  const epochDuration = await mcr.connect(staker).getEpochDuration();
  console.log("Epoch duration (seconds):", Number(epochDuration));
  const acceptingEpoch = await mcr.connect(staker).getAcceptingEpoch();
  console.log("Accepting epoch:", acceptingEpoch);
  // mcr contract should set the accepting epoch to the present epoch
  const presentEpoch = await mcr.connect(deployer).getPresentEpoch();
  console.log("Present epoch:", presentEpoch);
  const txSetAcceptingEpoch = await mcr.connect(deployer).setAcceptingEpoch(presentEpoch - 2n);
  await txSetAcceptingEpoch.wait();
  const acceptingEpoch2 = await mcr.connect(staker).getAcceptingEpoch();
  console.log("Accepting epoch after setting to present epoch:", acceptingEpoch2);
  if (presentEpoch > acceptingEpoch) {
    console.log("Present epoch is greater than accepting epoch, so rollover should update the accepting epoch.");
  }

  // rollover the epoch
  const txRollover = await mcr.connect(staker).postconfirmSuperBlocksAndRollover();
  await txRollover.wait();
  const acceptingEpoch3 = await mcr.connect(staker).getAcceptingEpoch();
  console.log("New accepting epoch after rollover:", acceptingEpoch3);

  // submit a dummy commitment
  const initialPostconfirmedSuperblockHeight = await mcr.getLastPostconfirmedSuperBlockHeight();
  console.log("Initial postconfirmed superblock height:", initialPostconfirmedSuperblockHeight);
  const dummyCommitment = {
    height: initialPostconfirmedSuperblockHeight + 1n,
    commitment: ethers.randomBytes(32),
    blockId: ethers.randomBytes(32)
  } as const;  
  const txCommit = await mcr.connect(staker).submitSuperBlockCommitment(dummyCommitment);
  await txCommit.wait();
  const lastPostconfirmedSuperblockHeight = await mcr.getLastPostconfirmedSuperBlockHeight();
  console.log("Last postconfirmed superblock height:", lastPostconfirmedSuperblockHeight);
  const newAcceptingEpoch = await mcr.getAcceptingEpoch();
  console.log("Accepting epoch:", newAcceptingEpoch);

  // postconfirm
  const txPostconfirm = await mcr.connect(staker).postconfirmSuperBlocksAndRollover();
  await txPostconfirm.wait();
  const newPostconfirmedSuperblockHeight = await mcr.getLastPostconfirmedSuperBlockHeight();
  if (newPostconfirmedSuperblockHeight !== dummyCommitment.height) {
    throw new Error("Last postconfirmed superblock height mismatch");
  }
  console.log("Last postconfirmed superblock height:", newPostconfirmedSuperblockHeight);
  const newAcceptingEpoch2 = await mcr.getAcceptingEpoch();
  console.log("Accepting epoch:", newAcceptingEpoch2);
  const newStakerBalance = await moveToken.balanceOf(staker.address);
  console.log("Staker balance:", ethers.formatEther(newStakerBalance));

  // wait for the epoch duration time to ensure at least one rollover
  await new Promise(resolve => setTimeout(resolve, Number(epochDuration)));
  const txRollOverEpoch = await mcr.connect(staker).postconfirmSuperBlocksAndRollover();
  await txRollOverEpoch.wait();
  const newStakerBalance3 = await moveToken.balanceOf(staker.address);
  console.log("Staker balance (latest point where it should be rewarded):", ethers.formatEther(newStakerBalance3));
}


// This function tests if the staker can unstake and if the stake is removed from the attester list
async function testUnstaking(
  staking: IMovementStaking & AccessControl, 
  moveToken: IERC20, 
  mcrAddress: string, 
  staker: ethers.Wallet, 
  mcr: IMCR
) {
  console.log("\n=== Testing Unstaking Functionality ===");

  // initial list of active attesters
  const initialActiveAttesters = await staking.getStakedAttestersForAcceptingEpoch(mcrAddress);
  console.log("Initial active attesters:", initialActiveAttesters);

  // get stake amount
  const stakeAmount = await staking.getAttesterStakeForAcceptingEpoch(mcrAddress, staker.address);
  console.log("Stake amount:", ethers.formatEther(stakeAmount));

  // unstake
  const txUnstake = await staking.connect(staker).unstake(mcrAddress, moveToken.target, stakeAmount);
  await txUnstake.wait();
  if (await staking.getAttesterStakeForAcceptingEpoch(mcrAddress, staker.address) !== 0n) {
    throw new Error("Unstaking failed");
  }
  console.log("Unstaking successful");

  // warp time into next epoch
  const epochDuration = await mcr.getEpochDuration();
  await new Promise(resolve => setTimeout(resolve, Number(epochDuration)));
  const txRollOverEpoch2 = await mcr.connect(staker).postconfirmSuperBlocksAndRollover();
  await txRollOverEpoch2.wait();
  if (await mcr.getAcceptingEpoch() !== 3n) {
    throw new Error("Epoch rollover failed");
  }
  const stakerBalance4 = await moveToken.balanceOf(staker.address);
  console.log("Staker balance:", ethers.formatEther(stakerBalance4));

  // check that the staker is removed from the active attesters list
  const activeAttesters = await staking.getStakedAttestersForAcceptingEpoch(mcrAddress);
  console.log("Active attesters:", activeAttesters);
  if (activeAttesters.includes(staker.address)) {
    throw new Error("Staker is still in the active attesters list");
  }
}



main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
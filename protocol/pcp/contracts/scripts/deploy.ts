import { ethers } from "hardhat";
import * as fs from 'fs';
import * as path from 'path';

async function main() {
  try {
    console.log("Starting PCP Dev deployment...");
    const [deployer] = await ethers.getSigners();
    console.log("Deploying contracts with account:", deployer.address);

    const deployments = {
      ProxyAdmin: "",
      MoveToken: { implementation: "", proxy: "" },
      MovementStaking: { implementation: "", proxy: "" },
      PCP: { implementation: "", proxy: "" }
    };

    // Deploy ProxyAdmin
    const ProxyAdmin = await ethers.getContractFactory("ProxyAdmin");
    const proxyAdmin = await ProxyAdmin.deploy(deployer.address);
    await proxyAdmin.waitForDeployment();
    console.log("Deployed ProxyAdmin at:", await proxyAdmin.getAddress());
    deployments.ProxyAdmin = await proxyAdmin.getAddress();

    // Deploy MintableToken implementation
    console.log("Deploying MintableToken implementation...");
    const MoveToken = await ethers.getContractFactory("MintableToken");
    const moveTokenImpl = await MoveToken.deploy();
    await moveTokenImpl.waitForDeployment();
    console.log("Deployed MintableToken implementation at:", await moveTokenImpl.getAddress());
    deployments.MoveToken.implementation = await moveTokenImpl.getAddress();

    // Deploy other implementations
    const MovementStaking = await ethers.getContractFactory("MovementStaking");
    const PCP = await ethers.getContractFactory("PCP");
    const TransparentProxy = await ethers.getContractFactory("TransparentUpgradeableProxy");

    const stakingImpl = await MovementStaking.deploy();
    await stakingImpl.waitForDeployment();
    console.log("Deployed MovementStaking implementation at:", await stakingImpl.getAddress());
    deployments.MovementStaking.implementation = await stakingImpl.getAddress();

    const pcpImpl = await PCP.deploy();
    await pcpImpl.waitForDeployment();
    console.log("Deployed PCP implementation at:", await pcpImpl.getAddress());
    deployments.PCP.implementation = await pcpImpl.getAddress();

    // Prepare MOVE token proxy deployment
    console.log("Preparing MOVE token proxy deployment...");
    const moveTokenData = MoveToken.interface.encodeFunctionData("initialize", [
      "Move Token",
      "MOVE"
    ]);

    console.log("Deploying MOVE token proxy...");
    const moveTokenProxy = await TransparentProxy.deploy(
      await moveTokenImpl.getAddress(),
      await proxyAdmin.getAddress(),
      moveTokenData
    );
    await moveTokenProxy.waitForDeployment();
    console.log("Deployed MOVE token proxy at:", await moveTokenProxy.getAddress());
    deployments.MoveToken.proxy = await moveTokenProxy.getAddress();

    // Deploy staking proxy
    console.log("Deploying staking proxy...");
    // const stakingData = MovementStaking.interface.encodeFunctionData("initialize", [
    //   await moveTokenProxy.getAddress()
    // ]);
    const stakingData = MovementStaking.interface.encodeFunctionData(
      "initialize(address)",
      [await moveTokenProxy.getAddress()]
    );
    

    const stakingProxy = await TransparentProxy.deploy(
      await stakingImpl.getAddress(),
      await proxyAdmin.getAddress(),
      stakingData
    );
    await stakingProxy.waitForDeployment();
    console.log("Deployed staking proxy at:", await stakingProxy.getAddress());
    deployments.MovementStaking.proxy = await stakingProxy.getAddress();

    // Deploy PCP proxy
    console.log("Deploying PCP proxy...");
    const custodians = [await moveTokenProxy.getAddress()];
    console.log("Custodian Address:", custodians[0]); // Debugging output

    const pcpData = PCP.interface.encodeFunctionData(
      "initialize(address,uint256,uint256,uint256,address[],uint256,address)",
      [
      await stakingProxy.getAddress(),  // stakingContract
      0,                                // lastPostconfirmedSuperBlockHeight
      5,                                // leadingSuperBlockTolerance
      10,                               // epochDuration (10 seconds)
      custodians,                       // custodians array
      5,                                // postconfirmerDuration (5 seconds)
      await moveTokenProxy.getAddress() // moveTokenAddress
      ]
    );

    const pcpProxy = await TransparentProxy.deploy(
      await pcpImpl.getAddress(),
      await proxyAdmin.getAddress(),
      pcpData
    );
    await pcpProxy.waitForDeployment();
    console.log("Deployed PCP proxy at:", await pcpProxy.getAddress());
    deployments.PCP.proxy = await pcpProxy.getAddress();

    // Set up roles and initial token distribution
    console.log("Setting up roles and minting initial tokens...");
    const moveToken = MoveToken.attach(await moveTokenProxy.getAddress());
    const pcp = PCP.attach(await pcpProxy.getAddress());

    // Mint initial tokens and set up roles
    await moveToken.mint(deployer.address, ethers.parseEther("100000"));
    await moveToken.grantMinterRole(deployer.address);
    await moveToken.grantMinterRole(await stakingProxy.getAddress());
    await pcp.grantCommitmentAdmin(deployer.address);

    // Log deployment addresses and configuration
    console.log("\n=== Deployment Complete ===");
    console.log("MOVE Token Proxy:", await moveTokenProxy.getAddress());
    console.log("Staking Proxy:", await stakingProxy.getAddress());
    console.log("PCP Proxy:", await pcpProxy.getAddress());

    // Verify deployment
    const deployerBalance = await moveToken.balanceOf(deployer.address);
    console.log("\n=== Verification ===");
    console.log("Deployer MOVE Balance:", ethers.formatEther(deployerBalance));
    console.log("Epoch Duration:", await pcp.getEpochDuration());
    console.log("Postconfirmer Duration:", await pcp.getPostconfirmerDuration());

    // Save deployments
    const deploymentsPath = path.join(__dirname, '../deployments/local.json');
    fs.writeFileSync(deploymentsPath, JSON.stringify(deployments, null, 2));
  } catch (error) {
    console.error("Detailed error:", error);
    if (error instanceof Error) {
      console.error("Error message:", error.message);
      console.error("Error stack:", error.stack);
    }
    throw error;
  }
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });

import { task } from "hardhat/config";
import "@nomicfoundation/hardhat-toolbox";
import "@nomicfoundation/hardhat-ethers";
import "./scripts/tasks/utils";

export default {
  solidity: {
    version: "0.8.22",
    settings: {
      optimizer: {
        enabled: true,
        runs: 200
      }
    }
  },
  paths: {
    sources: "./src",          // Point to src directory
    tests: "./test",           // Point to test directory
    cache: "./cache_hardhat",  // Use separate cache for Hardhat
    artifacts: "./artifacts"   // Where compiled contracts will go
  },
  networks: {
    localnet: {
      url: "http://127.0.0.1:52603",  // Updated Kurtosis RPC URL
      accounts: [
        "bcdf20249abf0ed6d944c0288fad489e33f66b3960d9e6229c1cd214ed3bbe31", // deployer
        "39725efee3fb28614de3bacaffe4cc4bd8c436257e2c8bb887c4b5c4be45e76d"  // funded account
      ]
    },
  },
};

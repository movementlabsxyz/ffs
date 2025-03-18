import { task } from "hardhat/config";

// Task to check balance of an address
// Example: npx hardhat balance --network localnet --address 0x8943545177806ED17B9F23F0a21ee5948eCaa776
task("balance", "Gets the balance of an address")
  .addParam("address", "The account address")
  .setAction(async (taskArgs, hre) => {
    const balance = await hre.ethers.provider.getBalance(taskArgs.address);
    console.log(`Balance: ${hre.ethers.formatEther(balance)} ETH`);
  });

// Task to send ETH and show sender & recipient balances before and after
// Example: npx hardhat sendTx --network localnet --to 0x1234567890abcdef1234567890abcdef12345678 --amount 0.1
task("sendTx", "Sends ETH and shows sender & recipient balances before and after")
  .addParam("to", "The recipient address")
  .addParam("amount", "The amount in ETH")
  .setAction(async (taskArgs, hre) => {
    const [signer] = await hre.ethers.getSigners();

    // Fetch initial balances
    const senderBalanceBefore = await hre.ethers.provider.getBalance(signer.address);
    const recipientBalanceBefore = await hre.ethers.provider.getBalance(taskArgs.to);

    console.log(`Sender (${signer.address}) balance before: ${hre.ethers.formatEther(senderBalanceBefore)} ETH`);
    console.log(`Recipient (${taskArgs.to}) balance before: ${hre.ethers.formatEther(recipientBalanceBefore)} ETH`);

    console.log(`Sending ${taskArgs.amount} ETH from ${signer.address} to ${taskArgs.to}...`);

    const tx = await signer.sendTransaction({
      to: taskArgs.to,
      value: hre.ethers.parseEther(taskArgs.amount),
    });

    console.log("Transaction sent! Hash:", tx.hash);
    await tx.wait();
    console.log("Transaction confirmed.");

    // Fetch updated balances
    const senderBalanceAfter = await hre.ethers.provider.getBalance(signer.address);
    const recipientBalanceAfter = await hre.ethers.provider.getBalance(taskArgs.to);

    console.log(`Sender (${signer.address}) balance after: ${hre.ethers.formatEther(senderBalanceAfter)} ETH`);
    console.log(`Recipient (${taskArgs.to}) balance after: ${hre.ethers.formatEther(recipientBalanceAfter)} ETH`);
  }); 
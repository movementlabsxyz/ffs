use clap::Subcommand;
use pcp_dlu_eth_deployer_core::contracts::ContractWorkspace;
use std::format;

#[derive(Subcommand)]
#[clap(rename_all = "kebab-case")]
pub enum Deploy {
    /// Deploy to local Anvil network
    Anvil {
        /// Admin address for deployed contracts
        #[clap(long)]
        admin: String,
        
        /// RPC URL (defaults to http://localhost:8545)
        #[clap(long, default_value = "http://localhost:8545")]
        rpc_url: String,
        
        /// Private key for deployment
        #[clap(long)]
        private_key: String,
    },
}

impl Deploy {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        match self {
            Deploy::Anvil { admin, rpc_url, private_key } => {
                // Create temporary workspace
                let workspace = ContractWorkspace::try_temp_tip()?;
                workspace.prepare_directory()?;

                // Run deployment
                // Configuration fields:
                // - contractAdmin: Admin address that will have control over the contracts
                // - tokenName: Display name of the token
                // - tokenSymbol: Symbol used in wallets/exchanges
                // - initialTokenMint: Initial supply (100,000 tokens with 18 decimals)
                // - custodians: Array of addresses that can act as custodians
                // - initialCommitmentHeight: Starting block height for the protocol
                // - leadingCommitmentTolerance: Number of blocks a validator can be ahead
                // - epochDuration: Duration of each epoch in seconds
                // - rewardOption: 0=none, 1=deploy ARO, 2=use existing reward contract
                // - existingRewardContract: Used if rewardOption=2
                // - existingProxyAdmin: Admin contract for proxies
                // - existingMoveTokenProxy: MOVE token proxy
                // - existingStakingProxy: Staking contract proxy
                // - existingPcpProxy: PCP protocol proxy
                // - existingAroProxy: ARO reward proxy
                // - destroyMode: If true, nullifies all proxies (for cleanup)
                let config = format!(
                    r#"{{
                        "contractAdmin": "{}",
                        "tokenName": "Move Token",
                        "tokenSymbol": "MOVE",
                        "initialTokenMint": "100000000000000000000000",
                        "custodians": [],
                        "initialCommitmentHeight": "0",
                        "leadingCommitmentTolerance": "10",
                        "epochDuration": "4",
                        "rewardOption": "0",
                        "existingRewardContract": "0x0000000000000000000000000000000000000000",
                        "existingProxyAdmin": "0x0000000000000000000000000000000000000000",
                        "existingMoveTokenProxy": "0x0000000000000000000000000000000000000000",
                        "existingStakingProxy": "0x0000000000000000000000000000000000000000",
                        "existingPcpProxy": "0x0000000000000000000000000000000000000000",
                        "existingAroProxy": "0x0000000000000000000000000000000000000000",
                        "destroyMode": false
                    }}"#,
                    admin
                );

                // Set config as environment variable
                std::env::set_var("DEPLOY_CONFIG", &config);

                workspace.run_command(
                    "forge",
                    [
                        "script",
                        "script/DeployPCPDev.s.sol",
                        "--tc", "DeployPCPDev",
                        "--rpc-url", rpc_url,
                        "--broadcast",
                        "--private-key", private_key,
                    ],
                ).await?;

                Ok(())
            }
        }
    }
}
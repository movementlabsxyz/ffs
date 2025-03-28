use clap::Subcommand;
use lifecycle::{LifecycleFrontend, LifecycleSubcommand};
use mcr_protocol_deployer_eth_core::Lifecycle;

#[derive(LifecycleSubcommand, Subcommand)]
#[lifecycle_subcommand(Lifecycle)]
#[clap(rename_all = "kebab-case")]
pub enum Eth {}

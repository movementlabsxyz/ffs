use clap::Subcommand;
use lifecycle::{LifecycleFrontend, LifecycleSubcommand};
use mcr_protocol_deployer_eth_core::Lifecycle;

#[derive(LifecycleSubcommand, Subcommand)]
#[lifecycle(Lifecycle)]
#[lifecycle_apply_is_subcommand]
#[clap(rename_all = "kebab-case")]
pub enum Eth {}

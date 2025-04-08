use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug, Serialize, Deserialize, Clone)]
#[clap(help_expected = true)]
pub struct Config {}

impl Config {}

use clap::Parser;

#[derive(Parser)]
#[clap(help_expected = true)]
pub struct PostCommitment {
    #[clap(flatten)]
    pub args: PostCommitmentArgs,
}

#[derive(Parser)]
pub struct PostCommitmentArgs {
    /// Hex-encoded commitment
    #[clap(long, conflicts_with = "preimage_string", required_unless_present = "preimage_string")]
    // for option `--commitment-hex "0x1234567890abcdef"`
    commitment_hex: Option<String>,

    /// String to be hashed into a commitment
    #[clap(long, conflicts_with = "commitment_hex", required_unless_present = "commitment_hex")]
    // for option `--preimage-string "test message"`
    preimage_string: Option<String>,
}

impl PostCommitment {
    pub async fn execute(&self) -> Result<(), anyhow::Error> {
        if let Some(hex) = &self.args.commitment_hex {
            println!("Would post commitment from hex: {}", hex);
        } else if let Some(preimage) = &self.args.preimage_string {
            println!("Would hash and post commitment from preimage: {}", preimage);
        }
        Ok(())
    }
} 
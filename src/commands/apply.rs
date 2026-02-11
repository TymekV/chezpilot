use clap::Parser;
use miette::Result;

use crate::{GlobalArgs, config::read_config};

#[derive(Parser, Debug, Clone)]
pub struct ApplyArgs {
    /// Perform a dry run without actually modifying anything on your system
    #[arg(long)]
    pub dry_run: bool,
}

pub async fn apply(global_args: GlobalArgs, args: ApplyArgs) -> Result<()> {
    let config = read_config(&global_args.file).await?;
    Ok(())
}

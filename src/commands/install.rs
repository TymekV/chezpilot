use clap::Parser;
use miette::Result;

use crate::GlobalArgs;

#[derive(Parser, Debug, Clone)]
pub struct InstallArgs {}

pub fn install(global_args: GlobalArgs, args: InstallArgs) -> Result<()> {
    Ok(())
}

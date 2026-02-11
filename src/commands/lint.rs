use miette::Result;

use crate::{GlobalArgs, config::read_config};

pub async fn lint(global_args: GlobalArgs) -> Result<()> {
    let config = read_config(&global_args.file).await?;
    Ok(())
}

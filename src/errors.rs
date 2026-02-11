use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Debug, Diagnostic)]
#[error("unable to read config")]
#[diagnostic(code(config::read_fail), help("Ensure that the file {path:?} exists."))]
pub struct UnableToReadConfig {
    pub path: PathBuf,
}

#[derive(Error, Debug, Diagnostic)]
#[error("invalid config: {0}")]
#[diagnostic(
    code(config::deserialization_failed),
    help("Ensure that the syntax is correct and all of the required fields are provided.")
)]
pub struct InvalidConfig(pub serde_yaml::Error);

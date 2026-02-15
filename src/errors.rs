use std::path::PathBuf;

use miette::Diagnostic;
use strum::Display;
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

#[derive(Error, Debug, Diagnostic)]
#[error("{manager}: platform not supported")]
#[diagnostic(
    code(package_manager::platform_not_supported),
    help(
        r#"This package manager does not support your platform.
        Update conditions in your configuration file to avoid missmatches like this.
        Use `lint` command to catch most of these errors early."#
    )
)]
pub struct UnsupportedPlatform {
    pub manager: &'static str,
}

#[derive(Error, Debug, Diagnostic)]
#[error("no AUR helper found")]
#[diagnostic(
    code(package::aur::helper_not_detected),
    help(
        "Install a popular AUR helper (e.g. paru or yay) or set the AUR_HELPER environment variable to point to your preferred one."
    )
)]
pub struct AurHelperNotDetected;

#[derive(Debug, Display)]
pub enum AurHelperPinReason {
    #[strum(serialize = "a config override")]
    ConfigOverride,
    #[strum(serialize = "an env override (AUR_HELPER)")]
    EnvOverride,
    #[strum(serialize = "other means")]
    Other,
}

#[derive(Error, Debug, Diagnostic)]
#[error("{helper}: command not found")]
#[diagnostic(
    code(package::aur::requested_helper_not_found),
    help("This heleper was specified by {reason}, but could not be found on your system.")
)]
pub struct RequestedAurHelperNotFound {
    pub helper: String,
    pub reason: AurHelperPinReason,
}

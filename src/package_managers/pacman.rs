#[cfg(target_os = "linux")]
use alpm::Alpm;
use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};
use schemars::JsonSchema;
use serde::Deserialize;

use crate::{config::OsName, package_managers::PackageManager};

pub struct Pacman {}

impl Pacman {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct PacmanPackage {
    pub name: String,
    pub version: String,
    pub aur: bool,
}

#[derive(Deserialize, Debug, Clone, JsonSchema)]
pub struct PacmanOptions {
    /// Packages installed using `pacman`
    pub repo: Option<Vec<String>>,

    /// Additional arguments passed to `pacman`
    pub pacman_args: Option<Vec<String>>,

    /// Packages installed using user's preferred AUR helper by default.
    pub aur: Option<Vec<String>>,

    /// Args passed to user's AUR helper.
    pub aur_helper_args: Option<Vec<String>>,

    /// Force the usage of a specified AUR helper.
    pub force_aur_helper: Option<String>,
}

#[async_trait]
impl PackageManager for Pacman {
    const NAME: &'static str = "pacman";
    const SUPPORTED_OS: &'static [OsName] = &[OsName::Linux];

    type Options = PacmanOptions;

    #[cfg(target_os = "linux")]
    async fn get_installed(&self) -> Result<HashMap<String, PackageMetadata>> {
        let alpm = Alpm::new("/", "/var/lib/pacman").into_diagnostic()?;
        let db = alpm.localdb();

        let packages = db
            .pkgs()
            .iter()
            .map(|package| {
                (
                    package.name().to_string(),
                    PackageMetadata {
                        version: Some(package.version().to_string()),
                    },
                )
            })
            .collect();

        Ok(packages)
    }

    #[cfg(target_os = "linux")]
    async fn install(&self, options: Self::Options) -> Result<()> {
        use miette::Context;
        use tokio::process::Command;

        if packages.is_empty() {
            return Ok(());
        }

        let status = Command::new("sudo")
            .arg("pacman")
            .arg("-S")
            .arg("--needed") // Skip packages that are already up to date
            .args(&packages)
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .status()
            .await
            .into_diagnostic()
            .wrap_err("Failed to execute pacman")?;

        if !status.success() {
            miette::bail!("pacman exited with status: {}", status);
        }

        Ok(())
    }
}

use std::collections::HashMap;

use async_trait::async_trait;
use miette::{IntoDiagnostic, Result};

#[cfg(target_os = "linux")]
use alpm::Alpm;

use crate::{
    config::OsName,
    package_managers::{PackageManager, PackageMetadata},
};

pub struct Pacman {}

impl Pacman {
    pub fn new() -> Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl PackageManager for Pacman {
    const NAME: &'static str = "pacman";
    const SUPPORTED_OS: &'static [OsName] = &[OsName::Linux];

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
    async fn install(&self, packages: Vec<String>) -> Result<()> {
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

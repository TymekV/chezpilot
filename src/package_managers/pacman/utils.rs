use owo_colors::OwoColorize;
use thiserror::Error;
use tokio::process::Command;
use tracing::{info, instrument};

use crate::errors::{AurHelperNotDetected, AurHelperPinReason, RequestedAurHelperNotFound};

static AUR_HELPERS: &[&str] = &["paru", "yay", "pikaur", "trizen"];

#[derive(Error, Debug)]
#[error(transparent)]
pub enum AurDetectionError {
    AurHelperNotDetected(#[from] AurHelperNotDetected),
    RequestedAurHelperNotFound(#[from] RequestedAurHelperNotFound),
}

async fn check_helper(
    helper: &str,
    reason: AurHelperPinReason,
) -> Result<(), RequestedAurHelperNotFound> {
    let result = Command::new(helper).arg("--version").status().await;

    if let Ok(status) = result
        && status.success()
    {
        return Ok(());
    }

    Err(RequestedAurHelperNotFound {
        helper: helper.to_string(),
        reason,
    })
}

#[instrument(skip(force_aur_helper))]
pub async fn select_aur_helper(
    force_aur_helper: Option<String>,
) -> Result<String, AurDetectionError> {
    // First, we check for config overides
    if let Some(helper) = force_aur_helper {
        check_helper(&helper, AurHelperPinReason::ConfigOverride).await?;
        return Ok(helper);
    }

    // Then we check for env variable overrides
    if let Ok(env_override) = std::env::var("AUR_HELPER") {
        info!(
            "Using {env_override} AUR helper {}",
            "(env override)".bold().dimmed()
        );
        check_helper(&env_override, AurHelperPinReason::ConfigOverride).await?;
        return Ok(env_override);
    }

    // Then we try the most popular AUR helpers
    for helper in AUR_HELPERS {
        if check_helper(helper, AurHelperPinReason::Other)
            .await
            .is_ok()
        {
            info!(
                "Using {helper} AUR helper {}",
                "(autodetect)".bold().dimmed()
            );
            return Ok(helper.to_string());
        }
    }

    Err(AurDetectionError::AurHelperNotDetected(
        AurHelperNotDetected,
    ))
}

mod commands;
mod config;
mod formatter;
mod package_managers;
mod report_handler;
mod utils;

use std::{ffi::OsStr, path::PathBuf, process};

use clap::{Parser, Subcommand};
use miette::Result;
use tracing::level_filters::LevelFilter;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::{commands::install, report_handler::ErrorReportHandler};

/// DotGet
///
/// A tool for managing and installing dependencies declared in your dotfiles configuration.
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[command(flatten)]
    args: GlobalArgs,
}

#[derive(Parser, Debug, Clone)]
pub struct GlobalArgs {
    #[arg(short = 'f', long, default_value_os = OsStr::new("dotget.yaml"))]
    file: PathBuf,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Install {
        #[command(flatten)]
        args: install::InstallArgs,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    miette::set_hook(Box::new(|_| Box::new(ErrorReportHandler::new())))?;
    let indicatif_layer = IndicatifLayer::new();

    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(indicatif_layer.get_stderr_writer())
                .event_format(formatter::EventFormatter)
                .with_filter(LevelFilter::INFO),
        )
        .with(indicatif_layer)
        .init();

    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Install { args } => install::install(cli.args, args),
    };

    if let Err(e) = result {
        eprintln!("{e:?}");
        process::exit(1);
    }

    Ok(())
}

mod binary;
mod ui;
mod analysis;
mod commands;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "binsight")]
#[command(about = "A binary size analyzer for developers", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze a binary file
    Analyze {
        /// Path to the binary
        path: PathBuf,
    },
    /// Run detailed binary health diagnosis
    Diagnose {
        /// Path to the binary
        path: PathBuf,
    },
    /// Show largest contributors
    Top {
        /// Path to the binary
        path: PathBuf,
        /// Grouping depth (1 for crate, 2+ for modules)
        #[arg(short, long, default_value_t = 1)]
        depth: usize,
    },
    /// Compare two binary files
    Diff {
        /// Path to the old binary
        old_path: PathBuf,
        /// Path to the new binary
        new_path: PathBuf,
    },
    /// Interactive TUI explorer
    Explore {
        /// Path to the binary
        path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    #[cfg(windows)]
    if !colored::control::SHOULD_COLOR.should_colorize() {
        // Simple heuristic: if color is disabled, maybe Unicode is also risky
        // in older CMD.exe environments.
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path } => commands::analyze::execute(path),
        Commands::Diagnose { path } => commands::diagnose::execute(path),
        Commands::Top { path, depth } => commands::top::execute(path, depth),
        Commands::Diff { old_path, new_path } => commands::diff::execute(old_path, new_path),
        Commands::Explore { path } => commands::explore::execute(path),
    }
}

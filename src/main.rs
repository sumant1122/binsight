mod binary;
mod ui;
mod analysis;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "binsize")]
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
    /// Show largest contributors
    Top {
        /// Path to the binary
        path: PathBuf,
    },
    /// Compare two binary files
    Diff {
        /// Path to the old binary
        old_path: PathBuf,
        /// Path to the new binary
        new_path: PathBuf,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path } => {
            let info = binary::load_and_analyze(&path)?;
            ui::display_analysis(&info);
            let suggestions = analysis::get_suggestions(&info);
            ui::display_suggestions(&suggestions);
        }
        Commands::Top { path } => {
            let info = binary::load_and_analyze(&path)?;
            ui::display_top_contributors(&info);
        }
        Commands::Diff { old_path, new_path } => {
            let old_info = binary::load_and_analyze(&old_path)?;
            let new_info = binary::load_and_analyze(&new_path)?;
            let diff = analysis::compare(&old_info, &new_info);
            ui::display_diff(&diff);
        }
    }

    Ok(())
}

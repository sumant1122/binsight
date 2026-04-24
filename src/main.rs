mod binary;
mod ui;
mod analysis;

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
    let cli = Cli::parse();

    match cli.command {
        Commands::Analyze { path } => {
            let info = binary::load_and_analyze(&path)?;
            ui::display_analysis(&info);
            let diags = analysis::run_diagnostics(&info);
            ui::display_diagnostics(&diags);
        }
        Commands::Diagnose { path } => {
            let info = binary::load_and_analyze(&path)?;
            let diags = analysis::run_diagnostics(&info);
            ui::display_diagnostics(&diags);
        }
        Commands::Top { path, depth } => {
            let info = binary::load_and_analyze(&path)?;
            ui::display_top_contributors(&info, depth);
        }
        Commands::Diff { old_path, new_path } => {
            let old_info = binary::load_and_analyze(&old_path)?;
            let new_info = binary::load_and_analyze(&new_path)?;
            let diff = analysis::compare(&old_info, &new_info);
            ui::display_diff(&diff);
        }
        Commands::Explore { path } => {
            let info = binary::load_and_analyze(&path)?;
            ui::tui::run_tui(info)?;
        }
    }

    Ok(())
}

use anyhow::Result;
use clap::{Parser, Subcommand};

mod git;
mod conflict_checker;
mod ui;

#[derive(Parser)]
#[command(name = "preflight")]
#[command(about = "Pre-push merge conflict predictor", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check for merge conflicts with base branch
    Check {
        /// Base branch to check against (defaults to main/master)
        #[arg(short, long)]
        base: Option<String>,

        /// Show detailed statistics (ahead/behind, file changes)
        #[arg(short, long)]
        stats: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { base, stats } => {
            conflict_checker::check_conflicts(base, stats)?;
        }
    }

    Ok(())
}

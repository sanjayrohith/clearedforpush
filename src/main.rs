use anyhow::Result;
use clap::{Parser, Subcommand};

mod conflict_checker;
mod git;
mod github;
mod hooks;
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

        /// Skip checking against open pull requests
        #[arg(long)]
        skip_prs: bool,
    },

    /// Install preflight as a pre-push git hook
    InstallHook {
        /// Overwrite or chain with existing pre-push hook
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall the preflight pre-push git hook
    UninstallHook,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check { base, stats, skip_prs } => {
            conflict_checker::check_conflicts(base, stats, skip_prs)?;
        }
        Commands::InstallHook { force } => {
            install_hook_cmd(force)?;
        }
        Commands::UninstallHook => {
            uninstall_hook_cmd()?;
        }
    }

    Ok(())
}

fn install_hook_cmd(force: bool) -> Result<()> {
    ui::print_header();
    println!();

    use hooks::HookInstallResult;
    match hooks::install_hook(force)? {
        HookInstallResult::Installed { path } => {
            ui::print_hook_installed(&path);
        }
        HookInstallResult::Chained => {
            ui::print_hook_chained();
        }
        HookInstallResult::AlreadyInstalled => {
            ui::print_hook_already_installed();
        }
        HookInstallResult::ExistingHookFound { path } => {
            ui::print_hook_exists(&path);
            std::process::exit(1);
        }
    }

    Ok(())
}

fn uninstall_hook_cmd() -> Result<()> {
    ui::print_header();
    println!();

    use hooks::HookUninstallResult;
    match hooks::uninstall_hook()? {
        HookUninstallResult::Removed => {
            ui::print_hook_removed();
        }
        HookUninstallResult::SectionRemoved => {
            ui::print_hook_section_removed();
        }
        HookUninstallResult::NoHookFound => {
            ui::print_hook_not_found();
        }
        HookUninstallResult::NotInstalled => {
            ui::print_hook_not_installed();
        }
    }

    Ok(())
}

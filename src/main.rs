use anyhow::Result;
use clap::{Parser, Subcommand};

mod config;
mod conflict_checker;
mod git;
mod github;
mod hooks;
mod output;
mod ui;

#[derive(Parser)]
#[command(name = "clearedforpush")]
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

        /// Show conflicting diff hunks
        #[arg(long)]
        diff: bool,

        /// Output format: text, json, compact
        #[arg(long)]
        format: Option<String>,
    },

    /// Install clearedforpush as a pre-push git hook
    InstallHook {
        /// Overwrite or chain with existing pre-push hook
        #[arg(short, long)]
        force: bool,
    },

    /// Uninstall the clearedforpush pre-push git hook
    UninstallHook,

    /// Initialize a .clearedforpush.toml config file in the current repo
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Check {
            base,
            stats,
            skip_prs,
            diff,
            format,
        } => {
            // Load config file
            let cfg = config::Config::load();

            // CLI flags override config. Config provides defaults.
            let effective_base = base.or(cfg.base_branch.clone());
            let effective_stats = stats || cfg.stats;
            let effective_diff = diff || cfg.diff;
            let effective_skip_prs = skip_prs || !cfg.check_prs;

            // Format: CLI flag > config > default "text"
            let format_str = format
                .or(cfg.format.clone())
                .unwrap_or_else(|| "text".to_string());

            let fmt = output::OutputFormat::from_str(&format_str).unwrap_or_else(|| {
                eprintln!(
                    "Unknown format '{}'. Using 'text'. Options: text, json, compact",
                    format_str
                );
                output::OutputFormat::Text
            });

            conflict_checker::check_conflicts(
                effective_base,
                effective_stats,
                effective_skip_prs,
                effective_diff,
                fmt,
                &cfg,
            )?;
        }
        Commands::InstallHook { force } => {
            install_hook_cmd(force)?;
        }
        Commands::UninstallHook => {
            uninstall_hook_cmd()?;
        }
        Commands::Init => {
            init_config()?;
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

fn init_config() -> Result<()> {
    ui::print_header();
    println!();

    let config_path = ".clearedforpush.toml";
    if std::path::Path::new(config_path).exists() {
        println!("  {} {} already exists.", "ℹ".yellow(), config_path);
        return Ok(());
    }

    let template = r#"# Cleared for Push configuration
# See: https://github.com/yourusername/clearedforpush

# Base branch to check against (auto-detected if not set)
# base = "main"

# Whether to check open PRs for conflicts (default: true)
# check_prs = true

# Default output format: "text", "json", or "compact"
# format = "text"

# Show statistics by default
# stats = false

# Show conflict diff hunks by default
# diff = false

# Paths to ignore when reporting conflicts
# ignore = ["*.lock", "docs/**", "*.generated.*"]

# [github]
# token = "ghp_..."  # Alternative to GITHUB_TOKEN env var
"#;

    std::fs::write(config_path, template)?;

    use colored::Colorize;
    println!("  {} Created {}", "✓".green(), config_path.cyan());
    println!(
        "  {} Edit the file to configure clearedforpush for this repo.",
        "→".dimmed()
    );
    println!();

    Ok(())
}

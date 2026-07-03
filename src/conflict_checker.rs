use anyhow::{Context, Result};
use colored::Colorize;
use crate::git;
use crate::ui;

pub fn check_conflicts(base_branch: Option<String>, show_stats: bool) -> Result<()> {
    // Print beautiful header
    ui::print_header();

    // Get current branch
    ui::print_loading("Detecting current branch");
    let current_branch = git::get_current_branch()
        .context("Failed to determine current branch")?;

    // Determine base branch
    let base = match base_branch {
        Some(b) => b,
        None => {
            ui::print_loading("Detecting base branch");
            git::detect_base_branch()
                .context("Failed to detect base branch")?
        }
    };

    // Print branch info
    ui::print_branch_info(&current_branch, &base);

    // Check if we're on the base branch
    if current_branch == base {
        ui::print_already_on_base(&base);
        std::process::exit(0);
    }

    // Fetch latest base branch
    ui::print_loading("Fetching latest from origin");
    git::fetch_remote_branch(&base)
        .context("Failed to fetch base branch")?;

    let base_ref = format!("origin/{}", base);

    // Get statistics if requested
    if show_stats {
        ui::print_loading("Gathering statistics");
        match git::get_branch_stats(&current_branch, &base_ref) {
            Ok(stats) => ui::print_stats(&stats),
            Err(_) => {
                // Non-fatal: continue without stats
                println!("  {} Could not gather statistics", "⚠️".yellow());
            }
        }
    }

    // Run merge-tree check
    ui::print_loading("Simulating merge");
    let result = git::check_merge_tree(&current_branch, &base_ref)
        .context("Failed to check merge tree")?;

    // Report results
    if result.has_conflicts {
        ui::print_conflicts(&result.conflicted_files);
        std::process::exit(1);
    } else {
        ui::print_success(&current_branch, &base_ref);
        std::process::exit(0);
    }
}

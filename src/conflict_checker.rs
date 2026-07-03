use anyhow::{Context, Result};
use colored::Colorize;
use crate::git;

pub fn check_conflicts(base_branch: Option<String>) -> Result<()> {
    // Get current branch
    let current_branch = git::get_current_branch()
        .context("Failed to determine current branch")?;

    println!("{} {}", "Current branch:".bold(), current_branch);

    // Determine base branch
    let base = match base_branch {
        Some(b) => b,
        None => git::detect_base_branch()
            .context("Failed to detect base branch")?,
    };

    println!("{} {}", "Base branch:".bold(), base);

    // Check if we're on the base branch
    if current_branch == base {
        println!("\n{}", "You are already on the base branch.".yellow());
        std::process::exit(0);
    }

    // Fetch latest base branch
    println!("\n{}", "Fetching latest base branch...".dimmed());
    git::fetch_remote_branch(&base)
        .context("Failed to fetch base branch")?;

    // Run merge-tree check
    println!("{}", "Checking for conflicts...".dimmed());
    let result = git::check_merge_tree(&current_branch, &format!("origin/{}", base))
        .context("Failed to check merge tree")?;

    // Report results
    println!();
    if result.has_conflicts {
        println!("{}", "❌ Conflicts detected!".red().bold());
        println!("\nConflicting files:");
        for file in &result.conflicted_files {
            println!("  {}", file.red());
        }
        println!("\n{}", "Resolve conflicts before pushing.".yellow());
        std::process::exit(1);
    } else {
        println!("{}", "✅ No conflicts detected!".green().bold());
        println!("{}", "Safe to push.".dimmed());
        std::process::exit(0);
    }
}

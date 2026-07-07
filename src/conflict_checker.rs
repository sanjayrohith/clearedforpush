use anyhow::{Context, Result};
use colored::Colorize;
use crate::git;
use crate::github;
use crate::ui;

pub fn check_conflicts(base_branch: Option<String>, show_stats: bool, skip_prs: bool) -> Result<()> {
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
                println!("  {} Could not gather statistics", "⚠️".yellow());
            }
        }
    }

    // Run merge-tree check against base
    ui::print_loading("Simulating merge");
    let result = git::check_merge_tree(&current_branch, &base_ref)
        .context("Failed to check merge tree")?;

    let base_has_conflicts = result.has_conflicts;

    // Report base branch results
    if result.has_conflicts {
        ui::print_conflicts(&result.conflicted_files);
    } else {
        ui::print_success(&current_branch, &base_ref);
    }

    // PR awareness check
    if !skip_prs {
        check_pr_conflicts(&current_branch, &base);
    }

    // Exit with appropriate code
    if base_has_conflicts {
        std::process::exit(1);
    } else {
        std::process::exit(0);
    }
}

/// Check for conflicts against open PRs targeting the same base
fn check_pr_conflicts(current_branch: &str, base_branch: &str) {
    // Determine PR source
    let prs = fetch_open_prs(base_branch);

    let prs = match prs {
        Ok(prs) => prs,
        Err(e) => {
            // PR checking is non-fatal — warn and continue
            ui::print_pr_warning(&format!("{}", e));
            return;
        }
    };

    if prs.is_empty() {
        ui::print_no_open_prs();
        return;
    }

    // Filter out our own branch
    let prs: Vec<_> = prs.into_iter()
        .filter(|pr| pr.head_branch != current_branch)
        .collect();

    if prs.is_empty() {
        ui::print_no_open_prs();
        return;
    }

    ui::print_pr_check_header(prs.len());

    let mut results: Vec<github::PrCheckResult> = Vec::new();

    for pr in prs {
        // Try to fetch the PR branch
        let fetch_result = git::fetch_remote_branch(&pr.head_branch);
        if fetch_result.is_err() {
            results.push(github::PrCheckResult {
                pr,
                has_conflicts: false,
                conflicted_files: vec!["(could not fetch branch)".to_string()],
            });
            continue;
        }

        let pr_ref = format!("origin/{}", pr.head_branch);

        // Check merge-tree between our branch and the PR branch
        match git::check_merge_tree(current_branch, &pr_ref) {
            Ok(merge_result) => {
                results.push(github::PrCheckResult {
                    pr,
                    has_conflicts: merge_result.has_conflicts,
                    conflicted_files: merge_result.conflicted_files,
                });
            }
            Err(_) => {
                results.push(github::PrCheckResult {
                    pr,
                    has_conflicts: false,
                    conflicted_files: vec!["(merge-tree failed)".to_string()],
                });
            }
        }
    }

    ui::print_pr_results(&results);
}

/// Fetch open PRs using the best available method
fn fetch_open_prs(base_branch: &str) -> Result<Vec<github::PullRequest>> {
    // Try gh CLI first
    if github::gh_cli_available() {
        return github::list_prs_gh_cli(base_branch);
    }

    // Fall back to direct API with token
    if let Some(token) = github::github_token() {
        let slug = github::get_repo_slug()
            .context("Could not determine GitHub repository")?;
        return github::list_prs_api(base_branch, &token, &slug);
    }

    // No method available
    anyhow::bail!(
        "PR check requires either 'gh' CLI (run 'gh auth login') or GITHUB_TOKEN env var"
    );
}

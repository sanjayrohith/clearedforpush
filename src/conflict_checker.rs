use crate::config::Config;
use crate::git;
use crate::github;
use crate::output::{CheckOutput, OutputFormat};
use crate::ui;
use anyhow::{Context, Result};
use colored::Colorize;

pub fn check_conflicts(
    base_branch: Option<String>,
    show_stats: bool,
    skip_prs: bool,
    show_diff: bool,
    format: OutputFormat,
    config: &Config,
) -> Result<()> {
    // For non-text formats, suppress all interactive output
    let is_text = format == OutputFormat::Text;

    if is_text {
        ui::print_header();
    }

    // Get current branch
    if is_text {
        ui::print_loading("Detecting current branch");
    }
    let current_branch = git::get_current_branch().context("Failed to determine current branch")?;

    // Determine base branch
    let base = match base_branch {
        Some(b) => b,
        None => {
            if is_text {
                ui::print_loading("Detecting base branch");
            }
            git::detect_base_branch().context("Failed to detect base branch")?
        }
    };

    if is_text {
        ui::print_branch_info(&current_branch, &base);
    }

    // Check if we're on the base branch
    if current_branch == base {
        if is_text {
            ui::print_already_on_base(&base);
        } else if format == OutputFormat::Compact {
            println!("OK: on base branch, no check needed");
        } else {
            // JSON
            println!("{{\"version\":1,\"current_branch\":\"{}\",\"base_branch\":\"{}\",\"has_conflicts\":false,\"exit_code\":0,\"note\":\"already on base branch\"}}", current_branch, base);
        }
        std::process::exit(0);
    }

    // Fetch latest base branch
    if is_text {
        ui::print_loading("Fetching latest from origin");
    }
    git::fetch_remote_branch(&base).context("Failed to fetch base branch")?;

    let base_ref = format!("origin/{}", base);

    // Get statistics (always gather for JSON, conditionally for text)
    let stats = if show_stats || format == OutputFormat::Json {
        if is_text {
            ui::print_loading("Gathering statistics");
        }
        match git::get_branch_stats(&current_branch, &base_ref) {
            Ok(stats) => Some(stats),
            Err(_) => {
                if is_text {
                    println!("  {} Could not gather statistics", "⚠️".yellow());
                }
                None
            }
        }
    } else {
        None
    };

    // Show stats in text mode
    if is_text {
        if let Some(ref s) = stats {
            if show_stats {
                ui::print_stats(s);
            }
        }
    }

    // Run merge-tree check against base
    if is_text {
        ui::print_loading("Simulating merge");
    }
    let mut result =
        git::check_merge_tree(&current_branch, &base_ref).context("Failed to check merge tree")?;

    // Apply ignore patterns from config
    if !config.ignore_patterns.is_empty() && result.has_conflicts {
        result.conflicted_files = config.filter_conflicts(&result.conflicted_files);
        if result.conflicted_files.is_empty() {
            result.has_conflicts = false;
        }
    }

    // Get conflict diffs if requested and conflicts exist
    let conflict_diffs = if (show_diff || format == OutputFormat::Json) && result.has_conflicts {
        git::get_conflict_diffs(&current_branch, &base_ref, &result.conflicted_files)
    } else {
        Vec::new()
    };

    let base_has_conflicts = result.has_conflicts;

    // Handle output based on format
    match format {
        OutputFormat::Text => {
            if result.has_conflicts {
                ui::print_conflicts(&result.conflicted_files);
                if show_diff && !conflict_diffs.is_empty() {
                    ui::print_conflict_diffs(&conflict_diffs);
                }
            } else {
                ui::print_success(&current_branch, &base_ref);
            }
        }
        OutputFormat::Compact => {
            if result.has_conflicts {
                println!("CONFLICT: {}", result.conflicted_files.join(", "));
            } else {
                println!("OK: no conflicts");
            }
        }
        OutputFormat::Json => {
            // Collect PR results for JSON
            let pr_results = if !skip_prs {
                collect_pr_results(&current_branch, &base)
            } else {
                Vec::new()
            };

            let output = CheckOutput {
                current_branch: current_branch.clone(),
                base_branch: base.clone(),
                base_ref: base_ref.clone(),
                merge_result: result,
                stats,
                conflict_diffs,
                pr_results,
            };
            println!("{}", output.to_json());
            std::process::exit(if base_has_conflicts { 1 } else { 0 });
        }
    }

    // PR awareness check (text/compact only — JSON handles it above)
    if format != OutputFormat::Json && !skip_prs && is_text {
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
    let prs = fetch_open_prs(base_branch);

    let prs = match prs {
        Ok(prs) => prs,
        Err(e) => {
            ui::print_pr_warning(&format!("{}", e));
            return;
        }
    };

    if prs.is_empty() {
        ui::print_no_open_prs();
        return;
    }

    let prs: Vec<_> = prs
        .into_iter()
        .filter(|pr| pr.head_branch != current_branch)
        .collect();

    if prs.is_empty() {
        ui::print_no_open_prs();
        return;
    }

    ui::print_pr_check_header(prs.len());

    let mut results: Vec<github::PrCheckResult> = Vec::new();

    for pr in prs {
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

/// Collect PR results for JSON output (non-interactive)
fn collect_pr_results(current_branch: &str, base_branch: &str) -> Vec<github::PrCheckResult> {
    let prs = match fetch_open_prs(base_branch) {
        Ok(prs) => prs,
        Err(_) => return Vec::new(),
    };

    let prs: Vec<_> = prs
        .into_iter()
        .filter(|pr| pr.head_branch != current_branch)
        .collect();

    let mut results = Vec::new();

    for pr in prs {
        if git::fetch_remote_branch(&pr.head_branch).is_err() {
            results.push(github::PrCheckResult {
                pr,
                has_conflicts: false,
                conflicted_files: vec!["(could not fetch branch)".to_string()],
            });
            continue;
        }

        let pr_ref = format!("origin/{}", pr.head_branch);

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

    results
}

/// Fetch open PRs using the best available method
fn fetch_open_prs(base_branch: &str) -> Result<Vec<github::PullRequest>> {
    if github::gh_cli_available() {
        return github::list_prs_gh_cli(base_branch);
    }

    // Check env var first, then config file
    let token = github::github_token().or_else(|| {
        let cfg = crate::config::Config::load();
        cfg.github_token
    });

    if let Some(token) = token {
        let slug = github::get_repo_slug().context("Could not determine GitHub repository")?;
        return github::list_prs_api(base_branch, &token, &slug);
    }

    anyhow::bail!(
        "PR check requires either 'gh' CLI (run 'gh auth login') or GITHUB_TOKEN env var"
    );
}

use colored::Colorize;

const BOX_WIDTH: usize = 54;

/// Print the Preflight header banner
pub fn print_header() {
    println!();
    println!("{}", format!("╔{}╗", "═".repeat(BOX_WIDTH)).cyan());
    println!("{}", format!("║{:^w$}║", "PREFLIGHT", w = BOX_WIDTH).cyan());
    println!("{}", format!("║{:^w$}║", "Pre-push Merge Conflict Predictor", w = BOX_WIDTH).cyan());
    println!("{}", format!("╚{}╝", "═".repeat(BOX_WIDTH)).cyan());
}

/// Print a step with checkmark
#[allow(dead_code)]
pub fn print_step(message: &str, done: bool) {
    if done {
        println!("  {} {}", "✓".green().bold(), message.dimmed());
    } else {
        println!("  {} {}...", "⚡".yellow(), message.dimmed());
    }
}

/// Print the "Clear for takeoff" success message
pub fn print_success(current_branch: &str, base_branch: &str) {
    println!();
    println!("{}", format!("╭{}╮", "─".repeat(BOX_WIDTH)).green());
    println!("{}", format!("│{:^w$}│", "CLEAR FOR TAKEOFF", w = BOX_WIDTH).green().bold());
    println!("{}", format!("│{:^w$}│", "", w = BOX_WIDTH).green());
    println!("{}", format!("│{:^w$}│", "No conflicts detected. Safe to push!", w = BOX_WIDTH).green());
    println!("{}", format!("╰{}╯", "─".repeat(BOX_WIDTH)).green());
    println!();
    println!("  {} -> {}", current_branch.cyan(), base_branch.cyan());
}

/// Print the "Conflicts detected" warning message
pub fn print_conflicts(files: &[String]) {
    println!();
    println!("{}", format!("╭{}╮", "─".repeat(BOX_WIDTH)).red());
    println!("{}", format!("│{:^w$}│", "CONFLICT ALERT - HOLD FOR CLEARANCE", w = BOX_WIDTH).red().bold());
    println!("{}", format!("│{:^w$}│", "", w = BOX_WIDTH).red());
    println!("{}", format!("│{:^w$}│", "Merge conflicts detected. Resolve before pushing.", w = BOX_WIDTH).red());
    println!("{}", format!("╰{}╯", "─".repeat(BOX_WIDTH)).red());
    println!();
    println!("{}", "  Conflicting files:".yellow().bold());
    for file in files {
        println!("    {} {}", "✗".red(), file.red());
    }
    println!();
    println!("{}", "  Tip: Rebase or merge to resolve conflicts".dimmed());
}

/// Print error message
#[allow(dead_code)]
pub fn print_error(error: &str) {
    let w: usize = 44;
    println!();
    println!("{}", format!("╭{}╮", "─".repeat(w)).red());
    println!("{}", format!("│{:^w$}│", "ERROR", w = w).red().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).red());
    // Truncate error if too long
    let display_err = if error.len() > w - 2 {
        &error[..w - 2]
    } else {
        error
    };
    println!("{}", format!("│ {:<w$}│", display_err, w = w - 1).red());
    println!("{}", format!("╰{}╯", "─".repeat(w)).red());
    println!();
}

/// Print branch information
pub fn print_branch_info(current: &str, base: &str) {
    println!();
    println!("     {} {}", "Current Branch:".bold(), current.cyan());
    println!("        {} {}", "Base Branch:".bold(), base.cyan());
    println!();
}

/// Print a loading/progress message
pub fn print_loading(message: &str) {
    println!("  {} {}...", "⚡".yellow(), message.dimmed());
}

/// Print the "already on base branch" message
pub fn print_already_on_base(branch: &str) {
    let w: usize = 44;
    let msg = format!("You're on {} - no check needed", branch);
    println!();
    println!("{}", format!("╭{}╮", "─".repeat(w)).yellow());
    println!("{}", format!("│{:^w$}│", "ALREADY ON BASE BRANCH", w = w).yellow().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).yellow());
    println!("{}", format!("│{:^w$}│", msg, w = w).yellow());
    println!("{}", format!("╰{}╯", "─".repeat(w)).yellow());
    println!();
}

/// Alternative: Compact mode (for --quiet or when piped)
#[allow(dead_code)]
pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

#[allow(dead_code)]
pub fn print_compact_success() {
    println!("{}", "✓ No conflicts".green());
}

#[allow(dead_code)]
pub fn print_compact_conflicts(files: &[String]) {
    println!("{}", "✗ Conflicts detected:".red());
    for file in files {
        println!("  {}", file);
    }
}

/// Print branch statistics
pub fn print_stats(stats: &crate::git::BranchStats) {
    println!();
    println!("{}", "  📊 Branch Statistics".bold().cyan());
    println!("  {}", "─".repeat(50).dimmed());

    // Ahead/Behind
    if stats.ahead > 0 || stats.behind > 0 {
        print!("  ");
        if stats.ahead > 0 {
            print!("{} {}", "↑".green(), format!("{} ahead", stats.ahead).green());
        }
        if stats.ahead > 0 && stats.behind > 0 {
            print!("  ");
        }
        if stats.behind > 0 {
            print!("{} {}", "↓".yellow(), format!("{} behind", stats.behind).yellow());
        }
        println!();
    }

    // Files changed
    if stats.files_changed > 0 {
        println!("  {} {} files changed", "📝".dimmed(), stats.files_changed.to_string().cyan());
    }

    // Insertions/Deletions
    if stats.insertions > 0 || stats.deletions > 0 {
        print!("  ± ");
        if stats.insertions > 0 {
            print!("{} ", format!("+{}", stats.insertions).green());
        }
        if stats.deletions > 0 {
            print!("{}", format!("-{}", stats.deletions).red());
        }
        println!();
    }

    // Merge base
    if !stats.merge_base.is_empty() {
        println!("  🔗 Merge base: {}", stats.merge_base.dimmed());
    }

    println!("  {}", "─".repeat(50).dimmed());
}

// ─── Hook Installation Messages ───────────────────────────────────────────────

/// Print hook successfully installed
pub fn print_hook_installed(path: &str) {
    let w: usize = 54;
    println!("{}", format!("╭{}╮", "─".repeat(w)).green());
    println!("{}", format!("│{:^w$}│", "HOOK INSTALLED", w = w).green().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).green());
    println!("{}", format!("│{:^w$}│", "Preflight will now run on every push.", w = w).green());
    println!("{}", format!("╰{}╯", "─".repeat(w)).green());
    println!();
    println!("  {} {}", "Location:".bold(), path.dimmed());
    println!("  {} {}", "Bypass:".bold(), "git push --no-verify".dimmed());
    println!();
}

/// Print hook chained with existing hook
pub fn print_hook_chained() {
    let w: usize = 54;
    println!("{}", format!("╭{}╮", "─".repeat(w)).green());
    println!("{}", format!("│{:^w$}│", "HOOK CHAINED", w = w).green().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).green());
    println!("{}", format!("│{:^w$}│", "Appended to existing pre-push hook.", w = w).green());
    println!("{}", format!("╰{}╯", "─".repeat(w)).green());
    println!();
    println!("  {} Your existing hook will still run.", "Note:".bold());
    println!("  {} {}", "Bypass:".bold(), "git push --no-verify".dimmed());
    println!();
}

/// Print hook already installed
pub fn print_hook_already_installed() {
    let w: usize = 44;
    println!("{}", format!("╭{}╮", "─".repeat(w)).yellow());
    println!("{}", format!("│{:^w$}│", "ALREADY INSTALLED", w = w).yellow().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).yellow());
    println!("{}", format!("│{:^w$}│", "Preflight hook is already active.", w = w).yellow());
    println!("{}", format!("╰{}╯", "─".repeat(w)).yellow());
    println!();
}

/// Print warning: existing hook found
pub fn print_hook_exists(path: &str) {
    let w: usize = 54;
    println!("{}", format!("╭{}╮", "─".repeat(w)).red());
    println!("{}", format!("│{:^w$}│", "EXISTING HOOK DETECTED", w = w).red().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).red());
    println!("{}", format!("│{:^w$}│", "A pre-push hook already exists.", w = w).red());
    println!("{}", format!("╰{}╯", "─".repeat(w)).red());
    println!();
    println!("  {} {}", "Path:".bold(), path.dimmed());
    println!();
    println!("  {} Use {} to chain with the existing hook.", "Tip:".bold(), "--force".cyan());
    println!("       This will append preflight to your existing hook.");
    println!();
}

/// Print hook removed successfully
pub fn print_hook_removed() {
    let w: usize = 44;
    println!("{}", format!("╭{}╮", "─".repeat(w)).green());
    println!("{}", format!("│{:^w$}│", "HOOK REMOVED", w = w).green().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).green());
    println!("{}", format!("│{:^w$}│", "Pre-push hook has been uninstalled.", w = w).green());
    println!("{}", format!("╰{}╯", "─".repeat(w)).green());
    println!();
}

/// Print hook section removed (other hooks remain)
pub fn print_hook_section_removed() {
    let w: usize = 54;
    println!("{}", format!("╭{}╮", "─".repeat(w)).green());
    println!("{}", format!("│{:^w$}│", "HOOK REMOVED", w = w).green().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).green());
    println!("{}", format!("│{:^w$}│", "Preflight section removed from pre-push hook.", w = w).green());
    println!("{}", format!("╰{}╯", "─".repeat(w)).green());
    println!();
    println!("  {} Other hook content was preserved.", "Note:".bold());
    println!();
}

/// Print no hook found
pub fn print_hook_not_found() {
    let w: usize = 44;
    println!("{}", format!("╭{}╮", "─".repeat(w)).yellow());
    println!("{}", format!("│{:^w$}│", "NO HOOK FOUND", w = w).yellow().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).yellow());
    println!("{}", format!("│{:^w$}│", "No pre-push hook exists.", w = w).yellow());
    println!("{}", format!("╰{}╯", "─".repeat(w)).yellow());
    println!();
}

/// Print preflight not installed in hook
pub fn print_hook_not_installed() {
    let w: usize = 54;
    println!("{}", format!("╭{}╮", "─".repeat(w)).yellow());
    println!("{}", format!("│{:^w$}│", "NOT INSTALLED", w = w).yellow().bold());
    println!("{}", format!("│{:^w$}│", "", w = w).yellow());
    println!("{}", format!("│{:^w$}│", "Preflight is not in the current pre-push hook.", w = w).yellow());
    println!("{}", format!("╰{}╯", "─".repeat(w)).yellow());
    println!();
}

// ─── PR Awareness Messages ────────────────────────────────────────────────────

/// Print PR check section header
pub fn print_pr_check_header(count: usize) {
    println!();
    println!("  {}", "─".repeat(50).dimmed());
    println!(
        "  {} Checking {} open {} for conflicts...",
        "🔍".dimmed(),
        count.to_string().cyan(),
        if count == 1 { "PR" } else { "PRs" }
    );
    println!("  {}", "─".repeat(50).dimmed());
}

/// Print PR check results
pub fn print_pr_results(results: &[crate::github::PrCheckResult]) {
    let conflicts: Vec<_> = results.iter().filter(|r| r.has_conflicts).collect();
    let clean: Vec<_> = results.iter().filter(|r| !r.has_conflicts).collect();

    println!();

    // Show clean PRs
    for result in &clean {
        println!(
            "  {} #{} {} {}",
            "✓".green(),
            result.pr.number.to_string().dimmed(),
            result.pr.head_branch.cyan(),
            format!("({})", result.pr.author).dimmed()
        );
        if !result.pr.title.is_empty() {
            println!("      {}", result.pr.title.dimmed());
        }
    }

    // Show conflicting PRs
    for result in &conflicts {
        println!(
            "  {} #{} {} {}",
            "✗".red(),
            result.pr.number.to_string().bold(),
            result.pr.head_branch.red(),
            format!("({})", result.pr.author).dimmed()
        );
        if !result.pr.title.is_empty() {
            println!("      {}", result.pr.title.dimmed());
        }
        for file in &result.conflicted_files {
            println!("      {} {}", "↳".red(), file.red());
        }
    }

    // Summary
    println!();
    if conflicts.is_empty() {
        println!(
            "  {} No conflicts with open PRs",
            "✓".green().bold()
        );
    } else {
        println!(
            "  {} {} {} with conflicts",
            "⚠".yellow().bold(),
            conflicts.len(),
            if conflicts.len() == 1 { "PR" } else { "PRs" }
        );
    }
    println!();
}

/// Print warning when PR check can't run
pub fn print_pr_warning(message: &str) {
    println!();
    println!(
        "  {} {} {}",
        "⚠".yellow(),
        "PR check skipped:".yellow(),
        message.dimmed()
    );
    println!(
        "  {} Use {} to silence this.",
        " ".dimmed(),
        "--skip-prs".cyan()
    );
    println!();
}

/// Print when there are no open PRs
pub fn print_no_open_prs() {
    println!();
    println!(
        "  {} {}",
        "ℹ".dimmed(),
        "No other open PRs targeting this base.".dimmed()
    );
}

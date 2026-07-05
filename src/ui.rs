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
pub fn is_tty() -> bool {
    atty::is(atty::Stream::Stdout)
}

pub fn print_compact_success() {
    println!("{}", "✓ No conflicts".green());
}

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

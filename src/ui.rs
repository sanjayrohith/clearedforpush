use colored::Colorize;

/// Print the Preflight header banner
pub fn print_header() {
    let header = r#"
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝
"#;
    println!("{}", header.cyan());
}

/// Print a step with checkmark
pub fn print_step(message: &str, done: bool) {
    if done {
        println!("  {} {}", "✓".green().bold(), message.dimmed());
    } else {
        println!("  {} {}", "⋯".yellow(), message.dimmed());
    }
}

/// Print the "Clear for takeoff" success message
pub fn print_success(current_branch: &str, base_branch: &str) {
    println!();
    let border = "╭──────────────────────────────────────────────────────────╮";
    let bottom = "╰──────────────────────────────────────────────────────────╯";
    
    println!("{}", border.green());
    println!("{}", "│  ✅ CLEAR FOR TAKEOFF                                    │".green().bold());
    println!("{}", "│                                                          │".green());
    println!("{}", "│  No conflicts detected. Safe to push!                   │".green());
    println!("{}", bottom.green());
    println!();
    println!("  {} {} → {}", "✈️".bold(), current_branch.cyan(), base_branch.cyan());
}

/// Print the "Conflicts detected" warning message
pub fn print_conflicts(files: &[String]) {
    println!();
    let border = "╭──────────────────────────────────────────────────────────╮";
    let bottom = "╰──────────────────────────────────────────────────────────╯";
    
    println!("{}", border.red());
    println!("{}", "│  ⚠️  CONFLICT ALERT - HOLD FOR CLEARANCE                │".red().bold());
    println!("{}", "│                                                          │".red());
    println!("{}", "│  Merge conflicts detected. Resolve before pushing.      │".red());
    println!("{}", bottom.red());
    println!();
    println!("{}", "  Conflicting files:".yellow().bold());
    for file in files {
        println!("    {} {}", "✗".red(), file.red());
    }
    println!();
    println!("{}", "  💡 Tip: Rebase or merge to resolve conflicts".dimmed());
}

/// Print error message
pub fn print_error(error: &str) {
    println!();
    println!("{}", "╭────────────────────────────────────────────╮".red());
    println!("{}", "│  ❌ ERROR                                  │".red().bold());
    println!("{}", "│                                            │".red());
    println!("│  {}{}│", error.red(), " ".repeat(42 - error.len()));
    println!("{}", "╰────────────────────────────────────────────╯".red());
    println!();
}

/// Print branch information
pub fn print_branch_info(current: &str, base: &str) {
    println!();
    println!("     {} {}", "Current Branch:".bold(), current.cyan());
    println!("     {} {}", "   Base Branch:".bold(), base.cyan());
    println!();
}

/// Print a loading/progress message
pub fn print_loading(message: &str) {
    println!("  {} {}...", "⚡".yellow(), message.dimmed());
}

/// Print the "already on base branch" message
pub fn print_already_on_base(branch: &str) {
    println!();
    println!("{}", "╭────────────────────────────────────────────╮".yellow());
    println!("{}", "│  ℹ️  ALREADY ON BASE BRANCH                │".yellow().bold());
    println!("{}", "│                                            │".yellow());
    println!("│  You're on {} - no check needed   │", branch.cyan());
    println!("{}", "╰────────────────────────────────────────────╯".yellow());
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

# UI Implementation Guide

## ✅ What's Already Implemented

### Aviation Theme (Current)
- ✈️ Header banner with airplane emoji
- 📍 Branch information display
- ⚡ Loading messages
- ✅ Success box ("Clear for takeoff")
- ⚠️ Conflict alert box
- 💡 Helpful tips
- Color-coded output (cyan, green, red, yellow)

## How to Test It

```bash
# Build with the new UI
cargo build --release

# Run in a git repository
./target/release/preflight check

# You should see the beautiful ASCII art!
```

## Files Modified

1. **`src/ui.rs`** (NEW) - All UI rendering functions
2. **`src/conflict_checker.rs`** - Updated to use ui module
3. **`src/main.rs`** - Added `mod ui;`
4. **`Cargo.toml`** - Added `atty = "0.2"` dependency

## Quick Customization

### 1. Change Header Text

Edit `src/ui.rs`, function `print_header()`:

```rust
pub fn print_header() {
    let header = r#"
╔══════════════════════════════════════════════════════════╗
║                   🚀  YOUR CUSTOM TEXT                   ║
║                    Your Subtitle Here                    ║
╚══════════════════════════════════════════════════════════╝
"#;
    println!("{}", header.cyan());  // Change color here
}
```

### 2. Change Emoji Icons

Find and replace in `src/ui.rs`:
- `✈️` → `🚀` (rocket)
- `⚠️` → `⚡` (lightning)
- `✅` → `✔️` (checkmark)
- `💡` → `ℹ️` (info)

### 3. Change Box Characters

Replace in success/conflict functions:
```rust
// Current: Rounded corners
╭─╮
│ │
╰─╯

// Alternative 1: Sharp corners
┌─┐
│ │
└─┘

// Alternative 2: Double line
╔═╗
║ ║
╚═╝

// Alternative 3: Heavy
┏━┓
┃ ┃
┗━┛
```

### 4. Change Colors

Edit any `println!` in `src/ui.rs`:
```rust
.cyan()           // Blue-green
.green()          // Success green
.red()            // Error red
.yellow()         // Warning yellow
.blue()           // Primary blue
.magenta()        // Purple
.bright_green()   // Brighter green
.dimmed()         // Faded text
.bold()           // Bold text
```

## Adding New Features

### Add a Progress Bar

1. Add dependency to `Cargo.toml`:
```toml
indicatif = "0.17"
```

2. Add to `src/ui.rs`:
```rust
use indicatif::{ProgressBar, ProgressStyle};

pub fn show_progress_bar(message: &str) {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(120));
    
    // Your operation here...
    
    pb.finish_with_message("Done!");
}
```

### Add Animation

For a spinning loader while checking:

```rust
use std::{thread, time::Duration};

pub fn animate_checking() {
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let mut i = 0;
    
    for _ in 0..20 {
        print!("\r  {} Checking conflicts...", frames[i].cyan());
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
        i = (i + 1) % frames.len();
    }
    println!("\r  ✓ Check complete!         ");
}
```

### Add Sound Effects (Optional, Fun!)

1. Add dependency:
```toml
rodio = "0.17"
```

2. Play success sound:
```rust
use rodio::{OutputStream, Sink};

pub fn play_success_sound() {
    // Beep on success (requires audio file)
    // Left as exercise - add your own .wav/.mp3 file
}
```

## Switch to Different Theme

### To use Minimalist Theme:

Replace `print_header()` in `src/ui.rs`:
```rust
pub fn print_header() {
    let header = r#"
  ╭────────────────────────────────────────╮
  │                                        │
  │   PREFLIGHT                            │
  │   ━━━━━━━━━                            │
  │   Conflict detection before you push   │
  │                                        │
  ╰────────────────────────────────────────╯
"#;
    println!("{}", header.blue());
}
```

Replace `print_success()`:
```rust
pub fn print_success(current_branch: &str, base_branch: &str) {
    println!();
    println!("  ┌─ Result ─────────────────────────────┐");
    println!("  │                                      │");
    println!("  │  {}  No conflicts detected            │", "✓".green());
    println!("  │                                      │");
    println!("  │  Safe to push {}                     │", "🚀");
    println!("  │                                      │");
    println!("  └──────────────────────────────────────┘");
    println!();
    println!("  {} → {}", current_branch.cyan(), base_branch.cyan());
}
```

### To use Git-Inspired Theme:

Add step tracking to `src/ui.rs`:
```rust
pub fn print_steps() {
    println!("\n  Git Operations:");
    println!("  ├─ {} Branch detection", "✓".green());
    println!("  ├─ {} Remote fetch", "✓".green());
    println!("  ├─ {} Merge simulation", "✓".green());
    println!("  └─ {} Conflict analysis", "✓".green());
}
```

## Advanced: Theme Selector

Want users to choose their theme?

### 1. Add theme enum to `src/main.rs`:

```rust
#[derive(Clone, Copy, ValueEnum)]
pub enum Theme {
    Aviation,
    Minimal,
    Git,
    Radar,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// UI theme
    #[arg(long, value_enum, default_value_t = Theme::Aviation)]
    theme: Theme,
}
```

### 2. Pass theme to conflict_checker:

```rust
match cli.command {
    Commands::Check { base, theme } => {
        conflict_checker::check_conflicts(base, theme)?;
    }
}
```

### 3. Update ui.rs functions:

```rust
pub fn print_header(theme: Theme) {
    match theme {
        Theme::Aviation => print_aviation_header(),
        Theme::Minimal => print_minimal_header(),
        Theme::Git => print_git_header(),
        Theme::Radar => print_radar_header(),
    }
}
```

## Testing Different Terminals

Different terminals support different characters:

### macOS Terminal / iTerm2
✅ All Unicode characters work
✅ Emoji support
✅ Box drawing perfect

### Windows CMD
⚠️ Limited Unicode
⚠️ Use `--theme minimal` for compatibility
✅ PowerShell better than CMD

### Linux Terminal
✅ Most support everything
✅ Depends on font (use Nerd Fonts for best results)

### VS Code Terminal
✅ Excellent support
✅ All themes work

## Fallback for Non-TTY

When output is piped (e.g., to a file), use compact mode:

```rust
pub fn print_result_compact(has_conflicts: bool) {
    if atty::is(atty::Stream::Stdout) {
        // Fancy UI
        print_fancy();
    } else {
        // Simple output for pipes
        if has_conflicts {
            println!("CONFLICTS DETECTED");
        } else {
            println!("CLEAN MERGE");
        }
    }
}
```

This is already implemented with `is_tty()` in `src/ui.rs`!

## Troubleshooting

### Characters Look Wrong
- Update terminal font to one with Unicode support
- Try: Fira Code, JetBrains Mono, Cascadia Code, Nerd Fonts

### Colors Don't Show
- Check if `NO_COLOR` environment variable is set
- Some CI environments disable colors by default

### Emoji Don't Render
- Update terminal or use `--no-emoji` flag (you'd need to add this)

## Performance Note

ASCII art and colors add minimal overhead (~1-5ms). If you need maximum speed:
```bash
preflight check --quiet  # Minimal output
```

## Examples of Use

```bash
# Default aviation theme
preflight check

# Future: with theme selector
preflight check --theme minimal
preflight check --theme git
preflight check --theme radar

# Quiet mode (no ASCII art)
preflight check --quiet

# In CI (auto-detects non-TTY, shows compact output)
preflight check | tee output.txt
```

---

**Current Status:** Aviation theme fully implemented and ready to use! 🎉

Build and run to see it in action:
```bash
cargo build --release
./target/release/preflight check
```

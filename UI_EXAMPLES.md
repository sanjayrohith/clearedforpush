# Preflight UI Examples

## Current Implementation: Aviation Theme ✈️

### Success (No Conflicts)

```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-auth
        Base Branch: main

  ⚡ Detecting current branch...
  ⚡ Detecting base branch...
  ⚡ Fetching latest from origin...
  ⚡ Simulating merge...

╭──────────────────────────────────────────────────────────╮
│  ✅ CLEAR FOR TAKEOFF                                    │
│                                                          │
│  No conflicts detected. Safe to push!                   │
╰──────────────────────────────────────────────────────────╯

  ✈️  feature-auth → origin/main
```

### Conflicts Detected

```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-auth
        Base Branch: main

  ⚡ Detecting current branch...
  ⚡ Fetching latest from origin...
  ⚡ Simulating merge...

╭──────────────────────────────────────────────────────────╮
│  ⚠️  CONFLICT ALERT - HOLD FOR CLEARANCE                │
│                                                          │
│  Merge conflicts detected. Resolve before pushing.      │
╰──────────────────────────────────────────────────────────╯

  Conflicting files:
    ✗ src/auth.rs
    ✗ src/main.rs
    ✗ src/lib.rs

  💡 Tip: Rebase or merge to resolve conflicts
```

### Already on Base Branch

```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: main
        Base Branch: main

╭────────────────────────────────────────────╮
│  ℹ️  ALREADY ON BASE BRANCH                │
│                                            │
│  You're on main - no check needed         │
╰────────────────────────────────────────────╯
```

---

## Alternative UI Themes (Not Implemented)

### Option 2: Radar Theme 📡

```
    ╔═══════════════════════════════════════════╗
    ║   PREFLIGHT CONFLICT SCANNER v0.1.0      ║
    ╚═══════════════════════════════════════════╝

       ┌─────────────────────────────────┐
       │  Scanning for conflicts...      │
       │                                 │
       │      ╭─────╮                    │
       │      │  ●  │ ← Your Branch      │
       │      ╰──┬──╯                    │
       │         │                       │
       │      ╭──┴──╮                    │
       │      │  ●  │ ← Base Branch      │
       │      ╰─────╯                    │
       └─────────────────────────────────┘

    ┌───────────────────────────────────────┐
    │ STATUS: ✅ CLEAN MERGE               │
    │ FILES:  5 files changed, 0 conflicts │
    └───────────────────────────────────────┘
```

**To switch to this theme:** Modify `src/ui.rs` header and success functions.

### Option 3: Minimalist Theme ⚡

```
  ╭────────────────────────────────────────╮
  │                                        │
  │   PREFLIGHT                            │
  │   ━━━━━━━━━                            │
  │   Conflict detection before you push   │
  │                                        │
  ╰────────────────────────────────────────╯

  → feature-auth
  ← origin/main

  ⚡ Checking conflicts...

  ┌─ Result ─────────────────────────────┐
  │                                      │
  │  ✓  No conflicts detected            │
  │                                      │
  │  Safe to push 🚀                     │
  │                                      │
  └──────────────────────────────────────┘
```

**To switch to this theme:** Use simpler box drawing characters in `src/ui.rs`.

### Option 4: Git-Inspired Theme 🔀

```
╭─────────────────────────────────────────────╮
│  PREFLIGHT                                  │
│  Git Merge Conflict Predictor               │
╰─────────────────────────────────────────────╯

  Current:  feature-auth
  Base:     origin/main
  Status:   Checking...

  Git Operations:
  ├─ ✓ Branch detection
  ├─ ✓ Remote fetch
  ├─ ✓ Merge simulation
  └─ ✓ Conflict analysis

╔═══════════════════════════════════════════╗
║  RESULT: Clean Merge ✅                   ║
║                                           ║
║  All files can merge automatically        ║
║  Ready to push your changes               ║
╚═══════════════════════════════════════════╝
```

**To switch to this theme:** Add step-by-step tree output in conflict_checker.rs.

---

## How to Customize

### Change Colors

Edit `src/ui.rs` and modify the color methods:
```rust
.cyan()    → .blue()
.green()   → .bright_green()
.red()     → .bright_red()
.yellow()  → .bright_yellow()
```

### Change Characters

Replace box-drawing characters in `src/ui.rs`:
```rust
// Current (double line)
"╔══════╗"
"║      ║"
"╚══════╝"

// Alternative (single line)
"┌──────┐"
"│      │"
"└──────┘"

// Alternative (thick)
"┏━━━━━━┓"
"┃      ┃"
"┗━━━━━━┛"
```

### Add Progress Bar

Use the `indicatif` crate:
```toml
[dependencies]
indicatif = "0.17"
```

```rust
use indicatif::{ProgressBar, ProgressStyle};

let pb = ProgressBar::new(100);
pb.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {msg}")
    .unwrap());
```

### Add Animations

For spinning loader:
```rust
use std::thread;
use std::time::Duration;

let spinners = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
for spinner in spinners.iter().cycle().take(20) {
    print!("\r  {} Checking conflicts...", spinner);
    thread::sleep(Duration::from_millis(100));
}
```

---

## Quick Theme Switching

Want to try different themes without rewriting code? Add a theme flag:

```rust
#[derive(Parser)]
struct Cli {
    #[arg(long, value_enum, default_value_t = Theme::Aviation)]
    theme: Theme,
}

#[derive(Clone, ValueEnum)]
enum Theme {
    Aviation,
    Radar,
    Minimal,
    Git,
}
```

Then use `match` statements in `ui.rs` to switch styles.

---

## Color Reference

| Color | Usage |
|-------|-------|
| Cyan | Branch names, headers |
| Green | Success messages, checkmarks |
| Red | Errors, conflicts |
| Yellow | Warnings, tips |
| Dimmed | Progress messages |
| Bold | Important text |

---

## Box Drawing Characters Reference

```
Single Line:
┌─┬─┐
│ │ │
├─┼─┤
└─┴─┘

Double Line:
╔═╦═╗
║ ║ ║
╠═╬═╣
╚═╩═╝

Rounded:
╭─┬─╮
│ │ │
├─┼─┤
╰─┴─╯

Heavy:
┏━┳━┓
┃ ┃ ┃
┣━╋━┫
┗━┻━┛
```

Choose based on your terminal font support!

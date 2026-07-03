<div align="center">

# ✈️ Preflight

### Know before you push

**Pre-push merge conflict detector with beautiful CLI**

Discover merge conflicts before you push — not during CI or PR review.

[![Crates.io](https://img.shields.io/crates/v/preflight.svg)](https://crates.io/crates/preflight)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

[Quick Start](#quick-start) • [Features](#features) • [Install](#installation) • [Usage](#usage)

---

</div>

## The Problem

You've been there:

```bash
$ git push origin feature-branch
# ✅ Pushed!

# Later...
❌ CI fails: "Merge conflicts detected"
# Or worse, during PR review...
```

By then, context is lost. Fixing conflicts becomes painful.

## The Solution

**Preflight checks for conflicts before you push.**

```bash
$ preflight check

✅ CLEAR FOR TAKEOFF
No conflicts detected. Safe to push!
```

Simple. Fast. Safe.

---

## Quick Start

```bash
# Install
cargo install preflight

# Check for conflicts
cd your-git-repo
preflight check

# See detailed stats
preflight check --stats
```

That's it! If there are conflicts, you'll know immediately — with full context to fix them.

---

## Features

### ✈️ Beautiful CLI
Aviation-themed interface that makes conflict checking delightful.

### ⚡ Lightning Fast
Uses Git's native `merge-tree` — typically under 2 seconds.

### 🛡️ 100% Safe
Read-only operations. Never touches your working directory, index, or branches.

### 📊 Smart Statistics
See how far ahead/behind you are, files changed, and insertions/deletions.

### 🎯 Accurate
Uses Git's own merge algorithm — same results as actual merges.

### 🔧 Hook-Ready
Fast enough to run automatically on every push.

---

## Installation

### From crates.io (Recommended)

```bash
cargo install preflight
```

### From Source

```bash
git clone https://github.com/yourusername/preflight
cd preflight
cargo install --path .
```

### Requirements
- **Git 2.38+** (check with `git --version`)
- **Rust 1.70+** (for building from source)

---

## Usage

### Basic Check

```bash
preflight check
```

Checks if your current branch will conflict with the base branch (auto-detected as `main` or `master`).

### With Statistics

```bash
preflight check --stats
```

Shows detailed information:
- **↑ Ahead**: Commits you're ahead of base
- **↓ Behind**: Commits base is ahead of you  
- **📝 Files changed**: Number of modified files
- **± Lines**: Insertions and deletions

### Custom Base Branch

```bash
preflight check --base develop
```

Check against a different branch.

### Example Output

<details>
<summary><b>✅ No Conflicts</b></summary>

```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-auth
        Base Branch: main

  📊 Branch Statistics
  ──────────────────────────────────────────────────────────
  ↑ 5 ahead  ↓ 2 behind
  📝 12 files changed
  ± +234 -56
  � BMerge base: abc123d
  ──────────────────────────────────────────────────────────

╭──────────────────────────────────────────────────────────╮
│  ✅ CLEAR FOR TAKEOFF                                    │
│                                                          │
│  No conflicts detected. Safe to push!                   │
╰──────────────────────────────────────────────────────────╯

  ✈️  feature-auth → origin/main
```
</details>

<details>
<summary><b>❌ Conflicts Detected</b></summary>

```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-refactor
        Base Branch: main

╭──────────────────────────────────────────────────────────╮
│  ⚠️  CONFLICT ALERT - HOLD FOR CLEARANCE                │
│                                                          │
│  Merge conflicts detected. Resolve before pushing.      │
╰──────────────────────────────────────────────────────────╯

  Conflicting files:
    ✗ src/auth.rs
    ✗ src/main.rs

  💡 Tip: Rebase or merge to resolve conflicts
```
</details>

---

## How It Works

Preflight uses Git's native `merge-tree` command in `--write-tree` mode:

```bash
git merge-tree --write-tree <base-branch> <current-branch>
```

This computes what a merge would look like **without actually performing it**.

**What happens:**
1. Detects your current branch and base branch
2. Fetches latest from remote (read-only)
3. Simulates the merge using Git's algorithm
4. Reports conflicts if any exist

**What doesn't happen:**
- ❌ No working directory changes
- ❌ No index modifications
- ❌ No branch updates
- ❌ No stash operations

100% safe. Always.

---

## Why Preflight?

| Without Preflight | With Preflight |
|-------------------|----------------|
| Push → CI fails → Fix conflicts | Check → Fix conflicts → Push ✅ |
| Context lost | Full context |
| Team blocked | Team unblocked |
| Wasted CI time | Clean CI runs |
| Frustrating | Smooth workflow |

---

## Use Cases

### 🚀 Before Pushing
```bash
preflight check && git push
```
Only push if no conflicts detected.

### 📊 Code Review Prep
```bash
preflight check --stats
```
Include stats in your PR description.

### 🪝 Git Hooks *(Coming Soon)*
```bash
# Automatically check on every push
preflight install-hook
```

### 👥 Team Workflows
Catch conflicts early, keep the team unblocked.

---

## Roadmap

- [x] **Phase 1**: Core conflict detection ✅
- [x] **Phase 1.1**: Statistics display ✅  
- [ ] **Phase 2**: Auto-install git hooks
- [ ] **Phase 3**: Check against open PRs
- [ ] **Phase 4**: Team dashboard
- [ ] **Phase 5**: Configuration files
- [ ] **Phase 6**: More integrations

[See full roadmap](docs/features/FEATURE_ROADMAP.md)

---



## FAQ

### Does Preflight modify my repository?

**No.** Preflight uses Git's `merge-tree` plumbing command which only reads data. Your working directory, index, HEAD, and all branches remain completely untouched. This is a hard guarantee.

### What Git version do I need?

Git 2.38.0 or later (released October 2022). This version introduced the `--write-tree` flag for `merge-tree`. Check your version with `git --version`.

### What if I'm already on the base branch?

Preflight detects this and exits cleanly with a message. No conflict check is needed if you're already on main.

### Can I use this with branches other than main?

Yes! Use `preflight check --base develop` or any other branch name.

### Does this work with remote branches?

Yes. Preflight automatically fetches the latest state of the base branch from origin before checking, so you're always comparing against the most recent remote state.

### What about merge conflicts in open PRs?

Planned for Phase 3! The tool will check your branch against all open PRs, not just the base branch.

### Can I use this in CI?

Absolutely. Use the exit codes (0 = clean, 1 = conflicts) in your CI scripts. JSON output mode is planned for Phase 4.

### Is it fast enough for a git hook?

Yes! Designed to run in under 2 seconds for typical repos. Phase 2 will add automatic hook installation.

### What about Windows support?

Should work (Rust is cross-platform), but not extensively tested yet. Contributions welcome!

---

## Contributing

Contributions welcome! Check out [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

**Ideas we'd love help with:**
- Windows testing and support
- Git hook auto-installation
- Config file support
- PR conflict detection

---

## License

Licensed under either of:

- **MIT License** ([LICENSE-MIT](LICENSE-MIT))
- **Apache License 2.0** ([LICENSE-APACHE](LICENSE-APACHE))

at your option.

---

<div align="center">

**Built with ❤️ by developers, for developers**

[Report Bug](https://github.com/yourusername/preflight/issues) • [Request Feature](https://github.com/yourusername/preflight/issues)

</div>

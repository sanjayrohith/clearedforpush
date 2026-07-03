# Preflight

**Pre-push merge conflict predictor**

Preflight tells you whether your branch will conflict with your base branch *before* you push, without touching your working directory.

## Status

🚧 **Draft v1 - Phase 1 (MVP) in progress**

## Problem

You discover merge conflicts too late - when you try to merge, when CI fails, or during PR review. Preflight gives you that feedback *before* you push.

## How It Works

Preflight uses Git's native `merge-tree` plumbing to compute what a merge would look like, without actually performing the merge. Your working directory, index, and branches remain untouched.

## Installation

**Not yet published.** Phase 1 MVP in progress.

Once stable:
```bash
cargo install preflight
```

## Usage (Planned - Phase 1)

```bash
# Check if current branch conflicts with main
preflight check

# Specify base branch
preflight check --base develop
```

Exit codes:
- `0`: Clean merge, no conflicts
- `1`: Conflicts detected
- `2`: Error (not a git repo, git too old, etc.)

## Roadmap

- [x] Phase 0: Task 0 verification of `git merge-tree` behavior
- [ ] Phase 1: MVP - check current branch vs base branch
- [ ] Phase 2: Git hook integration (`preflight install-hook`)
- [ ] Phase 3: GitHub PR awareness
- [ ] Phase 4: Better reporting (show hunks, colors, JSON output)
- [ ] Phase 5: Configuration file support
- [ ] Phase 6: Distribution (crates.io, binaries, AUR)

## Requirements

- Git ≥ 2.38 (for modern `merge-tree --write-tree` support)
- Rust ≥ 1.70 (for building from source)

**Note:** Minimum Git version is not yet finalized - pending Task 0 verification.

## Non-Goals

- This does NOT resolve conflicts automatically
- This does NOT replace `git merge` or `git rebase`
- This is NOT a GUI
- This is NOT a general Git workflow manager

## Development

See `TASK_0_VERIFICATION.md` for critical first step before implementing the parser.

```bash
cargo build
cargo test
cargo run -- check
```

## License

MIT OR Apache-2.0

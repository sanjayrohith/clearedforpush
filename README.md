# Preflight

**Pre-push merge conflict predictor**

Preflight tells you whether your branch will conflict with your base branch *before* you push, without touching your working directory.

## Status

✅ **Phase 1 MVP - Ready for Testing**

Task 0 verification complete! The git merge-tree parser is implemented and ready to use.

✨ **New:** Beautiful ASCII art UI with aviation theme!

## Requirements

- **Git ≥ 2.38.0** (for modern `merge-tree --write-tree` support)
- **Rust ≥ 1.70** (for building from source)

To check your Git version:
```bash
git --version
```

If you have Git < 2.38, please upgrade for full functionality.

## Problem

You discover merge conflicts too late - when you try to merge, when CI fails, or during PR review. Preflight gives you that feedback *before* you push.

## Quick Start

```bash
# 1. Build Preflight
cargo build --release

# 2. Navigate to your git repository
cd your-project

# 3. Check for conflicts before pushing
./path/to/preflight/target/release/preflight check

# 4. If clean, push safely
git push origin your-branch
```

That's it! Preflight will tell you if your branch conflicts with main before you push.

## How It Works

Preflight uses Git's native `merge-tree` plumbing command to simulate a merge without actually performing it. Your working directory, index, and branches remain completely untouched.

### Behind the Scenes

When you run `preflight check`:

1. **Detects branches:** Identifies your current branch and base branch (main/master)
2. **Fetches latest:** Updates remote refs (read-only, no local changes)
3. **Simulates merge:** Runs `git merge-tree --write-tree` to compute merge result
4. **Parses conflicts:** Extracts any conflicting files from the output
5. **Reports results:** Shows you exactly what would conflict

**Safety guarantee:** Nothing in your repository is ever modified. We only read Git's internal data.

## Installation

### From Source (Current)

```bash
# Clone the repository
git clone <repository-url>
cd preflight

# Build
cargo build --release

# Run
./target/release/preflight check

# Optional: Install to PATH
cargo install --path .
# Now you can use: preflight check
```

### Pre-built Binaries (Coming Soon)

Once published (Phase 6), you'll be able to install via:
```bash
# Cargo
cargo install preflight

# Homebrew (macOS)
brew install preflight

# Other package managers coming soon
```

## Usage

### Basic Usage

```bash
# Check if current branch conflicts with main
preflight check

# Specify base branch explicitly
preflight check --base develop

# Get help
preflight --help
```

### Example Output

**No conflicts:**
```bash
$ preflight check
Current branch: feature-auth
Base branch: main

Fetching latest base branch...
Checking for conflicts...

✅ No conflicts detected!
Safe to push.
```

**Conflicts detected:**
```bash
$ preflight check
Current branch: feature-auth
Base branch: main

Fetching latest base branch...
Checking for conflicts...

❌ Conflicts detected!

Conflicting files:
  src/auth.rs
  src/main.rs

Resolve conflicts before pushing.
```

### Exit Codes

Preflight uses standard exit codes for scripting:

- `0` - Clean merge, no conflicts detected
- `1` - Conflicts detected
- `2` - Error (not a git repo, Git too old, etc.)

### Integration Example

```bash
# In your workflow
preflight check && git push origin feature-branch
# Only pushes if no conflicts detected
```

## Roadmap

- [x] **Phase 0:** Task 0 verification of `git merge-tree` behavior ✅
- [x] **Phase 1:** MVP - check current branch vs base branch ✅
- [ ] **Phase 2:** Git hook integration (`preflight install-hook`)
- [ ] **Phase 3:** GitHub PR awareness (check against open PRs)
- [ ] **Phase 4:** Better reporting (show hunks, colors, JSON output)
- [ ] **Phase 5:** Configuration file support (.preflight.toml)
- [ ] **Phase 6:** Distribution (crates.io, binaries, AUR)

## Technical Details

### Git merge-tree Command

Preflight uses Git's built-in `merge-tree` command in `--write-tree` mode (available in Git 2.38+):

```bash
git merge-tree --write-tree <base-branch> <current-branch>
```

**How it works:**
- Computes a three-way merge internally
- Returns a tree object representing the merge result
- **Never touches:** working directory, index, HEAD, or any refs
- **Output format:**
  - Clean merge: Single line with tree SHA
  - Conflicts: Tree SHA + conflict messages listing affected files

**Example outputs:**

Clean merge:
```
abc123def456789...  
```

Conflicted merge:
```
abc123def456789...

Auto-merging src/main.rs
CONFLICT (content): Merge conflict in src/main.rs
Auto-merging src/lib.rs
CONFLICT (content): Merge conflict in src/lib.rs
```

This approach is:
- ✅ **Safe:** No repository mutation
- ✅ **Fast:** No working tree operations
- ✅ **Accurate:** Uses Git's actual merge algorithm
- ✅ **Reliable:** Built into Git itself

### Why Not Use `git merge --no-commit`?

`git merge --no-commit` still modifies your index and can leave your repository in a merging state. We avoid this entirely by using the lower-level plumbing command.

## Non-Goals

- This does NOT resolve conflicts automatically
- This does NOT replace `git merge` or `git rebase`
- This is NOT a GUI
- This is NOT a general Git workflow manager

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

This is planned for Phase 3! The tool will be able to check your branch against all open PRs, not just the base branch.

### Can I use this in CI?

Absolutely. Use the exit codes (0 = clean, 1 = conflicts) in your CI scripts. JSON output mode is planned for Phase 4.

### Is it fast enough for a git hook?

Yes! Preflight is designed to run in under 2 seconds for typical repositories, making it suitable for pre-push hooks. Phase 2 will add automatic hook installation.

### What about Windows support?

The Rust code is cross-platform. Windows support should work but hasn't been extensively tested yet. Contributions welcome!

## Development

### Task 0 Verification ✅

**Status: Complete**

We've verified `git merge-tree --write-tree` behavior and implemented the parser accordingly. See `TASK_0_RESULTS.md` for full details.

**Key findings:**
- Clean merges output a single line (tree SHA)
- Conflicted merges output multiple lines with "CONFLICT" markers
- File paths are extracted from conflict messages
- Exit codes: 0 for clean, 1 for conflicts

### Building from Source

See `TASK_0_VERIFICATION.md` for critical first step before implementing the parser.

```bash
cargo build
cargo test
cargo run -- check
```

## License

MIT OR Apache-2.0

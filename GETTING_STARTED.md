# Getting Started with Preflight Development

Welcome! This guide will help you start working on Preflight.

## Prerequisites

- **Rust** (1.70 or later): Install from https://rustup.rs/
- **Git** (version TBD, likely 2.38+)
- A terminal and text editor

## Quick Start

### 1. Verify the Project Compiles

```bash
cargo build
```

This should compile successfully, though the tool won't work correctly yet (see Task 0).

### 2. Complete Task 0 (CRITICAL)

**This is the most important step.** The tool cannot work without it.

```bash
./scripts/verify_merge_tree.sh
```

This script will:
- Create a test Git repository with known conflicts
- Test `git merge-tree` commands
- Show you the exact output format
- Save outputs to `/tmp/merge-tree-*.txt`

**Your job:** Study the output and answer these questions:

1. What Git version is installed?
2. Does `git merge-tree --write-tree` work? (modern syntax)
3. If not, does the old `git merge-tree <base> <branch1> <branch2>` work?
4. What does the output look like for a **conflicted** merge?
5. What does the output look like for a **clean** merge?
6. How are conflicting files listed?
7. Are conflict markers (`<<<<<<<`) included in the output?

**Document your findings** in `TASK_0_VERIFICATION.md` or a new file.

### 3. Update the Parser

Once you understand the output format, update `src/git.rs`:

**Function to fix:** `parse_merge_tree_output()`

Current state: Placeholder with TODO comments
Needs: Real parsing based on actual format

**Function to fix:** `check_merge_tree()`

Current state: Uses `--write-tree` flag (modern syntax)
Needs: Correct flags for the target Git version

### 4. Test Manually

Create a real test case:

```bash
# Create a test repo with conflicts
mkdir /tmp/preflight-manual-test
cd /tmp/preflight-manual-test
git init
git config user.email "you@example.com"
git config user.name "Your Name"

# Set up conflict scenario
echo "original" > file.txt
git add file.txt
git commit -m "base"

git checkout -b feature
echo "feature change" > file.txt
git commit -am "feature"

git checkout master
echo "master change" > file.txt
git commit -am "master"

# Now test preflight from your feature branch
git checkout feature
/path/to/preflight/target/debug/preflight check --base master
```

Expected behavior:
- Should detect the conflict on `file.txt`
- Should exit with code 1
- Should NOT modify your working directory

### 5. Verify Safety

After running preflight, check:

```bash
git status  # Should be clean
git diff    # Should be empty
```

The working directory must never be modified by preflight.

### 6. Run Tests (once implemented)

```bash
cargo test
```

### 7. Read the Code

Start with:
1. `src/main.rs` - CLI entry point
2. `src/conflict_checker.rs` - Main logic flow
3. `src/git.rs` - Git operations (this is where Task 0 fixes go)

## Common Issues

### "Git version not supported"
Update Git or implement fallback to older merge-tree syntax.

### "Not in a git repository"
Run preflight from inside a Git repository.

### "Could not detect base branch"
Use `--base` flag: `preflight check --base main`

### Output looks wrong
This means Task 0 parsing is incorrect. Re-verify the output format.

## Development Workflow

1. Make changes to Rust code
2. `cargo check` - Quick syntax check
3. `cargo build` - Full build
4. `cargo run -- check --base main` - Test the CLI
5. `cargo test` - Run tests (once added)
6. `cargo clippy` - Lint for issues

## What to Build Next

After Task 0 is complete and Phase 1 MVP works:

- Add tests (see TODO.md)
- Improve error messages
- Add `--json` output mode
- Move on to Phase 2 (hook installation)

See `TODO.md` for the full checklist.

## Getting Help

- Read the PRD (the document you were given)
- Check `DEVELOPMENT.md` for architecture details
- Review `TODO.md` for what's done and what's next
- Look for TODO comments in the code

## Key Principles

1. **Never mutate the repository** - Working directory, index, and refs must remain untouched
2. **Verify, don't assume** - Don't guess Git output format, test it
3. **Fail safely** - Better to refuse to run than to give wrong results
4. **Keep it fast** - This will run on every push

Good luck! Start with Task 0 and work through TODO.md.

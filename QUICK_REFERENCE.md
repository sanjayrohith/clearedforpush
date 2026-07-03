# Preflight - Quick Reference

## Essential Commands

```bash
# Build
cargo build

# Run
cargo run -- check
cargo run -- check --base develop

# Test (once implemented)
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## File Structure

```
preflight/
├── src/
│   ├── main.rs              - CLI entry (clap)
│   ├── git.rs               - Git operations ⚠️ NEEDS TASK 0
│   └── conflict_checker.rs  - Main logic
├── scripts/
│   └── verify_merge_tree.sh - Task 0 verification script
├── README.md                - User docs
├── GETTING_STARTED.md       - Dev onboarding
├── DEVELOPMENT.md           - Architecture
├── TODO.md                  - Task checklist
├── PROJECT_STATUS.md        - Current state
└── TASK_0_VERIFICATION.md   - Critical first step ⚠️
```

## Critical Paths

### Task 0 (MUST DO FIRST)
```bash
./scripts/verify_merge_tree.sh
# Study output in /tmp/merge-tree-*.txt
# Document findings
# Update src/git.rs
```

### Functions Blocked on Task 0
- `src/git.rs::parse_merge_tree_output()` - Parser stub
- `src/git.rs::check_merge_tree()` - May need flag changes

### Testing Checklist
- [ ] Clean merge detected correctly
- [ ] Conflict detected correctly
- [ ] Files listed correctly
- [ ] Exit codes correct (0=clean, 1=conflict)
- [ ] Working directory unchanged (git status)
- [ ] Index unchanged (git diff --cached)
- [ ] Performance < 2 seconds

## Exit Codes

- `0` - No conflicts, safe to merge
- `1` - Conflicts detected
- `2` - Error (not a repo, Git too old, etc.)

## Key Constraints

❌ **NEVER** modify:
- Working directory
- Index (staging area)
- HEAD or any branch
- Any refs
- Stashes

✅ **OK** to:
- Read git objects
- Fetch from remote
- Run merge-tree (doesn't touch working dir)
- Write to stdout/stderr

## Common Workflow

```bash
# On feature branch
git checkout feature-branch

# Before pushing
preflight check

# If clean (exit 0):
git push origin feature-branch

# If conflicts (exit 1):
# Resolve conflicts first
git fetch origin main
git rebase origin/main
# Then try again
preflight check
```

## Development Phases

1. ⚠️ **Phase 0** - Task 0 verification - **START HERE**
2. 🟡 **Phase 1** - MVP (current branch vs base)
3. ⏰ **Phase 2** - Hook installation
4. ⏰ **Phase 3** - GitHub PR checking
5. ⏰ **Phase 4** - Better reporting
6. ⏰ **Phase 5** - Configuration files
7. ⏰ **Phase 6** - Distribution

## Git Commands Used

```bash
# Current branch
git rev-parse --abbrev-ref HEAD

# Remote default branch
git symbolic-ref refs/remotes/origin/HEAD

# Fetch (read-only)
git fetch origin <branch>

# Merge-tree (modern, Git >= 2.38)
git merge-tree --write-tree <branch1> <branch2>

# Merge-tree (old syntax)
git merge-tree <base> <branch1> <branch2>
```

## When Things Break

**"unknown option --write-tree"**
→ Git too old, need fallback or require newer Git

**"Not in a git repository"**
→ Run from inside a Git repo

**"Could not detect base branch"**
→ Use `--base main` explicitly

**Wrong conflicts detected**
→ Task 0 parser is wrong, re-verify output format

**Working directory modified**
→ CRITICAL BUG, fix immediately

## Getting Unstuck

1. Read PROJECT_STATUS.md (current state)
2. Read GETTING_STARTED.md (setup guide)
3. Check TODO.md (what's next)
4. Review TASK_0_VERIFICATION.md (if parser issues)
5. Read the original PRD (full requirements)

## One-Liner Summary

**"Preflight tells you if your branch will conflict with main before you push, using git merge-tree, without touching your working directory."**

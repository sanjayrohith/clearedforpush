# Preflight - Project Handoff Document

**Date:** July 3, 2026  
**Status:** Phase 1 MVP scaffold complete, awaiting Task 0 verification

## Executive Summary

Preflight is a CLI tool that predicts merge conflicts before pushing code. It uses Git's native `merge-tree` plumbing to check if your current branch will conflict with a base branch, without modifying your working directory.

The project is **scaffolded and ready for implementation**, but the core conflict detection logic is intentionally blocked pending verification of `git merge-tree` behavior (Task 0 from the PRD).

## What's Been Built

### ✅ Complete

**Infrastructure:**
- Rust project with Cargo configuration
- Module structure (main, git, conflict_checker)
- Dependencies configured (clap, anyhow, colored)
- Dual MIT/Apache-2.0 licensing
- Git repository initialized
- .gitignore configured

**Core Code:**
- CLI skeleton with `check` subcommand (clap)
- Current branch detection (git rev-parse)
- Base branch auto-detection (origin/HEAD, then main/master)
- Remote branch fetching (read-only)
- Exit code handling (0=clean, 1=conflicts, 2=error)
- Colored terminal output
- Error propagation with context

**Documentation (9 files):**
1. **README.md** - User-facing overview and roadmap
2. **START_HERE.md** - Entry point for anyone picking up the project
3. **GETTING_STARTED.md** - Developer onboarding guide
4. **TASK_0_VERIFICATION.md** - The critical verification step
5. **TODO.md** - Complete implementation checklist
6. **DEVELOPMENT.md** - Technical design and constraints
7. **ARCHITECTURE.md** - System architecture and data flow
8. **PROJECT_STATUS.md** - Current state and progress
9. **QUICK_REFERENCE.md** - Command and concept cheat sheet
10. **HANDOFF.md** - This document

**Tooling:**
- Task 0 verification script (`scripts/verify_merge_tree.sh`)

### ⚠️ Intentionally Incomplete (Blocked on Task 0)

**In `src/git.rs`:**
- `check_merge_tree()` - Uses placeholder command syntax
- `parse_merge_tree_output()` - Contains stub logic with TODO comments

**Why blocked:** The PRD explicitly requires verifying the actual `git merge-tree` output format before implementing the parser. Different Git versions have different syntax and output. Guessing would be wrong.

## Critical Path to MVP

```
1. Complete Task 0 (MUST BE FIRST)
   └─ Run scripts/verify_merge_tree.sh
   └─ Document actual output format
   └─ Update src/git.rs functions
            │
            ▼
2. Manual Testing
   └─ Test with real conflicts
   └─ Test clean merges
   └─ Verify no working directory mutation
            │
            ▼
3. Automated Tests
   └─ Unit tests for parser
   └─ Integration tests with test repos
   └─ Safety tests
            │
            ▼
4. Phase 1 Complete ✅
```

## Files and Their Purpose

### Code
| File | Purpose | Status |
|------|---------|--------|
| `src/main.rs` | CLI entry, arg parsing | ✅ Complete |
| `src/conflict_checker.rs` | Orchestration, output | ✅ Complete |
| `src/git.rs` | Git operations | ⚠️ Parser blocked on Task 0 |

### Documentation
| File | Audience | Read When |
|------|----------|-----------|
| `START_HERE.md` | Anyone new | First |
| `README.md` | Users & contributors | Understanding the problem |
| `GETTING_STARTED.md` | Developers | Starting development |
| `TASK_0_VERIFICATION.md` | Implementer | Before coding parser |
| `TODO.md` | Developers | Planning work |
| `DEVELOPMENT.md` | Developers | Understanding design |
| `ARCHITECTURE.md` | Developers | Understanding structure |
| `PROJECT_STATUS.md` | Anyone | Checking progress |
| `QUICK_REFERENCE.md` | Developers | Quick lookup |
| `HANDOFF.md` | Project maintainer | This document |

### Scripts
| File | Purpose |
|------|---------|
| `scripts/verify_merge_tree.sh` | Task 0 verification - creates test repo and runs git merge-tree |

## Key Technical Decisions

| Decision | Rationale |
|----------|-----------|
| Language: Rust | Fast startup (important for hooks), static binary, memory safety |
| No git2 crate | Avoid reimplementing merge logic; use Git's own implementation |
| Shell out to git | Simpler, more reliable than parsing git internals |
| Task 0 verification | PRD requirement: don't assume Git output format |
| Colored output | Better UX, degrades gracefully |
| anyhow for errors | Simple, good for applications |
| clap for CLI | Standard, excellent derive API |

## Hard Constraints (from PRD)

These are **non-negotiable requirements:**

1. **Never mutate repository state**
   - Working directory unchanged
   - Index (staging area) unchanged
   - No branch or ref modifications
   - No stash modifications

2. **Verify, don't assume**
   - Don't guess git merge-tree output format
   - Complete Task 0 before implementing parser
   - Test against actual Git version in use

3. **Performance**
   - Must be fast enough for pre-push hooks
   - Target: < 2 seconds for typical repos

4. **Exit codes**
   - 0 = clean, safe to merge
   - 1 = conflicts detected
   - 2 = error (not a repo, git too old, etc.)

## Known Issues & Open Questions

**From PRD Section 8:**

1. ❓ Exact git merge-tree command syntax → **Resolve in Task 0**
2. ❓ Minimum Git version → **Resolve in Task 0**
3. ⚠️ Base branch detection edge cases → **Needs real-world testing**
4. ⏰ GitHub authentication strategy → **Defer to Phase 3**
5. ⏰ Tool name availability on crates.io → **Check before Phase 6**

**Technical Unknowns:**

- Does the modern `--write-tree` syntax work on the target system?
- Should we support older Git versions with fallback syntax?
- What error do we show if Git is too old?
- Windows compatibility (commands assume bash/Linux)

## What to Do Next

**Immediate (Critical Path):**

1. **Run Task 0 verification**
   ```bash
   ./scripts/verify_merge_tree.sh
   ```
   
2. **Study the output files** in `/tmp/merge-tree-*.txt`

3. **Update `src/git.rs`:**
   - Fix `check_merge_tree()` with correct command
   - Implement `parse_merge_tree_output()` based on real format

4. **Test manually:**
   - Create repo with conflict
   - Run `cargo run -- check`
   - Verify it detects the conflict
   - Verify `git status` is clean after

5. **Add tests** (see TODO.md)

**After Phase 1 MVP Works:**

See `TODO.md` for Phases 2-6:
- Phase 2: Git hook integration
- Phase 3: GitHub PR awareness
- Phase 4: Better reporting (hunks, JSON)
- Phase 5: Configuration files
- Phase 6: Distribution (crates.io, binaries)

## Testing Checklist (Before Phase 1 Complete)

- [ ] Detects conflicts correctly
- [ ] Detects clean merges correctly
- [ ] Lists conflicting files accurately
- [ ] Correct exit codes (0, 1, 2)
- [ ] No working directory mutation (git status clean)
- [ ] No index mutation (git diff --cached empty)
- [ ] Handles "already on base branch" case
- [ ] Handles "base branch doesn't exist" case
- [ ] Handles "not in git repo" case
- [ ] Handles detached HEAD gracefully
- [ ] Performance < 2 seconds on typical repo

## How to Verify Safety (Critical)

After running preflight, these MUST be true:

```bash
# Before
git status          # Note the state
git diff            # Should be empty
git diff --cached   # Should be empty

# Run preflight
cargo run -- check

# After
git status          # MUST match "before"
git diff            # MUST still be empty
git diff --cached   # MUST still be empty
```

If any file changes, refs change, or working directory is modified: **CRITICAL BUG**.

## Dependencies

Current (`Cargo.toml`):
```toml
clap = "4.5"        # CLI parsing
anyhow = "1.0"      # Error handling
colored = "2.1"     # Terminal colors
```

Future phases will add:
- Phase 3: `reqwest`, `serde_json` (GitHub API)
- Phase 5: `toml` (config parsing)

## Building and Running

```bash
# Build
cargo build

# Run (development)
cargo run -- check
cargo run -- check --base develop

# Build optimized
cargo build --release
./target/release/preflight check

# Test (once tests exist)
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

## Git Workflow Integration (Future)

Planned usage (Phase 2):

```bash
# Install hook
preflight install-hook

# Hook runs automatically on push
git push origin feature-branch
# → Preflight runs
# → Blocks push if conflicts detected
# → Or allows push if clean
```

## Contact & Ownership

- **License:** MIT OR Apache-2.0
- **Repository:** Not yet published
- **Maintainer:** TBD
- **Status:** Pre-release, Phase 1 in progress

## For AI Assistants Continuing This Work

**Read these files in order:**
1. This file (HANDOFF.md)
2. `TASK_0_VERIFICATION.md` - The critical first step
3. The original PRD (if available)
4. `DEVELOPMENT.md` - Technical details

**Critical instructions from PRD:**
- Do not assume git merge-tree output format
- Verify it empirically (Task 0)
- Never mutate working directory/index/refs
- All Section 8 items are open questions, not settled facts

**If you see a TODO comment in `src/git.rs`:**
- It's there intentionally
- Don't remove it until Task 0 is done
- The placeholder logic MUST be replaced with verified parsing

## Success Criteria

**Phase 1 MVP is complete when:**

- [ ] Task 0 verification documented
- [ ] Parser implemented based on real output
- [ ] Tests pass (conflict detection, clean merge, safety)
- [ ] Manual testing successful on real repos
- [ ] No working directory mutation verified
- [ ] Performance < 2 seconds
- [ ] Documentation updated with minimum Git version
- [ ] All Phase 1 items in TODO.md checked off

**The project is ready for users when:**

- [ ] Phase 1 complete
- [ ] Tested on multiple machines/Git versions
- [ ] README has installation instructions
- [ ] Published to crates.io
- [ ] Binaries available for download
- [ ] At least 3 real users have successfully used it

## Final Notes

This is a **well-scoped, achievable project**. The hard part isn't the architecture or the code structure—that's done. The hard part is getting Task 0 right. Once that's verified, implementation is straightforward.

The project follows good software engineering practices:
- ✅ Clear separation of concerns
- ✅ Comprehensive documentation
- ✅ Safety-first design
- ✅ Error handling with context
- ✅ Performance consideration
- ✅ Phased roadmap

**Don't skip Task 0.** Everything depends on it.

Good luck! 🚀

---

**Last updated:** Initial scaffold completion, July 3, 2026

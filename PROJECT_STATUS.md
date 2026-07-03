# Preflight - Project Status

**Last Updated:** Initial scaffold - July 3, 2026

## Current State: Phase 1 MVP - In Progress

### What's Done ✅

**Project Structure:**
- ✅ Rust project initialized with Cargo
- ✅ Dependencies configured (clap, anyhow, colored)
- ✅ Module structure created (main, git, conflict_checker)
- ✅ License files (MIT/Apache-2.0 dual)
- ✅ .gitignore configured

**Core Implementation:**
- ✅ CLI argument parsing skeleton (clap)
- ✅ `check` subcommand structure
- ✅ Current branch detection (`git rev-parse`)
- ✅ Base branch auto-detection (checks origin/HEAD, then main/master fallback)
- ✅ Remote branch fetch (read-only)
- ✅ Basic conflict checker flow
- ✅ Exit code handling (0 = clean, 1 = conflicts)
- ✅ Colored terminal output

**Documentation:**
- ✅ README.md with problem statement and roadmap
- ✅ DEVELOPMENT.md with architecture and constraints
- ✅ TODO.md with complete implementation checklist
- ✅ TASK_0_VERIFICATION.md explaining the critical first step
- ✅ GETTING_STARTED.md for new developers
- ✅ This status document

**Tooling:**
- ✅ Verification script (`scripts/verify_merge_tree.sh`)

### What's Blocked ⚠️

**Critical Blocker: Task 0 Not Complete**

The following cannot be finished until `git merge-tree` output format is verified:

- ❌ `parse_merge_tree_output()` function in src/git.rs
- ❌ Correct `check_merge_tree()` command syntax
- ❌ Accurate conflict detection
- ❌ File path extraction from conflicts
- ❌ Any meaningful testing

**Why This Matters:**
The PRD explicitly requires that we don't assume the format. Git's merge-tree output differs between versions, and implementing the parser without verifying actual output would be guessing.

### What's Next 📋

**Immediate Next Steps (in order):**

1. **Run Task 0 Verification**
   - Execute `./scripts/verify_merge_tree.sh`
   - Study the output files in `/tmp/merge-tree-*.txt`
   - Document findings

2. **Implement Parser**
   - Update `src/git.rs::parse_merge_tree_output()`
   - Handle both clean and conflicted cases
   - Extract file paths correctly

3. **Test with Real Conflicts**
   - Create test repositories
   - Verify correct conflict detection
   - Verify clean merge detection
   - **CRITICAL:** Verify no working directory mutation

4. **Add Tests**
   - Unit tests for parser
   - Integration tests with manufactured conflicts
   - Safety tests (verify no mutation)

5. **Polish Phase 1**
   - Better error messages
   - Handle edge cases (detached HEAD, no remote, etc.)
   - Performance testing

6. **Phase 1 Release**
   - All Phase 1 items in TODO.md complete
   - Documented and tested
   - Ready for early users

### Known Issues / Decisions Needed

**From PRD Section 8 (Open Questions):**

1. ❓ **Exact git merge-tree syntax** - BLOCKED on Task 0
2. ❓ **Minimum Git version** - BLOCKED on Task 0
3. ⚠️ **Base branch detection** - Implemented but needs real-world testing
4. ⏰ **GitHub auth strategy** - Defer to Phase 3
5. ⏰ **Tool name availability** - Must check before publishing

**Technical Decisions:**

- ✅ Language: Rust chosen (fast startup, static binary)
- ✅ CLI library: clap with derive macros
- ✅ Error handling: anyhow::Result
- ✅ Colors: colored crate
- ❓ Minimum Git version: TBD after Task 0
- ❓ Support old merge-tree syntax: TBD after Task 0

### Non-Functional Requirements Status

| Requirement | Status | Notes |
|------------|--------|-------|
| No working directory mutation | ⚠️ Needs testing | Commands are read-only by design, must verify |
| Fast enough for hooks (< 2s) | ⏰ Not tested | Test after implementation complete |
| Correct conflict detection | ❌ Blocked on Task 0 | Parser not yet implemented |
| Safe hook installation | ⏰ Phase 2 | Not yet started |
| Portable (Linux/Mac/Win) | ⚠️ Partially | Rust is portable, git commands need testing on Windows |

### Phase Progress

- **Phase 0 (Task 0):** ❌ Not started - **START HERE**
- **Phase 1 (MVP):** 🟡 40% complete (skeleton done, logic blocked)
- **Phase 2 (Hooks):** ⏰ Not started
- **Phase 3 (PR awareness):** ⏰ Not started
- **Phase 4 (Better reporting):** ⏰ Not started
- **Phase 5 (Config):** ⏰ Not started
- **Phase 6 (Distribution):** ⏰ Not started

### How to Contribute

If you're picking up this project:

1. **Read these files in order:**
   - This file (PROJECT_STATUS.md) - You are here
   - GETTING_STARTED.md - Development setup
   - TASK_0_VERIFICATION.md - The critical first task
   - TODO.md - Full task list
   - The original PRD - Full requirements

2. **Complete Task 0 first** - Everything else depends on it

3. **Follow the constraints:**
   - Never mutate working directory/index/refs
   - Verify, don't assume Git output format
   - Keep it fast (< 2 seconds)
   - Exit codes matter (0 = clean, 1 = conflict, 2 = error)

4. **Test thoroughly:**
   - Real repos with manufactured conflicts
   - Edge cases (detached HEAD, no remote, etc.)
   - Safety (git status clean after running)

### Contact / Ownership

- **Status:** Open source project, no current maintainer
- **License:** MIT OR Apache-2.0
- **Repository:** TBD (not yet published)

---

**Bottom Line:** The project is well-scaffolded and ready for Task 0 verification. Once that's done, the path to a working MVP is straightforward. Don't skip Task 0!

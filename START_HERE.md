# 🚀 START HERE

Welcome to **Preflight** - the pre-push merge conflict predictor!

## What You Need to Know

This project is a **Phase 1 MVP in progress**. The skeleton is complete, but the core conflict detection logic is blocked on **Task 0**.

## Your Mission (Choose Your Path)

### Path A: Just Want to Understand the Project?
**Read these (5 min):**
1. `README.md` - What problem does this solve?
2. `PROJECT_STATUS.md` - Where are we now?
3. `QUICK_REFERENCE.md` - Quick facts and commands

### Path B: Want to Continue Development?
**Do these (in order):**
1. ✅ Read `README.md` - Understand the problem
2. ✅ Read `GETTING_STARTED.md` - Set up your environment  
3. ⚠️ **Complete Task 0** (see `TASK_0_VERIFICATION.md`)
   - Run `./scripts/verify_merge_tree.sh`
   - Study the output
   - Update `src/git.rs` with real parsing logic
4. ✅ Read `TODO.md` - See what's next
5. ✅ Build and test: `cargo build && cargo run -- check`

### Path C: Need Architecture Details?
**Read these (20 min):**
1. `ARCHITECTURE.md` - System design and data flow
2. `DEVELOPMENT.md` - Detailed technical design
3. Look at code in `src/` with the architecture in mind

## The One Thing You Must Do

**⚠️ TASK 0 - Verify git merge-tree behavior**

Everything else is blocked on this. The project won't work until you:
1. Run the verification script
2. Document the actual git merge-tree output format
3. Fix the parser in `src/git.rs`

See: `TASK_0_VERIFICATION.md`

## Project Status at a Glance

```
✅ Rust project structure
✅ CLI argument parsing
✅ Branch detection
✅ Remote fetching
✅ Comprehensive documentation
⚠️ BLOCKED: Conflict detection (Task 0 incomplete)
⏰ TODO: Testing, Phase 2-6 features
```

## Quick Commands

```bash
# Verify it compiles
cargo build

# Run Task 0 verification
./scripts/verify_merge_tree.sh

# Try it (won't work correctly yet)
cargo run -- check --base main

# Read the checklist
cat TODO.md
```

## Documentation Map

```
START_HERE.md         ← You are here
│
├─ For understanding:
│  ├─ README.md              (What is Preflight?)
│  ├─ PROJECT_STATUS.md      (Current state)
│  └─ QUICK_REFERENCE.md     (Cheat sheet)
│
├─ For development:
│  ├─ GETTING_STARTED.md     (Setup guide)
│  ├─ TASK_0_VERIFICATION.md (Critical first step ⚠️)
│  ├─ TODO.md                (Task checklist)
│  ├─ DEVELOPMENT.md         (Technical design)
│  └─ ARCHITECTURE.md        (System architecture)
│
└─ For reference:
   ├─ Cargo.toml             (Dependencies)
   ├─ src/main.rs            (CLI entry)
   ├─ src/git.rs             (Git operations ⚠️)
   └─ src/conflict_checker.rs (Main logic)
```

## Next Steps

1. **Decide your path** (A, B, or C above)
2. **If Path B:** Go straight to Task 0
3. **If stuck:** Read `GETTING_STARTED.md`

## TL;DR

**Preflight tells you if your branch conflicts with main BEFORE you push.**

**Current blocker: Task 0 (verify git merge-tree output format)**

**Action: Run `./scripts/verify_merge_tree.sh` and fix `src/git.rs`**

---

Good luck! 🚀

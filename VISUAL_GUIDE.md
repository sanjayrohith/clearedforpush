# Preflight Visual Guide

## The Problem

```
                CURRENT WORKFLOW (❌ Late Feedback)
                
Developer          Git Server            CI/PR Review
    │                   │                      │
    │   git push        │                      │
    │─────────────────> │                      │
    │                   │  Trigger CI          │
    │                   │─────────────────────>│
    │                   │                      │
    │                   │      ❌ CONFLICTS!   │
    │                   │<─────────────────────│
    │   Oh no! 😢       │                      │
    │<──────────────────│                      │
    │                                          │
    └─ Now must fix conflicts and force-push  │
       (Wasted time, broken flow)              │
```

```
                PREFLIGHT WORKFLOW (✅ Early Feedback)
                
Developer (local)      Git Server
    │                       │
    │  preflight check      │
    │  (locally)            │
    ├────────┐              │
    │        │              │
    │  ✅ Clean!            │
    │<───────┘              │
    │                       │
    │  git push             │
    │──────────────────────>│
    │                       │
    └─ Confident, no surprises!
```

## How It Works

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│  YOUR LOCAL REPOSITORY                                      │
│                                                             │
│  ┌─────────────┐            ┌──────────────┐              │
│  │   main      │            │   feature    │  ← You are   │
│  │   branch    │            │   branch     │    here      │
│  └──────┬──────┘            └──────┬───────┘              │
│         │                          │                       │
│         │                          │                       │
│         ●──────●                   ●                       │
│         commit1  commit2         commit3                   │
│                                                             │
│                     ┌──────────────────┐                   │
│                     │  preflight check │                   │
│                     └────────┬─────────┘                   │
│                              │                             │
│                              ▼                             │
│                  ┌───────────────────────┐                 │
│                  │ Git merge-tree        │                 │
│                  │ (read-only simulation)│                 │
│                  └───────────┬───────────┘                 │
│                              │                             │
│                 ┌────────────┴──────────┐                  │
│                 │                       │                  │
│                 ▼                       ▼                  │
│           ✅ Clean!              ❌ Conflicts!             │
│           (exit 0)               (exit 1)                  │
│           Push safely            Fix first                 │
│                                                             │
│  ⚠️ IMPORTANT: Working directory unchanged!                │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

## Architecture (Simplified)

```
┌──────────────────────────────────────────────────────────────┐
│  preflight check --base main                                 │
└─────────────┬────────────────────────────────────────────────┘
              │
              ▼
       ┌──────────────┐
       │   main.rs    │   Parse args with clap
       │   (CLI)      │   Dispatch to conflict_checker
       └──────┬───────┘
              │
              ▼
    ┌──────────────────┐
    │conflict_checker  │   Orchestrate the check:
    │   .rs            │   1. Get current branch
    └──────┬───────────┘   2. Get/detect base branch
           │               3. Fetch latest base
           │               4. Run merge-tree check
           │               5. Report results
           ▼
    ┌──────────────────┐
    │   git.rs         │   All Git operations:
    │                  │
    │  ┌─────────────┐ │   ┌──────────────────┐
    │  │ Get branch  │ │   │ Check merge-tree │ ← NEEDS TASK 0
    │  └─────────────┘ │   └──────────────────┘
    │                  │   ┌──────────────────┐
    │  ┌─────────────┐ │   │ Parse output     │ ← NEEDS TASK 0
    │  │ Fetch       │ │   └──────────────────┘
    │  └─────────────┘ │
    └──────────────────┘
              │
              ▼
       ┌─────────────┐
       │ Git (binary)│   git merge-tree
       └─────────────┘   git rev-parse
                         git fetch
```

## Data Flow

```
┌───────────────┐
│ Current Dir   │
└───────┬───────┘
        │
        ▼
    ┌────────────────────────────────────────┐
    │ Check: Are we in a git repository?     │
    └────────────┬───────────────────────────┘
                 │ ✓ Yes
                 ▼
    ┌────────────────────────────────────────┐
    │ Get current branch name                │
    │ (git rev-parse --abbrev-ref HEAD)      │
    └────────────┬───────────────────────────┘
                 │ "feature-123"
                 ▼
    ┌────────────────────────────────────────┐
    │ Detect base branch                     │
    │ (origin/HEAD or main/master)           │
    └────────────┬───────────────────────────┘
                 │ "main"
                 ▼
    ┌────────────────────────────────────────┐
    │ Fetch latest base branch               │
    │ (git fetch origin main)                │
    │ Read-only! Working dir untouched.      │
    └────────────┬───────────────────────────┘
                 │ Updated: refs/remotes/origin/main
                 ▼
    ┌────────────────────────────────────────┐
    │ Run merge-tree simulation              │
    │ (git merge-tree --write-tree ...)      │ ← TASK 0
    │ Compute what merge WOULD look like     │
    │ Still read-only!                       │
    └────────────┬───────────────────────────┘
                 │ Raw output (text)
                 ▼
    ┌────────────────────────────────────────┐
    │ Parse merge-tree output                │ ← TASK 0
    │ Extract: conflicts? which files?       │
    └────────────┬───────────────────────────┘
                 │
          ┌──────┴──────┐
          │             │
          ▼             ▼
    ┌─────────┐   ┌──────────┐
    │ Clean   │   │Conflicts │
    └────┬────┘   └────┬─────┘
         │             │
         ▼             ▼
    ┌─────────────────────────┐
    │ Print colored report    │
    │ ✅ Safe to push         │
    │ OR                      │
    │ ❌ Conflicts in:        │
    │    - src/file.rs        │
    └────┬────────────────────┘
         │
         ▼
    ┌─────────────────────────┐
    │ Exit with code:         │
    │  0 = clean              │
    │  1 = conflicts          │
    │  2 = error              │
    └─────────────────────────┘
```

## Task 0 Workflow

```
┌────────────────────────────────────────────────────────┐
│ TASK 0: Verify git merge-tree Output Format           │
└───────────────────┬────────────────────────────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ Run verification script:     │
     │ ./scripts/verify_merge_tree.sh│
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ Script creates test repo:    │
     │                              │
     │   main: "line 1"             │
     │     ├─ branch-a: "MODIFIED A"│
     │     └─ branch-b: "MODIFIED B"│  ← Conflict!
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ Run: git merge-tree ...      │
     │ Save output to:              │
     │   /tmp/merge-tree-output.txt │
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ YOU: Study the output!       │
     │                              │
     │ Questions to answer:         │
     │ • Exit code?                 │
     │ • Output format?             │
     │ • How are conflicts shown?   │
     │ • How are files listed?      │
     │ • Conflict markers present?  │
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ Update src/git.rs:           │
     │ • check_merge_tree()         │
     │ • parse_merge_tree_output()  │
     └──────────────┬───────────────┘
                    │
                    ▼
     ┌──────────────────────────────┐
     │ Test manually                │
     │ Verify it works!             │
     └──────────────────────────────┘
```

## Project Roadmap

```
PHASES
───────

Phase 0 ━━━━━━━━━━━━━━━━━━━┓
│                          ┃  ← YOU ARE HERE (Task 0)
│  Verify merge-tree       ┃     Complete this first!
│  behavior                ┃
└──────────────────────────┚

Phase 1 (MVP) ━━━━━━━━━━━━┓
│                          ┃
│  Check current branch    ┃  Scaffold done ✅
│  vs base branch          ┃  Parser blocked on Task 0 ⚠️
│                          ┃
└──────────────────────────┚

Phase 2 ━━━━━━━━━━━━━━━━━━┓
│                          ┃
│  Git hook integration    ┃  Not started
│  (auto-run on push)      ┃
│                          ┃
└──────────────────────────┚

Phase 3 ━━━━━━━━━━━━━━━━━━┓
│                          ┃
│  Check vs open PRs       ┃  Not started
│  (GitHub awareness)      ┃
│                          ┃
└──────────────────────────┚

Phase 4 ━━━━━━━━━━━━━━━━━━┓
│                          ┃
│  Better reporting        ┃  Not started
│  (hunks, JSON)           ┃
│                          ┃
└──────────────────────────┚

Phase 5 ━━━━━━━━━━━━━━━━━━┓
│                          ┃
│  Config file support     ┃  Not started
│                          ┃
└──────────────────────────┚

Phase 6 ━━━━━━━━━━━━━━━━━━┓
│                          ┃
│  Distribution            ┃  Not started
│  (crates.io, binaries)   ┃
│                          ┃
└──────────────────────────┚
```

## File Dependency Graph

```
User wants to start working
         │
         ▼
   START_HERE.md ──────┐
         │             │
         ├────> README.md (What is this?)
         │
         ├────> PROJECT_STATUS.md (Where are we?)
         │             │
         ▼             ▼
   GETTING_STARTED.md  QUICK_REFERENCE.md
         │
         ▼
   TASK_0_VERIFICATION.md ← CRITICAL FIRST STEP
         │
         ▼
   Update src/git.rs
         │
         ▼
   TODO.md (What's next?)
         │
         ├────> DEVELOPMENT.md (Technical details)
         │
         └────> ARCHITECTURE.md (System design)
```

## Safety Guarantee Visualization

```
BEFORE Preflight:

Working Directory    Index (Staged)    HEAD    Branches
┌──────────────┐    ┌─────────────┐   ┌───┐   ┌────────┐
│ file1.rs     │    │ file2.rs    │   │ ● │   │ main   │
│ file2.rs     │    │ (staged)    │   └───┘   │ feature│
│ ...          │    └─────────────┘           └────────┘
└──────────────┘

                    ▼ Run preflight ▼

AFTER Preflight:

Working Directory    Index (Staged)    HEAD    Branches
┌──────────────┐    ┌─────────────┐   ┌───┐   ┌────────┐
│ file1.rs     │    │ file2.rs    │   │ ● │   │ main   │
│ file2.rs     │    │ (staged)    │   └───┘   │ feature│
│ ...          │    └─────────────┘           └────────┘
└──────────────┘

        ⚠️ MUST BE IDENTICAL ⚠️
        
        If ANYTHING changed → CRITICAL BUG
```

## Quick Command Reference Visual

```
┌─────────────────────────────────────────────────┐
│  DEVELOPMENT COMMANDS                           │
├─────────────────────────────────────────────────┤
│                                                 │
│  ▶ cargo build                                  │
│    Compile the project                          │
│                                                 │
│  ▶ cargo run -- check                           │
│    Run against current directory                │
│                                                 │
│  ▶ cargo run -- check --base develop           │
│    Check against 'develop' branch               │
│                                                 │
│  ▶ cargo test                                   │
│    Run tests (once implemented)                 │
│                                                 │
│  ▶ cargo clippy                                 │
│    Lint the code                                │
│                                                 │
│  ▶ ./scripts/verify_merge_tree.sh              │
│    ⚠️ TASK 0 - Run this first!                  │
│                                                 │
└─────────────────────────────────────────────────┘
```

## Success Path

```
        START
          │
          ▼
    Read Documentation
    (START_HERE.md)
          │
          ▼
    Complete Task 0 ←───── ⚠️ CRITICAL
          │
          ▼
    Fix Parser in git.rs
          │
          ▼
    Manual Testing
          │
          ▼
    Add Automated Tests
          │
          ▼
    Phase 1 Complete! ✅
          │
          ▼
    Phase 2: Hooks
          │
          ▼
    Phase 3: GitHub PRs
          │
          ▼
    Phase 4-6: Polish & Ship
          │
          ▼
    Users Love It! 🎉
```

---

**Remember:** This is a simple, focused tool. The hard part is Task 0. After that, it's straightforward implementation. Don't overthink it!

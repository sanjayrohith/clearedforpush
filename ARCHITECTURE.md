# Preflight Architecture

## High-Level Flow

```
User runs: preflight check
         │
         ▼
    ┌─────────┐
    │ main.rs │  Parse CLI args with clap
    └────┬────┘
         │
         ▼
┌──────────────────┐
│conflict_checker  │  Orchestrate the check
│  ::check()       │
└────┬─────────────┘
     │
     ├─────────────────────────────────┐
     │                                 │
     ▼                                 ▼
┌──────────┐                    ┌─────────────┐
│ git.rs   │                    │ git.rs      │
│ get_     │                    │ detect_     │
│ current_ │                    │ base_       │
│ branch() │                    │ branch()    │
└──────────┘                    └─────────────┘
     │                                 │
     └────────────┬────────────────────┘
                  │
                  ▼
         ┌────────────────┐
         │ git.rs         │
         │ fetch_remote_  │
         │ branch()       │
         └────────┬───────┘
                  │
                  ▼
         ┌────────────────┐
         │ git.rs         │  ⚠️ BLOCKED ON TASK 0
         │ check_merge_   │
         │ tree()         │
         └────────┬───────┘
                  │
                  ▼
         ┌────────────────┐
         │ git.rs         │  ⚠️ BLOCKED ON TASK 0
         │ parse_merge_   │
         │ tree_output()  │
         └────────┬───────┘
                  │
                  ▼
         ┌────────────────┐
         │ MergeResult    │
         │ {              │
         │   conflicts,   │
         │   files[]      │
         │ }              │
         └────────┬───────┘
                  │
                  ▼
      ┌──────────────────────┐
      │ conflict_checker     │
      │ Print results        │
      │ Set exit code        │
      └──────────────────────┘
```

## Module Responsibilities

### `main.rs`
- CLI argument parsing (clap)
- Dispatch to appropriate subcommand
- Currently: only `check` command

### `conflict_checker.rs`
- Orchestrates the conflict check flow
- Handles user-facing output (colored terminal)
- Sets process exit codes
- Error handling and reporting

### `git.rs`
- All Git operations
- Current branch detection
- Base branch detection and auto-discovery
- Remote fetching (read-only)
- **merge-tree execution** ⚠️ Needs Task 0
- **Output parsing** ⚠️ Needs Task 0
- Encapsulates all `std::process::Command` calls

## Data Flow

```
User Input
    │
    ├─ --base flag (optional)
    └─ Current working directory (implicit)
           │
           ▼
    Git Operations (read-only)
           │
           ├─ git rev-parse (current branch)
           ├─ git symbolic-ref (detect base)
           ├─ git fetch (update remote refs)
           └─ git merge-tree (compute result)
                   │
                   ▼
           Raw Git Output (stdout)
                   │
                   ▼
           Parse into MergeResult
                   │
                   ▼
           Pretty Print + Exit Code
```

## Safety Guarantees

All operations must preserve repository state:

```
Before preflight:          After preflight:
├─ Working Directory  ═══> ├─ Working Directory (UNCHANGED)
├─ Index (staged)     ═══> ├─ Index (UNCHANGED)
├─ HEAD               ═══> ├─ HEAD (UNCHANGED)
├─ Branches           ═══> ├─ Branches (UNCHANGED)
└─ Stash              ═══> └─ Stash (UNCHANGED)

Only changed:
└─ Remote refs (refs/remotes/origin/*) ← fetch only
```

## Git Operations Detail

### Read-Only Operations ✅
- `git rev-parse --abbrev-ref HEAD` - Get current branch
- `git symbolic-ref refs/remotes/origin/HEAD` - Get remote default
- `git rev-parse --verify refs/heads/<branch>` - Check branch exists
- `git fetch origin <branch>` - Update remote refs (doesn't touch working dir)
- `git merge-tree <args>` - Compute merge (doesn't touch working dir)

### Never Used ❌
- `git merge` - Would create merge commit
- `git rebase` - Would modify history
- `git checkout` - Would change HEAD
- `git reset` - Would modify index/working dir
- `git stash` - Would modify stash
- Any command with `--force`, `-f`, or mutation flags

## Error Handling Strategy

```rust
Result<T, anyhow::Error>
    │
    ├─ Context added at each layer
    │  .context("What failed and why")
    │
    └─ Bubbles up to main()
           │
           ▼
    conflict_checker handles gracefully
           │
           ├─ Print user-friendly message
           └─ Exit with code 2 (error)
```

## Future Architecture (Phase 2+)

```
Phase 2 - Hook Integration:
  └─ Add: hook_manager.rs
       ├─ install_hook()
       ├─ uninstall_hook()
       └─ chain_with_existing()

Phase 3 - GitHub PR Awareness:
  └─ Add: github.rs
       ├─ list_open_prs()
       ├─ check_pr_branch()
       └─ aggregate_results()

Phase 4 - Better Reporting:
  └─ Add: reporter.rs
       ├─ format_text()
       ├─ format_json()
       └─ format_hunks()

Phase 5 - Configuration:
  └─ Add: config.rs
       ├─ load_config()
       ├─ parse_toml()
       └─ merge_with_defaults()
```

## Performance Considerations

**Target: < 2 seconds for typical repos**

Breakdown:
- Branch detection: < 50ms
- Remote fetch: 500-1500ms (network bound)
- merge-tree: < 500ms (compute bound)
- Output parsing: < 10ms
- Pretty print: < 10ms

Optimization opportunities:
- `--no-fetch` flag to skip fetch (assume local refs are fresh)
- Cache fetch results for N seconds
- Parallel fetch + branch detection

## Testing Strategy

```
Unit Tests (per module)
    │
    ├─ git.rs::parse_merge_tree_output()
    │    └─ Test various output formats
    │
    └─ git.rs::detect_base_branch()
         └─ Test fallback logic

Integration Tests (end-to-end)
    │
    ├─ Create test repo with conflict
    ├─ Run preflight check
    ├─ Assert: exit code = 1
    ├─ Assert: files listed correctly
    └─ Assert: git status clean

Safety Tests (critical)
    │
    └─ After running preflight:
         ├─ git status is clean
         ├─ git diff is empty
         ├─ git diff --cached is empty
         └─ All branches unchanged
```

## Dependencies

```toml
clap = "4.5"          # CLI parsing
anyhow = "1.0"        # Error handling
colored = "2.1"       # Terminal colors
```

**No HTTP dependencies yet** (Phase 3 will add reqwest for GitHub API)

**No config dependencies yet** (Phase 5 will add toml for config parsing)

## Build Targets

```
Development:    cargo build
                └─ target/debug/preflight

Release:        cargo build --release
                └─ target/release/preflight (optimized, small)

Distribution:   Cross-compilation for:
                ├─ x86_64-unknown-linux-gnu
                ├─ x86_64-apple-darwin
                ├─ aarch64-apple-darwin
                └─ x86_64-pc-windows-msvc
```

## Code Organization Principles

1. **Separation of concerns:** CLI / Logic / Git operations
2. **Error propagation:** Use `Result` and `?`, add context
3. **No panics:** Fail gracefully with error messages
4. **Testability:** Pure functions where possible (parsing, logic)
5. **Clear naming:** No abbreviations (except common: PR, CLI, URL)
6. **Comments on non-obvious:** Especially Git quirks

## Key Design Decisions

| Decision | Rationale |
|----------|-----------|
| Rust | Fast startup, static binary, safety guarantees |
| clap | Standard CLI library, good UX, derive macros |
| anyhow | Simple error handling, good for applications |
| No git2 crate | Avoid reimplementing merge logic, shell out to git |
| Colored output | Better UX, degrades gracefully in non-TTY |
| Exit codes matter | Enables scripting and hook integration |

## Security Considerations

1. **Command injection:** Sanitize branch names before passing to shell
2. **Path traversal:** Validate file paths in output
3. **Token exposure:** Phase 3 must handle GitHub tokens safely
4. **Temp files:** None created currently, but if added: use secure temp dirs

## Documentation Map

```
For Users:
├─ README.md              - What, why, how to install
└─ --help output          - Usage examples

For Developers:
├─ GETTING_STARTED.md     - Onboarding
├─ ARCHITECTURE.md        - This file
├─ DEVELOPMENT.md         - Detailed design
├─ TODO.md                - Task list
├─ PROJECT_STATUS.md      - Current state
└─ QUICK_REFERENCE.md     - Cheat sheet

For Task 0:
└─ TASK_0_VERIFICATION.md - Critical first step
```

---

**Remember:** This is a simple tool with a focused purpose. Don't over-engineer. The complexity is in getting merge-tree right (Task 0), not in the architecture.

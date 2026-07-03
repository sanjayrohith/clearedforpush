# Development Guide

## Project Structure

```
preflight/
├── src/
│   ├── main.rs              # CLI entry point and argument parsing
│   ├── git.rs               # Git operations (merge-tree, fetch, branch detection)
│   └── conflict_checker.rs  # Main conflict checking logic
├── Cargo.toml               # Rust dependencies
├── README.md                # User-facing documentation
├── TASK_0_VERIFICATION.md   # Critical first step: verify git merge-tree
└── DEVELOPMENT.md           # This file
```

## Getting Started

1. **Complete Task 0 First!**
   
   Read `TASK_0_VERIFICATION.md` and complete the git merge-tree verification.
   This is REQUIRED before the tool will work correctly.

2. **Build the project:**
   ```bash
   cargo build
   ```

3. **Run the tool:**
   ```bash
   cargo run -- check
   # Or specify base branch:
   cargo run -- check --base develop
   ```

4. **Run tests:**
   ```bash
   cargo test
   ```

## Implementation Status

### Phase 1 (MVP) - In Progress

- [x] CLI skeleton with clap
- [x] Current branch detection
- [x] Base branch auto-detection
- [x] Read-only fetch of remote branch
- [ ] **BLOCKED:** Verify git merge-tree behavior (Task 0)
- [ ] **BLOCKED:** Parse merge-tree output correctly
- [ ] Exit codes (0 = clean, 1 = conflict, 2 = error)
- [ ] Testing with real repos
- [ ] Verify no mutation of working directory

### Next Steps

1. Complete Task 0 verification
2. Implement proper `parse_merge_tree_output()` in `src/git.rs`
3. Add tests with manufactured conflicts
4. Verify safety: ensure no working directory mutation
5. Test performance on various repo sizes

## Key Constraints

### Must Not Mutate Repository State

This is a HARD requirement. The tool must never:
- Modify the working directory
- Update the index (staging area)
- Change HEAD or any branch pointers
- Create or modify stashes
- Write to any refs

Every git command must be read-only or operate on separate refs.

### Performance

Must be fast enough for pre-push hooks. Target: < 2 seconds for typical repos.

## Testing Strategy

### Manual Testing Checklist

- [ ] Test with no conflicts
- [ ] Test with content conflicts
- [ ] Test with rename conflicts (if Git version supports)
- [ ] Test when on base branch already
- [ ] Test when base branch doesn't exist
- [ ] Test when not in a git repo
- [ ] Test with detached HEAD
- [ ] Verify no files modified (git status clean before/after)
- [ ] Verify no index changes (git diff --cached empty before/after)

### Automated Tests

TODO: Add unit tests after Task 0 completion:
- Parse clean merge output
- Parse conflicted merge output
- Branch detection logic
- Error handling

## Git Compatibility

**Minimum Git version: TBD** (pending Task 0 verification)

Likely requirement: Git >= 2.38 for modern `merge-tree --write-tree`

Add version check on startup:
```rust
fn check_git_version() -> Result<()> {
    // TODO: Implement after determining minimum version
}
```

## Code Style

- Use `anyhow::Result` for error handling
- Context on all errors: `.context("description")?`
- No panics in normal operation
- Clear variable names (no abbreviations unless obvious)
- Comments on non-obvious logic
- TODOs for Task 0 blockers are intentional - do not remove until verified

## Future Phases

See README.md for roadmap. Phase 2 and beyond should only start after Phase 1 is fully working and tested.

# TODO: Implementation Checklist

## ⚠️ CRITICAL - Must Complete First

### Task 0: Git merge-tree Verification
- [ ] Run the verification steps in `TASK_0_VERIFICATION.md`
- [ ] Document the exact `git merge-tree` command that works
- [ ] Document the exact output format for clean merges
- [ ] Document the exact output format for conflicted merges
- [ ] Determine minimum supported Git version
- [ ] Record exit codes for success vs. conflict
- [ ] Test performance on a real repo

**Nothing below can be completed until Task 0 is done.**

---

## Phase 1: MVP Implementation

### Git Operations (`src/git.rs`)
- [x] Implement `get_current_branch()`
- [x] Implement `detect_base_branch()`
- [x] Implement `fetch_remote_branch()`
- [ ] **Fix `check_merge_tree()`** based on Task 0 results
- [ ] **Fix `parse_merge_tree_output()`** based on Task 0 results
- [ ] Add Git version detection
- [ ] Add minimum version check with helpful error message
- [ ] Handle edge case: detached HEAD
- [ ] Handle edge case: no remote configured
- [ ] Handle edge case: remote branch doesn't exist

### Conflict Checker (`src/conflict_checker.rs`)
- [x] Basic flow implemented
- [ ] Add better error messages
- [ ] Handle Ctrl+C gracefully
- [ ] Add verbose/quiet modes
- [ ] Add dry-run mode for testing

### Main CLI (`src/main.rs`)
- [x] Basic `check` command
- [ ] Add `--help` documentation
- [ ] Add `--version` flag
- [ ] Add `--verbose` flag
- [ ] Add `--quiet` flag
- [ ] Add `--json` output mode (for CI)

### Testing
- [ ] Create test repository with known conflicts
- [ ] Test: no conflicts case
- [ ] Test: simple content conflict
- [ ] Test: multiple conflicted files
- [ ] Test: already on base branch
- [ ] Test: base branch doesn't exist
- [ ] Test: not in a git repository
- [ ] Test: detached HEAD state
- [ ] Test: no remote configured
- [ ] **CRITICAL:** Verify working directory never modified
- [ ] **CRITICAL:** Verify index never modified
- [ ] **CRITICAL:** Verify no ref changes

### Documentation
- [x] README.md
- [x] DEVELOPMENT.md
- [ ] Add usage examples to README
- [ ] Add screenshots/demo GIF
- [ ] Document exit codes
- [ ] Add FAQ section

### Polish
- [ ] Better error messages (show what command failed)
- [ ] Colored output (respect NO_COLOR env var)
- [ ] Progress indicators for slow operations
- [ ] Handle narrow terminals gracefully

---

## Phase 2: Git Hook Integration

- [ ] Implement `install-hook` subcommand
- [ ] Detect existing pre-push hook
- [ ] Warn before overwriting hooks
- [ ] Support hook chaining
- [ ] Implement `uninstall-hook` subcommand
- [ ] Add `--force` flag for hook installation
- [ ] Test hook installation safety
- [ ] Document hook installation in README

---

## Phase 3: GitHub PR Awareness

- [ ] Detect if `gh` CLI is installed
- [ ] Alternative: direct GitHub API with token
- [ ] List open PRs targeting same base
- [ ] Check conflicts against each PR branch
- [ ] Multi-branch conflict report
- [ ] Handle rate limiting
- [ ] Handle authentication errors
- [ ] Add `--skip-prs` flag

---

## Phase 4: Better Reporting

- [ ] Show conflicting hunks, not just files
- [ ] Syntax highlighting for diff output
- [ ] `--json` mode with stable schema
- [ ] Document JSON schema
- [ ] Add `--format` flag (text/json/compact)
- [ ] Show merge-base commit info
- [ ] Show ahead/behind counts

---

## Phase 5: Configuration

- [ ] Design config file format (TOML recommended)
- [ ] Implement config file parser
- [ ] Config: base branch override
- [ ] Config: ignored paths/patterns
- [ ] Config: PR checking default
- [ ] Config: GitHub token location
- [ ] Config: output format preference
- [ ] Search for config in `.preflight.toml`
- [ ] Document all config options

---

## Phase 6: Distribution

- [ ] Verify name "preflight" is available on crates.io
- [ ] Publish v0.1.0 to crates.io (Phase 1 complete)
- [ ] Set up CI for automated builds
- [ ] Build binaries for Linux (x86_64)
- [ ] Build binaries for macOS (x86_64, arm64)
- [ ] Build binaries for Windows (x86_64)
- [ ] Create GitHub releases with binaries
- [ ] Create AUR package (PKGBUILD)
- [ ] Submit to AUR
- [ ] Add installation instructions for each platform
- [ ] Set up changelog automation

---

## Open Questions (from PRD Section 8)

These need decisions during implementation:

1. **Exact git merge-tree command** → Resolved by Task 0
2. **Minimum Git version** → Resolved by Task 0
3. **Base branch auto-detection strategy** → Partially implemented, needs testing
4. **GitHub authentication approach** → Decide in Phase 3
5. **Tool name availability** → Check before Phase 6 publishing

---

## Code Quality

- [ ] Add rustfmt configuration
- [ ] Add clippy configuration
- [ ] Run `cargo clippy` and fix warnings
- [ ] Add CI for linting
- [ ] Add CI for tests
- [ ] Add code coverage reporting
- [ ] Document public APIs
- [ ] Add examples in doc comments

---

## Performance

- [ ] Profile on large repos
- [ ] Optimize if > 2 seconds
- [ ] Consider caching remote fetch results
- [ ] Add `--no-fetch` flag for speed

---

## Security

- [ ] Audit command injection risks
- [ ] Sanitize branch names in shell commands
- [ ] Safe handling of git output
- [ ] Review token storage for Phase 3
- [ ] Add security policy (SECURITY.md)

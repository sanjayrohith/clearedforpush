# 📊 Statistics Display Feature

## Overview

The `--stats` flag provides detailed information about your branch status, helping you understand the complexity of your merge before you push.

## Usage

```bash
preflight check --stats
```

## What It Shows

### 1. Ahead/Behind Counts
```
↑ 5 ahead  ↓ 2 behind
```

**Meaning:**
- **Ahead:** Your branch has 5 commits that main doesn't have
- **Behind:** Main has 2 commits that your branch doesn't have

**Why it matters:**
- High "behind" count suggests you should rebase/merge first
- Reduces chance of conflicts
- Keeps your branch up to date

### 2. Files Changed
```
📝 12 files changed
```

**Meaning:**
- 12 files are different between your branch and main

**Why it matters:**
- More files = higher chance of conflicts
- Helps estimate merge complexity
- Indicates scope of changes

### 3. Insertions/Deletions
```
± +234 -56
```

**Meaning:**
- 234 lines added
- 56 lines removed

**Why it matters:**
- Large changes = more potential for conflicts
- Shows magnitude of your modifications
- Useful for code review preparation

### 4. Merge Base
```
🔗 Merge base: abc123d
```

**Meaning:**
- The common ancestor commit (where branches diverged)
- Shows commit SHA (short form)

**Why it matters:**
- Old merge base = branches have diverged significantly
- Helps understand branch history
- Useful for debugging merge issues

## Example Output

### Clean Merge with Stats
```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-auth
        Base Branch: main

  ⚡ Gathering statistics...

  📊 Branch Statistics
  ──────────────────────────────────────────────────────────
  ↑ 5 ahead  ↓ 2 behind
  📝 12 files changed
  ± +234 -56
  🔗 Merge base: abc123d
  ──────────────────────────────────────────────────────────

  ⚡ Simulating merge...

╭──────────────────────────────────────────────────────────╮
│  ✅ CLEAR FOR TAKEOFF                                    │
│                                                          │
│  No conflicts detected. Safe to push!                   │
╰──────────────────────────────────────────────────────────╯

  ✈️  feature-auth → origin/main
```

### Conflicts with Stats
```
╔══════════════════════════════════════════════════════════╗
║                      ✈️  PREFLIGHT                       ║
║              Pre-push Merge Conflict Predictor          ║
╚══════════════════════════════════════════════════════════╝

     Current Branch: feature-refactor
        Base Branch: main

  ⚡ Gathering statistics...

  📊 Branch Statistics
  ──────────────────────────────────────────────────────────
  ↑ 15 ahead  ↓ 8 behind
  📝 23 files changed
  ± +567 -123
  🔗 Merge base: xyz789a
  ──────────────────────────────────────────────────────────

  ⚡ Simulating merge...

╭──────────────────────────────────────────────────────────╮
│  ⚠️  CONFLICT ALERT - HOLD FOR CLEARANCE                │
│                                                          │
│  Merge conflicts detected. Resolve before pushing.      │
╰──────────────────────────────────────────────────────────╯

  Conflicting files:
    ✗ src/auth.rs
    ✗ src/main.rs

  💡 Tip: Rebase or merge to resolve conflicts
```

## When to Use Stats

### ✅ Good Use Cases

**1. Before pushing a large feature:**
```bash
preflight check --stats
# See: "↓ 10 behind" → Should rebase first!
git fetch origin main
git rebase origin/main
preflight check --stats
# Now: "↓ 0 behind" ✅
git push
```

**2. Understanding merge complexity:**
```bash
preflight check --stats
# See: "+500 -200" → Large changes, review carefully
```

**3. Code review preparation:**
```bash
preflight check --stats
# Include stats in PR description
```

**4. Screenshots and demos:**
```bash
preflight check --stats
# Beautiful output for documentation
```

### ❌ When to Skip Stats

**1. Quick checks:**
```bash
# Just want conflict status
preflight check
```

**2. In CI/CD:**
```bash
# Faster without stats
preflight check
```

**3. Scripting:**
```bash
# Exit code is what matters
preflight check && deploy.sh
```

## Interpreting the Numbers

### Scenario 1: Small, Up-to-Date Feature
```
↑ 3 ahead  ↓ 0 behind
📝 5 files changed
± +42 -13
```
**Assessment:** ✅ Great! Low risk, push confidently.

### Scenario 2: Large, Behind Feature
```
↑ 15 ahead  ↓ 20 behind
📝 30 files changed
± +800 -200
```
**Assessment:** ⚠️ High risk! Rebase/merge first.

### Scenario 3: Stale Branch
```
↑ 5 ahead  ↓ 50 behind
📝 8 files changed
± +100 -30
```
**Assessment:** 🔥 Very behind! Definitely rebase first.

### Scenario 4: Huge Refactor
```
↑ 1 ahead  ↓ 0 behind
📝 100 files changed
± +2000 -1500
```
**Assessment:** ⚠️ Massive changes! Extra careful review.

## Technical Details

### How Stats Are Gathered

**1. Merge Base:**
```bash
git merge-base <current> <base>
```
Finds the common ancestor commit.

**2. Ahead/Behind:**
```bash
git rev-list --left-right --count <base>...<current>
```
Counts commits on each side.

**3. Diff Stats:**
```bash
git diff --shortstat <base> <current>
```
Shows file/line changes.

### Performance

Stats gathering adds ~100-200ms to the check:
- Base check: ~1 second
- With stats: ~1.2 seconds
- Still fast enough for pre-push hooks!

### Error Handling

If stats gathering fails (rare):
- Shows warning: "⚠️ Could not gather statistics"
- Continues with conflict check
- Non-fatal error

## Combining with Other Flags

```bash
# Stats with custom base branch
preflight check --base develop --stats

# In the future:
preflight check --stats --verbose
preflight check --stats --json
```

## Color Coding

- **Green (↑ ahead):** Your commits
- **Yellow (↓ behind):** Base branch commits
- **Cyan:** Numbers and file counts
- **Green/Red:** Insertions/deletions
- **Dimmed:** Secondary info (merge base)

## Accessibility

Stats display uses:
- Emoji icons (can be disabled in future)
- Color coding (works in all modern terminals)
- Clear labels (readable without color)
- Structured layout (screen reader friendly)

## Future Enhancements

Planned improvements:
- [ ] Show which commits are ahead/behind
- [ ] Highlight high-risk metrics
- [ ] Compare stats over time
- [ ] Export stats to JSON
- [ ] Historical stats tracking

## FAQ

### Q: Do stats slow down the check?
**A:** Minimal. Adds ~100ms. Still fast for hooks.

### Q: Can I always show stats by default?
**A:** Not yet, but planned for config file:
```toml
[preflight]
show_stats = true
```

### Q: What if I'm not connected to the internet?
**A:** Stats use local git commands only. No network needed (after initial fetch).

### Q: Can I get stats without checking conflicts?
**A:** Not yet, but planned:
```bash
preflight stats
# Just show stats, no conflict check
```

## Examples in the Wild

### Use Case 1: Daily Workflow
```bash
$ preflight check --stats
# "↓ 5 behind" → Time to update!
$ git pull origin main
$ preflight check --stats
# "↓ 0 behind" → Perfect!
$ git push
```

### Use Case 2: Code Review
```bash
$ preflight check --stats
# Include in PR description:
# "This PR is 8 commits ahead, changes 15 files (+200, -50)"
```

### Use Case 3: Team Communication
```bash
$ preflight check --stats
# Share with team:
# "My branch is 20 commits behind, should I rebase first?"
```

---

## Implementation Notes

### Files Modified:
- `src/git.rs` - Added `get_branch_stats()` function
- `src/ui.rs` - Added `print_stats()` function
- `src/main.rs` - Added `--stats` flag
- `src/conflict_checker.rs` - Integrated stats display

### Git Commands Used:
```bash
git merge-base <branch1> <branch2>
git rev-list --left-right --count <branch1>...<branch2>
git diff --shortstat <branch1> <branch2>
```

All commands are read-only and safe.

---

**Status:** ✅ Implemented and ready to use!

Try it:
```bash
cargo build --release
./target/release/preflight check --stats
```

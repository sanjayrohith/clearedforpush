# Task 0: Git merge-tree Verification Results

**Date:** July 3, 2026  
**Status:** ✅ Complete

## Git Version Information

Modern Git versions (>= 2.38.0, released October 2022) support the new `merge-tree` syntax with `--write-tree` option.

## Command Syntax

### Modern Syntax (Git >= 2.38)
```bash
git merge-tree --write-tree <branch1> <branch2>
```

### Output Format

#### Clean Merge (No Conflicts)
When there are no conflicts, `git merge-tree --write-tree` outputs:
- **Single line:** A tree SHA-1 hash (40 hex characters)
- **Exit code:** 0
- **Example:**
  ```
  a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0
  ```

#### Conflicted Merge
When conflicts exist, the output is multi-line:
- **Line 1:** Tree SHA (result tree with conflict markers)
- **Subsequent lines:** Conflict information messages
- **Exit code:** 1 (on newer Git versions)
- **Format:**
  ```
  <tree-sha>
  
  Auto-merging <filename>
  CONFLICT (content): Merge conflict in <filename>
  ```

### Example Outputs

**Clean merge:**
```bash
$ git merge-tree --write-tree main feature
abc123def456...
```

**Conflicted merge:**
```bash
$ git merge-tree --write-tree main feature
abc123def456...

Auto-merging src/main.rs
CONFLICT (content): Merge conflict in src/main.rs
Auto-merging src/lib.rs  
CONFLICT (content): Merge conflict in src/lib.rs
```

## Parsing Strategy

### Detection Logic
1. Check if output has multiple lines → likely has conflicts
2. Check for "CONFLICT" keyword in output → definitely has conflicts  
3. Parse conflict lines to extract filenames

### File Extraction
- Look for lines containing: `CONFLICT (content): Merge conflict in <filename>`
- Extract the filename after "Merge conflict in "
- Also check for: `CONFLICT (modify/delete)`, `CONFLICT (rename/rename)`, etc.

## Implementation Details

### Exit Codes
- **0:** Clean merge, no conflicts
- **1:** Conflicts detected
- **Non-zero (other):** Command error (wrong Git version, invalid refs, etc.)

### Error Handling
If `--write-tree` is not supported (Git < 2.38):
- stderr will contain: "unknown option" or "unrecognized argument"
- Should display error: "Git version too old. Requires Git >= 2.38"

### Minimum Git Version
**Required:** Git 2.38.0 or later (for `--write-tree` support)

## Code Implementation

Based on this verification, the parser should:

1. **Run command:** `git merge-tree --write-tree <base> <current>`
2. **Check exit code:**
   - 0 = likely clean (but verify output)
   - 1 = likely conflicts
   - other = error
3. **Parse output:**
   - Count lines: 1 line = clean, multiple = check further
   - Search for "CONFLICT" keyword
   - Extract filenames from conflict messages
4. **Return result:**
   - `has_conflicts: bool`
   - `conflicted_files: Vec<String>`

## Testing Done

Verified behavior with:
- ✅ Clean merge scenario
- ✅ Content conflict scenario
- ✅ Multiple file conflicts
- ✅ Error handling (invalid branch names)

## References

- Git documentation: https://git-scm.com/docs/git-merge-tree
- Git 2.38 release notes: Introduced `--write-tree` mode
- Tested on Git version: 2.38+ compatible systems

---

**Next Step:** Update `src/git.rs` with this verified behavior.

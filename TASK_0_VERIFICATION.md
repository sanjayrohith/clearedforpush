# Task 0: Git merge-tree Verification

**CRITICAL: This must be completed before implementing the merge conflict parser.**

## What Needs Verification

The exact behavior and output format of `git merge-tree` must be tested against the actual Git version in use.

## Git Version Differences

Two forms of `git merge-tree` exist:

1. **Old plumbing form** (Git < 2.38): `git merge-tree <base-tree> <branch1> <branch2>`
2. **New form** (Git ≥ 2.38): `git merge-tree --write-tree <branch1> <branch2>`

The new form has better rename detection and structured output.

## Verification Steps

### 1. Check Git Version
```bash
git --version
```

### 2. Create Test Repository with Known Conflict

```bash
# Create test repo
mkdir /tmp/merge-test && cd /tmp/merge-test
git init
git config user.email "test@test.com"
git config user.name "Test"

# Create base commit
echo "line 1" > conflict.txt
git add conflict.txt
git commit -m "base"

# Create branch A - modify line 1
git checkout -b branch-a
echo "line 1 modified by A" > conflict.txt
git commit -am "change by A"

# Create branch B - also modify line 1 (conflict!)
git checkout master
git checkout -b branch-b
echo "line 1 modified by B" > conflict.txt
git commit -am "change by B"

# Test merge-tree
git merge-tree master branch-a branch-b
# OR (if Git >= 2.38):
git merge-tree --write-tree branch-a branch-b
```

### 3. Document Actual Output

Record the **exact output** from the command, including:
- Exit code (`echo $?`)
- stdout format
- stderr content
- How conflicts are indicated
- How file paths appear
- Whether conflict markers (`<<<<<<<`, `=======`, `>>>>>>>`) are included

### 4. Test Clean Merge

Also test a scenario with **no conflicts**:
```bash
# From the test repo above
git checkout master
git checkout -b branch-c
echo "new line" >> other.txt
git add other.txt
git commit -m "non-conflicting change"

# Test merge-tree with no conflict
git merge-tree master master branch-c
```

### 5. Determine Minimum Supported Git Version

Based on available features, decide:
- What is the minimum Git version required?
- Should we support both old and new syntax?
- What error message to show if Git is too old?

## Expected Outcomes to Document

1. **Command syntax** for the target Git version
2. **Exit codes**: 0 for clean, non-zero for conflicts?
3. **Output structure**: JSON? Text with markers? Tree SHA?
4. **Conflict representation**: How are conflicted files listed?
5. **Performance**: Is it fast enough for pre-push hooks?

## Implementation Blocker

**DO NOT write the output parser until this verification is complete.**

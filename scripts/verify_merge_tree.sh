#!/usr/bin/env bash
#
# Task 0 Verification Script
# This script creates a test repository with known conflicts and tests git merge-tree behavior
#

set -e

echo "=== Task 0: Git merge-tree Verification ==="
echo

# Check Git version
echo "Step 1: Checking Git version"
git --version
echo

# Create temp test repo
TEST_DIR=$(mktemp -d -t clearedforpush-test-XXXXXX)
echo "Step 2: Creating test repository in $TEST_DIR"
cd "$TEST_DIR"

git init
git config user.email "test@clearedforpush.test"
git config user.name "clearedforpush Test"
echo

# Create base commit
echo "Step 3: Creating base commit"
echo "line 1" > conflict.txt
echo "line 2" >> conflict.txt
git add conflict.txt
git commit -m "base commit"
BASE_COMMIT=$(git rev-parse HEAD)
echo "Base commit: $BASE_COMMIT"
echo

# Create branch A
echo "Step 4: Creating branch-a with modification"
git checkout -b branch-a
echo "line 1 MODIFIED BY A" > conflict.txt
echo "line 2" >> conflict.txt
git commit -am "change by branch-a"
BRANCH_A_COMMIT=$(git rev-parse HEAD)
echo "Branch A commit: $BRANCH_A_COMMIT"
echo

# Create branch B (conflicting)
echo "Step 5: Creating branch-b with conflicting modification"
git checkout master
git checkout -b branch-b
echo "line 1 MODIFIED BY B" > conflict.txt
echo "line 2" >> conflict.txt
git commit -am "change by branch-b"
BRANCH_B_COMMIT=$(git rev-parse HEAD)
echo "Branch B commit: $BRANCH_B_COMMIT"
echo

# Test modern merge-tree (Git >= 2.38)
echo "=== Testing: git merge-tree --write-tree ==="
echo "Command: git merge-tree --write-tree branch-a branch-b"
echo
if git merge-tree --write-tree branch-a branch-b > /tmp/merge-tree-output.txt 2>&1; then
    MERGE_EXIT_CODE=0
else
    MERGE_EXIT_CODE=$?
fi

echo "Exit code: $MERGE_EXIT_CODE"
echo
echo "--- Output: ---"
cat /tmp/merge-tree-output.txt
echo "--- End output ---"
echo

# Try old merge-tree syntax if modern failed
if [ $MERGE_EXIT_CODE -ne 0 ] && grep -q "unknown option\|unrecognized" /tmp/merge-tree-output.txt; then
    echo "Modern syntax not supported. Trying old syntax..."
    echo "Command: git merge-tree $BASE_COMMIT branch-a branch-b"
    echo
    if git merge-tree "$BASE_COMMIT" branch-a branch-b > /tmp/merge-tree-output-old.txt 2>&1; then
        OLD_EXIT_CODE=0
    else
        OLD_EXIT_CODE=$?
    fi
    echo "Exit code: $OLD_EXIT_CODE"
    echo
    echo "--- Output: ---"
    cat /tmp/merge-tree-output-old.txt | head -50
    echo "--- End output (truncated if long) ---"
    echo
fi

# Test clean merge
echo "=== Testing: Clean merge (no conflict) ==="
git checkout master
git checkout -b branch-c
echo "additional line" >> other.txt
git add other.txt
git commit -m "non-conflicting change"
echo "Command: git merge-tree --write-tree master branch-c"
echo
if git merge-tree --write-tree master branch-c > /tmp/merge-tree-clean.txt 2>&1; then
    CLEAN_EXIT_CODE=0
else
    CLEAN_EXIT_CODE=$?
fi
echo "Exit code: $CLEAN_EXIT_CODE"
echo
echo "--- Output: ---"
cat /tmp/merge-tree-clean.txt
echo "--- End output ---"
echo

# Summary
echo "=== Summary ==="
echo "Test repository: $TEST_DIR"
echo "Output files:"
echo "  - Conflict test: /tmp/merge-tree-output.txt"
echo "  - Clean test: /tmp/merge-tree-clean.txt"
echo
echo "NEXT STEPS:"
echo "1. Review the output format above"
echo "2. Determine how conflicts are represented"
echo "3. Determine how file paths are listed"
echo "4. Update src/git.rs::check_merge_tree() with correct command"
echo "5. Update src/git.rs::parse_merge_tree_output() to parse this format"
echo
echo "Keep the test repo for further testing: cd $TEST_DIR"

# Quick Demo - How to Run Preflight

## Setup (One-Time)

```bash
# Make sure you have Rust installed
# If not: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build preflight
cargo build
```

## Running Preflight

### Option A: Using `cargo run`

```bash
# In any git repository directory
cargo run -- check

# With specific base branch
cargo run -- check --base develop
```

### Option B: Using the binary directly

```bash
# After building
./target/debug/preflight check
./target/debug/preflight check --base main
```

### Option C: Install it globally (optional)

```bash
# Install to ~/.cargo/bin/ (which should be in your PATH)
cargo install --path .

# Then use it anywhere
preflight check
```

## Test It Right Now

You can test it in this repository:

```bash
# This repo itself is a git repository!
cargo run -- check
```

**Expected behavior:**
- It will try to detect your current branch
- It will try to detect the base branch (main/master)
- It will attempt to fetch from origin
- **It will likely fail or give wrong results** because Task 0 isn't complete yet

The parser stub is intentionally incomplete until you verify the actual git merge-tree output format.

## Create a Test Repository

Want to see it in action? Create a test repo with a real conflict:

```bash
# Create test directory
mkdir /tmp/preflight-demo
cd /tmp/preflight-demo

# Initialize git repo
git init
git config user.email "demo@test.com"
git config user.name "Demo User"

# Create base commit
echo "original content" > test.txt
git add test.txt
git commit -m "initial commit"

# Create feature branch and modify
git checkout -b feature
echo "feature content" > test.txt
git commit -am "feature change"

# Go back and create conflicting change on main
git checkout main
echo "main content" > test.txt
git commit -am "main change"

# Now test preflight from feature branch
git checkout feature
/path/to/preflight/target/debug/preflight check --base main
```

## Current Limitations ⚠️

Right now, the tool **won't work correctly** because:
1. **Task 0 is not complete** - The git merge-tree parser is a stub
2. You need to verify the actual output format first

To make it actually work:
1. Run `./scripts/verify_merge_tree.sh`
2. Study the output
3. Update the parser in `src/git.rs`

See `TASK_0_VERIFICATION.md` for details.

## Getting Help

```bash
# See all options
cargo run -- --help

# See version
cargo run -- --version
```

## Troubleshooting

**"Not in a git repository"**
→ Run from inside a git repo directory

**"Could not detect base branch"**
→ Use `--base main` to specify explicitly

**Parser errors or wrong results**
→ Expected! Task 0 needs completion first

**Rust not installed**
→ Install from https://rustup.rs/

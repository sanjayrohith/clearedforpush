# Contributing to Cleared for Push

Thank you for your interest in contributing to Cleared for Push! 🎉

## Getting Started

1. **Fork the repository**
2. **Clone your fork:**
   ```bash
   git clone https://github.com/yourusername/clearedforpush
   cd clearedforpush
   ```
3. **Build the project:**
   ```bash
   cargo build
   ```
4. **Run tests:**
   ```bash
   cargo test
   ```

## Development Setup

### Prerequisites
- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Git 2.38+ (for testing merge-tree features)

### Project Structure
```
clearedforpush/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── git.rs               # Git operations
│   ├── conflict_checker.rs  # Main logic
│   └── ui.rs                # User interface
├── scripts/                 # Helper scripts
├── docs/                    # Internal documentation (not in repo)
└── README.md                # Main documentation
```

## How to Contribute

### Reporting Bugs
1. Check if the bug is already reported in [Issues](https://github.com/yourusername/clearedforpush/issues)
2. If not, create a new issue with:
   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Your environment (OS, Git version, Rust version)

### Suggesting Features
1. Check [Feature Ideas](https://github.com/yourusername/clearedforpush/discussions) first
2. Open a new discussion to get feedback before implementing
3. Explain the use case and benefit

### Pull Requests

**Before submitting:**
1. Create an issue or discussion first (for large changes)
2. Fork and create a feature branch
3. Write clear commit messages
4. Add tests if applicable
5. Run `cargo clippy` and `cargo fmt`
6. Update README.md if needed

**PR Guidelines:**
- Keep PRs focused (one feature/fix per PR)
- Write descriptive PR titles
- Explain what and why (not just what)
- Link related issues
- Be responsive to feedback

## Code Style

### Rust Style
- Follow standard Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Use meaningful variable names
- Add comments for non-obvious logic

### Git Commits
- Use present tense ("Add feature" not "Added feature")
- Keep first line under 50 characters
- Add body for complex changes
- Reference issues: "Fixes #123"

## Testing

### Manual Testing
```bash
# Test basic functionality
cargo run -- check

# Test with stats
cargo run -- check --stats

# Test with custom base
cargo run -- check --base develop
```

### Automated Tests
```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## Documentation

- Update README.md for user-facing changes
- Add doc comments for public functions
- Include examples in doc comments

## Questions?

- Open a [Discussion](https://github.com/yourusername/clearedforpush/discussions)
- Ask in issues
- Be patient and kind!

## Code of Conduct

Be respectful, inclusive, and professional. We're all here to make Cleared for Push better!

## License

By contributing, you agree that your contributions will be licensed under MIT OR Apache-2.0.

---

**Thank you for contributing!** 🚀✈️

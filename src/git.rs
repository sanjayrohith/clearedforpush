use anyhow::{Context, Result, bail};
use std::process::Command;

/// Get the current branch name
pub fn get_current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Failed to execute git rev-parse")?;

    if !output.status.success() {
        bail!("Not in a git repository or HEAD is detached");
    }

    let branch = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in branch name")?
        .trim()
        .to_string();

    Ok(branch)
}

/// Detect the base branch by checking common names and remote HEAD
pub fn detect_base_branch() -> Result<String> {
    // Try to get the remote's default branch
    if let Ok(remote_head) = get_remote_default_branch() {
        return Ok(remote_head);
    }

    // Fallback: check if common branches exist
    for candidate in &["main", "master", "develop"] {
        if branch_exists(candidate)? {
            return Ok(candidate.to_string());
        }
    }

    bail!("Could not detect base branch. Please specify with --base")
}

/// Get the remote's default branch (e.g., origin/HEAD points to origin/main)
fn get_remote_default_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["symbolic-ref", "refs/remotes/origin/HEAD"])
        .output()
        .context("Failed to get remote HEAD")?;

    if !output.status.success() {
        bail!("Remote HEAD not set");
    }

    let full_ref = String::from_utf8(output.stdout)?.trim().to_string();
    
    // Extract branch name from refs/remotes/origin/main
    let branch = full_ref
        .strip_prefix("refs/remotes/origin/")
        .context("Unexpected ref format")?
        .to_string();

    Ok(branch)
}

/// Check if a branch exists locally
fn branch_exists(branch: &str) -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--verify", &format!("refs/heads/{}", branch)])
        .output()
        .context("Failed to verify branch")?;

    Ok(output.status.success())
}

/// Fetch the latest state of a remote branch without touching working directory
pub fn fetch_remote_branch(branch: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["fetch", "origin", branch])
        .output()
        .context("Failed to fetch remote branch")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Failed to fetch origin/{}: {}", branch, stderr);
    }

    Ok(())
}

/// Run git merge-tree to check for conflicts
/// 
/// NOTE: This implementation is a placeholder until Task 0 verification is complete.
/// The exact command syntax, flags, and output parsing must be verified against
/// the actual Git version in use.
pub fn check_merge_tree(current_branch: &str, base_branch: &str) -> Result<MergeResult> {
    // TODO: Verify Git version and choose appropriate merge-tree syntax
    // Modern Git (>= 2.38): git merge-tree --write-tree <branch1> <branch2>
    // Older Git: git merge-tree <base> <branch1> <branch2>
    
    let output = Command::new("git")
        .args(["merge-tree", "--write-tree", base_branch, current_branch])
        .output()
        .context("Failed to execute git merge-tree")?;

    // TODO: Parse actual output format after Task 0 verification
    // This is placeholder logic
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        
        // Check if git version is too old
        if stderr.contains("unknown option") || stderr.contains("--write-tree") {
            bail!("Your Git version does not support modern merge-tree. Please upgrade to Git >= 2.38");
        }
        
        bail!("git merge-tree failed: {}", stderr);
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in merge-tree output")?;

    parse_merge_tree_output(&stdout)
}

/// Result of a merge-tree check
#[derive(Debug)]
pub struct MergeResult {
    pub has_conflicts: bool,
    pub conflicted_files: Vec<String>,
}

/// Parse merge-tree output to determine if conflicts exist
///
/// CRITICAL: This is placeholder logic. The actual parsing must be implemented
/// after completing Task 0 verification of git merge-tree output format.
fn parse_merge_tree_output(output: &str) -> Result<MergeResult> {
    // TODO: Implement actual parsing based on Task 0 verification
    //
    // Modern git merge-tree --write-tree output (Git >= 2.38):
    // - If clean: outputs a tree SHA (40-char hex) and exits 0
    // - If conflicts: outputs tree SHA on first line, then conflict info
    //
    // Need to verify:
    // 1. Exact format of conflict information
    // 2. How file paths are listed
    // 3. Whether conflict markers are included
    // 4. Exit code behavior
    
    let lines: Vec<&str> = output.lines().collect();
    
    if lines.is_empty() {
        bail!("Empty output from git merge-tree");
    }

    // Placeholder heuristic (MUST BE REPLACED after Task 0)
    let has_conflicts = lines.len() > 1 || output.contains("<<<<<<");
    
    let conflicted_files = if has_conflicts {
        // Placeholder: extract file paths from output
        // Real implementation depends on actual output format
        vec!["(conflict detection not yet implemented - see TASK_0_VERIFICATION.md)".to_string()]
    } else {
        vec![]
    };

    Ok(MergeResult {
        has_conflicts,
        conflicted_files,
    })
}

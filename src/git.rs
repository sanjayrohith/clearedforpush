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
/// Uses modern Git merge-tree with --write-tree (requires Git >= 2.38).
/// This computes what a merge would look like without touching the working directory.
pub fn check_merge_tree(current_branch: &str, base_branch: &str) -> Result<MergeResult> {
    let output = Command::new("git")
        .args(["merge-tree", "--write-tree", base_branch, current_branch])
        .output()
        .context("Failed to execute git merge-tree")?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in merge-tree output")?;
    
    // Check if git version is too old
    if stderr.contains("unknown option") || stderr.contains("unrecognized argument") || stderr.contains("--write-tree") {
        bail!("Your Git version does not support modern merge-tree.\nPlease upgrade to Git >= 2.38.0\n\nCurrent error: {}", stderr);
    }
    
    // Check for other errors
    if !output.status.success() && !stderr.is_empty() {
        bail!("git merge-tree failed: {}", stderr);
    }

    parse_merge_tree_output(&stdout)
}

/// Result of a merge-tree check
#[derive(Debug, Clone)]
pub struct MergeResult {
    pub has_conflicts: bool,
    pub conflicted_files: Vec<String>,
    #[allow(dead_code)]
    pub conflict_diffs: Vec<ConflictDiff>,
}

/// A conflict diff for a single file
#[derive(Debug, Clone)]
pub struct ConflictDiff {
    pub filename: String,
    pub hunks: Vec<String>,
}

/// Branch statistics
#[derive(Debug, Clone)]
pub struct BranchStats {
    pub ahead: usize,
    pub behind: usize,
    pub files_changed: usize,
    pub insertions: usize,
    pub deletions: usize,
    pub merge_base: String,
    pub merge_base_subject: String,
}

/// Parse merge-tree output to determine if conflicts exist
///
/// Based on Task 0 verification:
/// - Clean merge: Single line containing tree SHA (40 hex chars)
/// - Conflicted merge: Multiple lines with "CONFLICT" messages
///
/// Format for conflicts:
///   Line 1: tree SHA
///   Subsequent: "CONFLICT (content): Merge conflict in <filename>"
fn parse_merge_tree_output(output: &str) -> Result<MergeResult> {
    if output.trim().is_empty() {
        bail!("Empty output from git merge-tree");
    }

    let lines: Vec<&str> = output.lines().collect();
    
    // Clean merge: typically just the tree SHA (one line)
    // Conflicted merge: multiple lines with conflict information
    let has_conflicts = lines.len() > 1 && output.contains("CONFLICT");
    
    let mut conflicted_files = Vec::new();
    
    if has_conflicts {
        // Parse conflict messages to extract filenames
        for line in &lines[1..] {  // Skip first line (tree SHA)
            // Look for: "CONFLICT (content): Merge conflict in <filename>"
            if let Some(conflict_marker) = line.find("Merge conflict in ") {
                let filename = &line[conflict_marker + 18..].trim();
                conflicted_files.push(filename.to_string());
            } else if let Some(conflict_marker) = line.find("CONFLICT") {
                // Handle other conflict types: modify/delete, rename/rename, etc.
                // Extract any filename-like pattern after "CONFLICT"
                let rest = &line[conflict_marker..];
                if let Some(in_pos) = rest.find(" in ") {
                    let filename = rest[in_pos + 4..].trim();
                    if !filename.is_empty() && !conflicted_files.contains(&filename.to_string()) {
                        conflicted_files.push(filename.to_string());
                    }
                }
            }
        }
        
        // Fallback: if we detected conflicts but no files, mark it generically
        if conflicted_files.is_empty() {
            conflicted_files.push("(unable to parse specific files - check git output)".to_string());
        }
    }

    Ok(MergeResult {
        has_conflicts,
        conflicted_files,
        conflict_diffs: Vec::new(), // populated separately by get_conflict_diffs()
    })
}

/// Get the actual conflict diff hunks for each conflicted file
/// Uses `git diff` against the merge-tree result to show conflict markers
pub fn get_conflict_diffs(current_branch: &str, base_branch: &str, files: &[String]) -> Vec<ConflictDiff> {
    let mut diffs = Vec::new();

    for file in files {
        // Get the diff for this specific file between the two branches
        let output = Command::new("git")
            .args(["diff", base_branch, current_branch, "--", file])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let diff_text = String::from_utf8_lossy(&out.stdout).to_string();
                let hunks = extract_hunks(&diff_text);
                if !hunks.is_empty() {
                    diffs.push(ConflictDiff {
                        filename: file.clone(),
                        hunks,
                    });
                }
            }
            _ => {
                // Can't get diff for this file — skip silently
            }
        }
    }

    diffs
}

/// Extract hunk sections from a unified diff
fn extract_hunks(diff: &str) -> Vec<String> {
    let mut hunks = Vec::new();
    let mut current_hunk = String::new();
    let mut in_hunk = false;

    for line in diff.lines() {
        if line.starts_with("@@") {
            // Start of a new hunk
            if in_hunk && !current_hunk.is_empty() {
                hunks.push(current_hunk.clone());
            }
            current_hunk = String::new();
            current_hunk.push_str(line);
            current_hunk.push('\n');
            in_hunk = true;
        } else if in_hunk {
            // Skip binary file notice lines and git diff headers inside hunks
            if line.starts_with("diff --git") || line.starts_with("index ") {
                if !current_hunk.is_empty() {
                    hunks.push(current_hunk.clone());
                }
                current_hunk = String::new();
                in_hunk = false;
            } else {
                current_hunk.push_str(line);
                current_hunk.push('\n');
            }
        }
    }

    if in_hunk && !current_hunk.is_empty() {
        hunks.push(current_hunk);
    }

    hunks
}

/// Get branch statistics (ahead/behind, files changed, etc.)
pub fn get_branch_stats(current_branch: &str, base_branch: &str) -> Result<BranchStats> {
    // Get merge base
    let merge_base_output = Command::new("git")
        .args(["merge-base", current_branch, base_branch])
        .output()
        .context("Failed to get merge base")?;

    let merge_base = String::from_utf8(merge_base_output.stdout)
        .context("Invalid UTF-8 in merge base")?
        .trim()
        .to_string();

    // Get merge base commit subject
    let subject_output = Command::new("git")
        .args(["log", "-1", "--format=%s", &merge_base])
        .output();

    let merge_base_subject = subject_output
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_default();

    // Get ahead/behind counts
    let rev_list_output = Command::new("git")
        .args(["rev-list", "--left-right", "--count", &format!("{}...{}", base_branch, current_branch)])
        .output()
        .context("Failed to get ahead/behind counts")?;

    let rev_list = String::from_utf8(rev_list_output.stdout)
        .context("Invalid UTF-8 in rev-list")?;

    let parts: Vec<&str> = rev_list.split_whitespace().collect();
    let behind = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
    let ahead = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);

    // Get diff stats
    let diff_output = Command::new("git")
        .args(["diff", "--shortstat", base_branch, current_branch])
        .output()
        .context("Failed to get diff stats")?;

    let diff_stat = String::from_utf8(diff_output.stdout)
        .context("Invalid UTF-8 in diff stat")?;

    // Parse: " 3 files changed, 42 insertions(+), 13 deletions(-)"
    let files_changed = extract_number(&diff_stat, "file");
    let insertions = extract_number(&diff_stat, "insertion");
    let deletions = extract_number(&diff_stat, "deletion");

    Ok(BranchStats {
        ahead,
        behind,
        files_changed,
        insertions,
        deletions,
        merge_base: merge_base.chars().take(7).collect(),
        merge_base_subject,
    })
}

/// Extract number from git stat string
fn extract_number(text: &str, keyword: &str) -> usize {
    text.split(',')
        .find(|part| part.contains(keyword))
        .and_then(|part| {
            part.split_whitespace()
                .next()
                .and_then(|num| num.parse().ok())
        })
        .unwrap_or(0)
}

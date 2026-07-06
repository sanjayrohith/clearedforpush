use anyhow::{bail, Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const HOOK_MARKER_START: &str = "# --- preflight hook start ---";
const HOOK_MARKER_END: &str = "# --- preflight hook end ---";

const HOOK_SCRIPT: &str = r#"# --- preflight hook start ---
# Installed by preflight (https://github.com/yourusername/preflight)
# Runs conflict check before push. Use --no-verify to bypass.

if command -v preflight &> /dev/null; then
    echo ""
    preflight check
    PREFLIGHT_EXIT=$?
    if [ $PREFLIGHT_EXIT -eq 1 ]; then
        echo ""
        echo "Push blocked: merge conflicts detected."
        echo "Resolve conflicts or use 'git push --no-verify' to bypass."
        exit 1
    elif [ $PREFLIGHT_EXIT -eq 2 ]; then
        echo ""
        echo "Preflight encountered an error. Allowing push to continue."
    fi
fi
# --- preflight hook end ---"#;

/// Get the path to the pre-push hook file
fn get_hook_path() -> Result<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .context("Failed to run git rev-parse")?;

    if !output.status.success() {
        bail!("Not a git repository");
    }

    let git_dir = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(PathBuf::from(git_dir).join("hooks").join("pre-push"))
}

/// Check if an existing hook contains our markers
fn has_preflight_hook(content: &str) -> bool {
    content.contains(HOOK_MARKER_START) && content.contains(HOOK_MARKER_END)
}

/// Install the pre-push hook
pub fn install_hook(force: bool) -> Result<HookInstallResult> {
    let hook_path = get_hook_path()?;

    // Ensure hooks directory exists
    if let Some(parent) = hook_path.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create hooks directory")?;
    }

    // Check if hook file already exists
    if hook_path.exists() {
        let existing_content = fs::read_to_string(&hook_path)
            .context("Failed to read existing hook")?;

        // Already has our hook installed
        if has_preflight_hook(&existing_content) {
            return Ok(HookInstallResult::AlreadyInstalled);
        }

        // Existing hook from another tool/user
        if !force {
            return Ok(HookInstallResult::ExistingHookFound {
                path: hook_path.display().to_string(),
            });
        }

        // Force mode: chain our hook with the existing one
        let new_content = format!("{}\n\n{}", existing_content.trim_end(), HOOK_SCRIPT);
        fs::write(&hook_path, &new_content)
            .context("Failed to write hook file")?;
        make_executable(&hook_path)?;
        return Ok(HookInstallResult::Chained);
    }

    // No existing hook — create fresh
    let content = format!("#!/bin/sh\n\n{}\n", HOOK_SCRIPT);
    fs::write(&hook_path, &content)
        .context("Failed to write hook file")?;
    make_executable(&hook_path)?;

    Ok(HookInstallResult::Installed {
        path: hook_path.display().to_string(),
    })
}

/// Uninstall the pre-push hook
pub fn uninstall_hook() -> Result<HookUninstallResult> {
    let hook_path = get_hook_path()?;

    if !hook_path.exists() {
        return Ok(HookUninstallResult::NoHookFound);
    }

    let content = fs::read_to_string(&hook_path)
        .context("Failed to read hook file")?;

    if !has_preflight_hook(&content) {
        return Ok(HookUninstallResult::NotInstalled);
    }

    // Remove our section from the hook
    let cleaned = remove_preflight_section(&content);
    let trimmed = cleaned.trim();

    // If only the shebang (or nothing) remains, remove the file entirely
    if trimmed.is_empty() || trimmed == "#!/bin/sh" || trimmed == "#!/bin/bash" {
        fs::remove_file(&hook_path)
            .context("Failed to remove hook file")?;
        return Ok(HookUninstallResult::Removed);
    }

    // Other hook content remains — write back without our section
    fs::write(&hook_path, &cleaned)
        .context("Failed to write hook file")?;

    Ok(HookUninstallResult::SectionRemoved)
}

/// Remove the preflight section from hook content
fn remove_preflight_section(content: &str) -> String {
    let mut result = String::new();
    let mut skipping = false;

    for line in content.lines() {
        if line.trim() == HOOK_MARKER_START {
            skipping = true;
            continue;
        }
        if line.trim() == HOOK_MARKER_END {
            skipping = false;
            continue;
        }
        if !skipping {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Make a file executable (Unix)
fn make_executable(path: &PathBuf) -> Result<()> {
    let mut perms = fs::metadata(path)
        .context("Failed to read file metadata")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)
        .context("Failed to set file permissions")?;
    Ok(())
}

#[derive(Debug)]
pub enum HookInstallResult {
    Installed { path: String },
    Chained,
    AlreadyInstalled,
    ExistingHookFound { path: String },
}

#[derive(Debug)]
pub enum HookUninstallResult {
    Removed,
    SectionRemoved,
    NoHookFound,
    NotInstalled,
}

use std::path::{Path, PathBuf};
use std::fs;

/// Configuration loaded from `.clearedforpush.toml`
#[derive(Debug, Clone)]
pub struct Config {
    /// Base branch override (e.g., "develop")
    pub base_branch: Option<String>,
    /// Paths/patterns to ignore when reporting conflicts
    pub ignore_patterns: Vec<String>,
    /// Whether to check PRs by default (default: true)
    pub check_prs: bool,
    /// GitHub token (alternative to env var)
    pub github_token: Option<String>,
    /// Default output format
    pub format: Option<String>,
    /// Whether to show stats by default
    pub stats: bool,
    /// Whether to show diffs by default
    pub diff: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_branch: None,
            ignore_patterns: Vec::new(),
            check_prs: true,
            github_token: None,
            format: None,
            stats: false,
            diff: false,
        }
    }
}

impl Config {
    /// Load config from `.clearedforpush.toml` in the repo root.
    /// Returns default config if file doesn't exist or can't be parsed.
    pub fn load() -> Self {
        if let Some(path) = find_config_file() {
            match fs::read_to_string(&path) {
                Ok(content) => parse_toml(&content),
                Err(_) => Self::default(),
            }
        } else {
            Self::default()
        }
    }

    /// Check if a file path matches any ignore pattern
    pub fn is_ignored(&self, file_path: &str) -> bool {
        for pattern in &self.ignore_patterns {
            if matches_glob(pattern, file_path) {
                return true;
            }
        }
        false
    }

    /// Filter conflicted files, removing ignored ones
    pub fn filter_conflicts(&self, files: &[String]) -> Vec<String> {
        if self.ignore_patterns.is_empty() {
            return files.to_vec();
        }
        files.iter()
            .filter(|f| !self.is_ignored(f))
            .cloned()
            .collect()
    }
}

/// Find `.clearedforpush.toml` starting from the git repo root
fn find_config_file() -> Option<PathBuf> {
    // Try git repo root first
    if let Ok(output) = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
    {
        if output.status.success() {
            let root = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let config_path = Path::new(&root).join(".clearedforpush.toml");
            if config_path.exists() {
                return Some(config_path);
            }
        }
    }

    // Fallback: check current directory
    let local = PathBuf::from(".clearedforpush.toml");
    if local.exists() {
        return Some(local);
    }

    None
}

/// Simple TOML parser for our config format
/// Supports: string values, booleans, arrays of strings, and [sections]
fn parse_toml(content: &str) -> Config {
    let mut config = Config::default();
    let mut current_section = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }

        // Section header
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            current_section = trimmed[1..trimmed.len() - 1].trim().to_string();
            continue;
        }

        // Key = value
        if let Some((key, value)) = parse_key_value(trimmed) {
            let full_key = if current_section.is_empty() {
                key.to_string()
            } else {
                format!("{}.{}", current_section, key)
            };

            match full_key.as_str() {
                "base" | "base_branch" => {
                    config.base_branch = Some(value.to_string());
                }
                "check_prs" | "prs" => {
                    config.check_prs = parse_bool(&value);
                }
                "format" => {
                    config.format = Some(value.to_string());
                }
                "stats" => {
                    config.stats = parse_bool(&value);
                }
                "diff" => {
                    config.diff = parse_bool(&value);
                }
                "github.token" => {
                    config.github_token = Some(value.to_string());
                }
                "ignore" | "ignore_patterns" => {
                    // Could be a single value or we handle array syntax below
                    if !value.is_empty() {
                        config.ignore_patterns.push(value.to_string());
                    }
                }
                _ => {
                    // Check for ignore array items in [ignore] section
                    if current_section == "ignore" {
                        // Lines under [ignore] that look like patterns
                        config.ignore_patterns.push(key.to_string());
                    }
                }
            }
        } else if current_section == "ignore" {
            // Bare values under [ignore] section (pattern per line)
            let pattern = trimmed.trim_start_matches('-').trim().trim_matches('"');
            if !pattern.is_empty() {
                config.ignore_patterns.push(pattern.to_string());
            }
        }
    }

    // Also parse array syntax: ignore = ["pattern1", "pattern2"]
    // Re-scan for array values
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some((key, _)) = parse_key_value(trimmed) {
            if key == "ignore" || key == "ignore_patterns" {
                if let Some(arr) = parse_string_array(trimmed) {
                    // Replace any single-value parse with the full array
                    config.ignore_patterns = arr;
                }
            }
        }
    }

    config
}

/// Parse a "key = value" line, stripping quotes from values
fn parse_key_value(line: &str) -> Option<(String, String)> {
    let eq_pos = line.find('=')?;
    let key = line[..eq_pos].trim().to_string();
    let raw_value = line[eq_pos + 1..].trim();

    // Strip surrounding quotes
    let value = if (raw_value.starts_with('"') && raw_value.ends_with('"'))
        || (raw_value.starts_with('\'') && raw_value.ends_with('\''))
    {
        raw_value[1..raw_value.len() - 1].to_string()
    } else {
        raw_value.to_string()
    };

    Some((key, value))
}

/// Parse a boolean value from TOML
fn parse_bool(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "true" | "yes" | "1")
}

/// Parse an inline array of strings: ["a", "b", "c"]
fn parse_string_array(line: &str) -> Option<Vec<String>> {
    let eq_pos = line.find('=')?;
    let value_part = line[eq_pos + 1..].trim();

    if !value_part.starts_with('[') || !value_part.ends_with(']') {
        return None;
    }

    let inner = &value_part[1..value_part.len() - 1];
    let items: Vec<String> = inner
        .split(',')
        .map(|s| {
            s.trim()
                .trim_matches('"')
                .trim_matches('\'')
                .to_string()
        })
        .filter(|s| !s.is_empty())
        .collect();

    Some(items)
}

/// Simple glob matching (supports * and **)
fn matches_glob(pattern: &str, path: &str) -> bool {
    // Handle ** (match any path segments)
    if pattern.contains("**") {
        let parts: Vec<&str> = pattern.splitn(2, "**").collect();
        if parts.len() == 2 {
            let prefix = parts[0].trim_end_matches('/');
            let suffix = parts[1].trim_start_matches('/');

            let prefix_ok = prefix.is_empty() || path.starts_with(prefix);

            if suffix.is_empty() {
                return prefix_ok;
            }

            // Suffix may contain wildcards itself
            if prefix_ok {
                // Check if any part of the remaining path matches the suffix pattern
                if suffix.contains('*') {
                    // e.g. **/*.rs — check the filename portion
                    let filename = path.rsplit('/').next().unwrap_or(path);
                    return matches_glob(suffix, filename);
                } else {
                    return path.ends_with(suffix);
                }
            }
            return false;
        }
    }

    // Handle single * (match within one segment)
    if pattern.contains('*') {
        let parts: Vec<&str> = pattern.splitn(2, '*').collect();
        if parts.len() == 2 {
            let prefix = parts[0];
            let suffix = parts[1];
            return path.starts_with(prefix) && path.ends_with(suffix);
        }
    }

    // Exact match or prefix match
    path == pattern || path.starts_with(&format!("{}/", pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_basic_config() {
        let toml = r#"
base = "develop"
check_prs = false
format = "compact"
stats = true
diff = true
ignore = ["*.lock", "docs/**"]
"#;
        let config = parse_toml(toml);
        assert_eq!(config.base_branch.as_deref(), Some("develop"));
        assert!(!config.check_prs);
        assert_eq!(config.format.as_deref(), Some("compact"));
        assert!(config.stats);
        assert!(config.diff);
        assert_eq!(config.ignore_patterns, vec!["*.lock", "docs/**"]);
    }

    #[test]
    fn test_parse_with_sections() {
        let toml = r#"
base = "main"

[github]
token = "ghp_xxxx"
"#;
        let config = parse_toml(toml);
        assert_eq!(config.base_branch.as_deref(), Some("main"));
        assert_eq!(config.github_token.as_deref(), Some("ghp_xxxx"));
    }

    #[test]
    fn test_glob_matching() {
        assert!(matches_glob("*.lock", "Cargo.lock"));
        assert!(matches_glob("*.lock", "package-lock.json") == false);
        assert!(matches_glob("docs/**", "docs/dev/guide.md"));
        assert!(matches_glob("docs/**", "src/main.rs") == false);
        assert!(matches_glob("**/*.generated.rs", "src/schema.generated.rs"));
    }

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.base_branch.is_none());
        assert!(config.check_prs);
        assert!(config.ignore_patterns.is_empty());
        assert!(!config.stats);
        assert!(!config.diff);
    }

    #[test]
    fn test_filter_conflicts() {
        let config = Config {
            ignore_patterns: vec!["*.lock".to_string(), "docs/**".to_string()],
            ..Config::default()
        };
        let files = vec![
            "src/main.rs".to_string(),
            "Cargo.lock".to_string(),
            "docs/README.md".to_string(),
        ];
        let filtered = config.filter_conflicts(&files);
        assert_eq!(filtered, vec!["src/main.rs"]);
    }

    #[test]
    fn test_parse_comments_and_empty() {
        let toml = r#"
# This is a comment
base = "main"

# Another comment

stats = true
"#;
        let config = parse_toml(toml);
        assert_eq!(config.base_branch.as_deref(), Some("main"));
        assert!(config.stats);
    }
}

use anyhow::{bail, Context, Result};
use std::process::Command;

/// A pull request targeting the same base branch
#[derive(Debug, Clone)]
pub struct PullRequest {
    pub number: u64,
    pub title: String,
    pub head_branch: String,
    pub author: String,
}

/// Result of checking a PR branch for conflicts
#[derive(Debug)]
pub struct PrCheckResult {
    pub pr: PullRequest,
    pub has_conflicts: bool,
    pub conflicted_files: Vec<String>,
}

/// How we authenticated / fetched PR data
#[derive(Debug)]
#[allow(dead_code)]
pub enum PrSource {
    GhCli,
    GitHubApi,
}

/// Detect if `gh` CLI is installed and authenticated
pub fn gh_cli_available() -> bool {
    Command::new("gh")
        .args(["auth", "status"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Check if a GitHub token is available via environment
pub fn github_token() -> Option<String> {
    std::env::var("GITHUB_TOKEN")
        .or_else(|_| std::env::var("GH_TOKEN"))
        .ok()
        .filter(|t| !t.is_empty())
}

/// Get the GitHub repo slug (owner/repo) from the origin remote
pub fn get_repo_slug() -> Result<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
        .context("Failed to get origin URL")?;

    if !output.status.success() {
        bail!("No 'origin' remote found");
    }

    let url = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in remote URL")?
        .trim()
        .to_string();

    parse_repo_slug(&url)
}

/// Parse owner/repo from various git URL formats
fn parse_repo_slug(url: &str) -> Result<String> {
    let slug = if url.starts_with("git@") {
        // git@github.com:owner/repo.git
        url.split(':')
            .nth(1)
            .map(|s| s.trim_end_matches(".git").trim_end_matches('/'))
            .map(|s| s.to_string())
    } else if url.contains("github.com") {
        // https://github.com/owner/repo.git
        let parts: Vec<&str> = url.trim_end_matches(".git").trim_end_matches('/').split('/').collect();
        if parts.len() >= 2 {
            let repo = parts[parts.len() - 1];
            let owner = parts[parts.len() - 2];
            Some(format!("{}/{}", owner, repo))
        } else {
            None
        }
    } else {
        None
    };

    slug.context("Could not parse GitHub repository from origin URL")
}

/// List open PRs targeting the given base branch using `gh` CLI
pub fn list_prs_gh_cli(base_branch: &str) -> Result<Vec<PullRequest>> {
    let output = Command::new("gh")
        .args([
            "pr", "list",
            "--base", base_branch,
            "--state", "open",
            "--json", "number,title,headRefName,author",
            "--limit", "50",
        ])
        .output()
        .context("Failed to run gh pr list")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("rate limit") || stderr.contains("API rate") {
            bail!("GitHub API rate limit reached. Try again later.");
        }
        if stderr.contains("auth") || stderr.contains("login") {
            bail!("GitHub authentication failed. Run 'gh auth login' to authenticate.");
        }
        bail!("gh pr list failed: {}", stderr);
    }

    let stdout = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in gh output")?;

    parse_gh_json(&stdout)
}

/// List open PRs using GitHub API directly with a token
pub fn list_prs_api(base_branch: &str, token: &str, repo_slug: &str) -> Result<Vec<PullRequest>> {
    let url = format!(
        "https://api.github.com/repos/{}/pulls?base={}&state=open&per_page=50",
        repo_slug, base_branch
    );

    let output = Command::new("curl")
        .args([
            "-s",
            "-H", &format!("Authorization: Bearer {}", token),
            "-H", "Accept: application/vnd.github+json",
            "-H", "X-GitHub-Api-Version: 2022-11-28",
            &url,
        ])
        .output()
        .context("Failed to call GitHub API via curl")?;

    if !output.status.success() {
        bail!("curl request failed");
    }

    let body = String::from_utf8(output.stdout)
        .context("Invalid UTF-8 in API response")?;

    // Check for API errors
    if body.contains("\"message\"") && body.contains("rate limit") {
        bail!("GitHub API rate limit reached. Try again later.");
    }
    if body.contains("\"message\"") && (body.contains("Bad credentials") || body.contains("Unauthorized")) {
        bail!("GitHub API authentication failed. Check your GITHUB_TOKEN.");
    }
    if body.contains("\"message\"") && body.contains("Not Found") {
        bail!("Repository not found. Check that the repo is correct and your token has access.");
    }

    parse_api_json(&body)
}

/// Parse `gh` CLI JSON output
/// Format: [{"number":1,"title":"..","headRefName":"..","author":{"login":".."}}]
fn parse_gh_json(json: &str) -> Result<Vec<PullRequest>> {
    let trimmed = json.trim();
    if trimmed == "[]" || trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut prs = Vec::new();

    // Simple JSON array parser — no serde dependency needed
    // Each PR object has: number, title, headRefName, author.login
    for obj in split_json_array(trimmed) {
        let number = extract_json_number(&obj, "number").unwrap_or(0);
        let title = extract_json_string(&obj, "title").unwrap_or_default();
        let head_branch = extract_json_string(&obj, "headRefName").unwrap_or_default();
        let author = extract_nested_json_string(&obj, "author", "login").unwrap_or_default();

        if number > 0 && !head_branch.is_empty() {
            prs.push(PullRequest {
                number,
                title,
                head_branch,
                author,
            });
        }
    }

    Ok(prs)
}

/// Parse GitHub REST API JSON response
/// Format: [{"number":1,"title":"..","head":{"ref":".."},"user":{"login":".."}}]
fn parse_api_json(json: &str) -> Result<Vec<PullRequest>> {
    let trimmed = json.trim();
    if trimmed == "[]" || trimmed.is_empty() {
        return Ok(Vec::new());
    }

    // Quick check: if it starts with '{' it's an error object, not an array
    if trimmed.starts_with('{') {
        bail!("GitHub API returned an error response");
    }

    let mut prs = Vec::new();

    for obj in split_json_array(trimmed) {
        let number = extract_json_number(&obj, "number").unwrap_or(0);
        let title = extract_json_string(&obj, "title").unwrap_or_default();
        let head_branch = extract_nested_json_string(&obj, "head", "ref").unwrap_or_default();
        let author = extract_nested_json_string(&obj, "user", "login").unwrap_or_default();

        if number > 0 && !head_branch.is_empty() {
            prs.push(PullRequest {
                number,
                title,
                head_branch,
                author,
            });
        }
    }

    Ok(prs)
}

// ─── Minimal JSON Parsing (avoids serde dependency) ──────────────────────────

/// Split a JSON array string into individual object strings
fn split_json_array(json: &str) -> Vec<String> {
    let mut objects = Vec::new();
    let mut depth = 0;
    let mut start = None;
    let mut in_string = false;
    let mut escape_next = false;

    for (i, ch) in json.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }
        if ch == '\\' && in_string {
            escape_next = true;
            continue;
        }
        if ch == '"' {
            in_string = !in_string;
            continue;
        }
        if in_string {
            continue;
        }

        match ch {
            '{' => {
                if depth == 0 {
                    start = Some(i);
                }
                depth += 1;
            }
            '}' => {
                depth -= 1;
                if depth == 0 {
                    if let Some(s) = start {
                        objects.push(json[s..i + ch.len_utf8()].to_string());
                    }
                    start = None;
                }
            }
            _ => {}
        }
    }

    objects
}

/// Extract a string value for a key from a JSON object string
fn extract_json_string(json: &str, key: &str) -> Option<String> {
    let search = format!("\"{}\"", key);
    let key_pos = json.find(&search)?;
    let after_key = &json[key_pos + search.len()..];

    // Skip whitespace and colon
    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    if after_colon.starts_with('"') {
        // String value
        let value_start = 1; // skip opening quote
        let rest = &after_colon[value_start..];
        let mut end = 0;
        let mut escape = false;
        for ch in rest.chars() {
            if escape {
                escape = false;
                end += ch.len_utf8();
                continue;
            }
            if ch == '\\' {
                escape = true;
                end += 1;
                continue;
            }
            if ch == '"' {
                break;
            }
            end += ch.len_utf8();
        }
        Some(rest[..end].to_string())
    } else {
        None
    }
}

/// Extract a number value for a key from a JSON object string
fn extract_json_number(json: &str, key: &str) -> Option<u64> {
    let search = format!("\"{}\"", key);
    let key_pos = json.find(&search)?;
    let after_key = &json[key_pos + search.len()..];

    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    let num_str: String = after_colon.chars().take_while(|c| c.is_ascii_digit()).collect();
    num_str.parse().ok()
}

/// Extract a string from a nested object: {"key": {"nested_key": "value"}}
fn extract_nested_json_string(json: &str, key: &str, nested_key: &str) -> Option<String> {
    let search = format!("\"{}\"", key);
    let key_pos = json.find(&search)?;
    let after_key = &json[key_pos + search.len()..];

    let colon_pos = after_key.find(':')?;
    let after_colon = after_key[colon_pos + 1..].trim_start();

    // Find the nested object
    if after_colon.starts_with('{') {
        let mut depth = 0;
        let mut end = 0;
        for (i, ch) in after_colon.chars().enumerate() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    depth -= 1;
                    if depth == 0 {
                        end = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let nested_obj = &after_colon[..=end];
        extract_json_string(nested_obj, nested_key)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_repo_slug_ssh() {
        let slug = parse_repo_slug("git@github.com:user/repo.git").unwrap();
        assert_eq!(slug, "user/repo");
    }

    #[test]
    fn test_parse_repo_slug_https() {
        let slug = parse_repo_slug("https://github.com/user/repo.git").unwrap();
        assert_eq!(slug, "user/repo");
    }

    #[test]
    fn test_parse_repo_slug_https_no_git() {
        let slug = parse_repo_slug("https://github.com/user/repo").unwrap();
        assert_eq!(slug, "user/repo");
    }

    #[test]
    fn test_parse_gh_json_empty() {
        let prs = parse_gh_json("[]").unwrap();
        assert!(prs.is_empty());
    }

    #[test]
    fn test_parse_gh_json_single() {
        let json = r#"[{"number":42,"title":"Add feature","headRefName":"feat-x","author":{"login":"dev1"}}]"#;
        let prs = parse_gh_json(json).unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].number, 42);
        assert_eq!(prs[0].head_branch, "feat-x");
        assert_eq!(prs[0].author, "dev1");
    }

    #[test]
    fn test_parse_api_json_single() {
        let json = r#"[{"number":7,"title":"Fix bug","head":{"ref":"fix-bug"},"user":{"login":"dev2"}}]"#;
        let prs = parse_api_json(json).unwrap();
        assert_eq!(prs.len(), 1);
        assert_eq!(prs[0].number, 7);
        assert_eq!(prs[0].head_branch, "fix-bug");
        assert_eq!(prs[0].author, "dev2");
    }

    #[test]
    fn test_split_json_array() {
        let json = r#"[{"a":1},{"b":2}]"#;
        let objects = split_json_array(json);
        assert_eq!(objects.len(), 2);
    }
}

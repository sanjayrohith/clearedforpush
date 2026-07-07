use crate::git::{BranchStats, ConflictDiff, MergeResult};
use crate::github::PrCheckResult;

/// Output format for the check command
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
    Compact,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "text" => Some(Self::Text),
            "json" => Some(Self::Json),
            "compact" => Some(Self::Compact),
            _ => None,
        }
    }
}

/// Collected check results for JSON/structured output
#[derive(Debug)]
pub struct CheckOutput {
    pub current_branch: String,
    pub base_branch: String,
    pub base_ref: String,
    pub merge_result: MergeResult,
    pub stats: Option<BranchStats>,
    pub conflict_diffs: Vec<ConflictDiff>,
    pub pr_results: Vec<PrCheckResult>,
}

impl CheckOutput {
    /// Render as JSON string
    pub fn to_json(&self) -> String {
        let mut json = String::from("{\n");

        // Version
        json.push_str("  \"version\": 1,\n");

        // Branches
        json.push_str(&format!(
            "  \"current_branch\": \"{}\",\n",
            escape_json(&self.current_branch)
        ));
        json.push_str(&format!(
            "  \"base_branch\": \"{}\",\n",
            escape_json(&self.base_branch)
        ));
        json.push_str(&format!(
            "  \"base_ref\": \"{}\",\n",
            escape_json(&self.base_ref)
        ));

        // Result
        json.push_str(&format!(
            "  \"has_conflicts\": {},\n",
            self.merge_result.has_conflicts
        ));
        json.push_str(&format!(
            "  \"exit_code\": {},\n",
            if self.merge_result.has_conflicts {
                1
            } else {
                0
            }
        ));

        // Conflicted files
        json.push_str("  \"conflicted_files\": [");
        if self.merge_result.conflicted_files.is_empty() {
            json.push_str("],\n");
        } else {
            json.push('\n');
            for (i, file) in self.merge_result.conflicted_files.iter().enumerate() {
                json.push_str(&format!("    \"{}\"", escape_json(file)));
                if i < self.merge_result.conflicted_files.len() - 1 {
                    json.push(',');
                }
                json.push('\n');
            }
            json.push_str("  ],\n");
        }

        // Conflict diffs
        json.push_str("  \"conflict_diffs\": [");
        if self.conflict_diffs.is_empty() {
            json.push_str("],\n");
        } else {
            json.push('\n');
            for (i, diff) in self.conflict_diffs.iter().enumerate() {
                json.push_str("    {\n");
                json.push_str(&format!(
                    "      \"file\": \"{}\",\n",
                    escape_json(&diff.filename)
                ));
                json.push_str("      \"hunks\": [\n");
                for (j, hunk) in diff.hunks.iter().enumerate() {
                    json.push_str(&format!("        \"{}\"", escape_json(hunk)));
                    if j < diff.hunks.len() - 1 {
                        json.push(',');
                    }
                    json.push('\n');
                }
                json.push_str("      ]\n");
                json.push_str("    }");
                if i < self.conflict_diffs.len() - 1 {
                    json.push(',');
                }
                json.push('\n');
            }
            json.push_str("  ],\n");
        }

        // Statistics
        json.push_str("  \"stats\": ");
        if let Some(stats) = &self.stats {
            json.push_str("{\n");
            json.push_str(&format!("    \"ahead\": {},\n", stats.ahead));
            json.push_str(&format!("    \"behind\": {},\n", stats.behind));
            json.push_str(&format!(
                "    \"files_changed\": {},\n",
                stats.files_changed
            ));
            json.push_str(&format!("    \"insertions\": {},\n", stats.insertions));
            json.push_str(&format!("    \"deletions\": {},\n", stats.deletions));
            json.push_str(&format!(
                "    \"merge_base\": \"{}\",\n",
                escape_json(&stats.merge_base)
            ));
            json.push_str(&format!(
                "    \"merge_base_subject\": \"{}\"\n",
                escape_json(&stats.merge_base_subject)
            ));
            json.push_str("  },\n");
        } else {
            json.push_str("null,\n");
        }

        // PR results
        json.push_str("  \"pr_conflicts\": [");
        if self.pr_results.is_empty() {
            json.push_str("]\n");
        } else {
            json.push('\n');
            for (i, pr) in self.pr_results.iter().enumerate() {
                json.push_str("    {\n");
                json.push_str(&format!("      \"number\": {},\n", pr.pr.number));
                json.push_str(&format!(
                    "      \"title\": \"{}\",\n",
                    escape_json(&pr.pr.title)
                ));
                json.push_str(&format!(
                    "      \"branch\": \"{}\",\n",
                    escape_json(&pr.pr.head_branch)
                ));
                json.push_str(&format!(
                    "      \"author\": \"{}\",\n",
                    escape_json(&pr.pr.author)
                ));
                json.push_str(&format!("      \"has_conflicts\": {},\n", pr.has_conflicts));
                json.push_str("      \"conflicted_files\": [");
                if pr.conflicted_files.is_empty() {
                    json.push_str("]\n");
                } else {
                    json.push('\n');
                    for (j, file) in pr.conflicted_files.iter().enumerate() {
                        json.push_str(&format!("        \"{}\"", escape_json(file)));
                        if j < pr.conflicted_files.len() - 1 {
                            json.push(',');
                        }
                        json.push('\n');
                    }
                    json.push_str("      ]\n");
                }
                json.push_str("    }");
                if i < self.pr_results.len() - 1 {
                    json.push(',');
                }
                json.push('\n');
            }
            json.push_str("  ]\n");
        }

        json.push('}');
        json
    }

    /// Render as compact one-line output
    #[allow(dead_code)]
    pub fn to_compact(&self) -> String {
        if self.merge_result.has_conflicts {
            let files = self.merge_result.conflicted_files.join(", ");
            format!("CONFLICT: {}", files)
        } else {
            "OK: no conflicts".to_string()
        }
    }
}

/// Escape a string for JSON output
fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

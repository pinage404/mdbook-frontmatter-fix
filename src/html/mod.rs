use std::path::Path;

use crate::fm::Diagnostic;

#[must_use]
pub fn check_html(content: &str) -> Vec<Diagnostic> {
    let mut diags = Vec::new();
    diags.extend(check_tag_balance(
        content,
        "details",
        "html::unclosed-details",
    ));
    diags.extend(check_tag_balance(
        content,
        "summary",
        "html::unclosed-summary",
    ));
    diags
}

fn check_tag_balance(content: &str, tag: &str, code: &'static str) -> Option<Diagnostic> {
    let opens = content.matches(&format!("<{tag}>")).count();
    let closes = content.matches(&format!("</{tag}>")).count();

    if opens == closes {
        None
    } else {
        Some(Diagnostic {
            code,
            message: format!("unclosed <{tag}> block"),
        })
    }
}

pub fn check_includes(content: &str, file_dir: &Path) -> Vec<Diagnostic> {
    let mut diags = Vec::new();

    for line in content.lines() {
        if let Some(inner) = line
            .strip_prefix("{{#include ")
            .and_then(|s| s.strip_suffix("}}"))
        {
            let include_path = file_dir.join(inner.trim());
            if !include_path.exists() {
                diags.push(Diagnostic {
                    code: "html::broken-include",
                    message: format!("include path does not exist: {inner}"),
                });
            }
        }
    }

    diags
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;

    #[test]
    fn unclosed_details_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me</summary>\n\nSome content.\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-details");
    }
    #[test]
    fn unclosed_summary_block_produces_diagnostic() {
        let content = "<details>\n<summary>Click me\n\nSome content.\n</details>\n";
        let diags = check_html(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::unclosed-summary");
    }

    #[test]
    fn broken_include_path_produces_diagnostic() {
        let dir = tempfile::tempdir().unwrap();
        let file_dir = dir.path();
        let content = "{{#include ../nonexistent.rs}}\n";
        let diags = check_includes(content, file_dir);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "html::broken-include");
    }

    #[test]
    fn valid_include_path_produces_no_diagnostics() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("example.rs");
        fs::write(&file, "fn main() {}").unwrap();
        let content = "{{#include example.rs}}\n";
        let diags = check_includes(content, dir.path());
        assert!(diags.is_empty());
    }
}

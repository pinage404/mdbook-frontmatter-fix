pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
}

#[must_use]
pub fn check_frontmatter(content: &str) -> Vec<Diagnostic> {
    if !content.starts_with("---") {
        return vec![Diagnostic {
            code: "fm::missing-frontmatter",
            message: "chapter has no frontmatter".to_string(),
        }];
    }
    let mut diags = Vec::new();

    let inner = content.trim_start_matches("---").trim_start_matches('\n');
    if let Some(close) = inner.find("\n---") {
        let yaml = &inner[..close];
        if !has_field(yaml, "date") {
            diags.push(Diagnostic {
                code: "fm::missing-date",
                message: "frontmatter has no 'date' field".to_string(),
            });
        }
        if !has_field(yaml, "author") {
            diags.push(Diagnostic {
                code: "fm::missing-author",
                message: "frontmatter has no 'author' field".to_string(),
            });
        }
        if !has_field(yaml, "title") {
            diags.push(Diagnostic {
                code: "fm::missing-title",
                message: "frontmatter has no 'title' field".to_string(),
            });
        }
    }
    diags
}

/// Does the frontmatter have `field`?
fn has_field(yaml: &str, field: &str) -> bool {
    yaml.lines().any(|l| l.starts_with(&format!("{field}:")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_frontmatter_produces_diagnostic() {
        let content = "# Hello\n\nSome content.\n";
        let diags = check_frontmatter(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-frontmatter");
    }
    #[test]
    fn missing_date_produces_diagnostic() {
        let content = "---\ntitle: Hello\nauthor: Jr\n---\n\nSome content.\n";
        let diags = check_frontmatter(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-date");
    }

    #[test]
    fn missing_author_produces_diagnostic() {
        let content = "---\ntitle: Hello\ndate: 2026-09-03\n---\n\nSome content.\n";
        let diags = check_frontmatter(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-author");
    }

    #[test]
    fn missing_title_produces_diagnostic() {
        let content = "---\nauthor: Jr\ndate: 2026-09-03\n---\n\nSome content.\n";
        let diags = check_frontmatter(content);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-title");
    }

    #[test]
    fn valid_frontmatter_produces_no_diagnostics() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\n---\n\nSome content.\n";
        let diags = check_frontmatter(content);
        assert!(diags.is_empty());
    }
}

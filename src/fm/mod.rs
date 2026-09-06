pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
}

pub struct Frontmatter<'a> {
    pub title: &'a str,
    pub author: &'a str,
    pub date: &'a str,
    pub lang: &'a str,
    pub tags: Vec<String>,
}

#[must_use]
pub fn fix_frontmatter(content: &str, fm: &Frontmatter<'_>) -> String {
    let mut block = String::from("---\n");

    if !fm.title.is_empty() {
        block.push_str(&format!("title: {}\n", fm.title));
    }
    if !fm.author.is_empty() {
        block.push_str(&format!("author: {}\n", fm.author));
    }
    if !fm.date.is_empty() {
        block.push_str(&format!("date: {}\n", fm.date));
    }
    if !fm.lang.is_empty() {
        block.push_str(&format!("lang: {}\n", fm.lang));
    }
    if !fm.tags.is_empty() {
        block.push_str("tags:\n");
        for tag in &fm.tags {
            block.push_str(&format!("  - {tag}\n"));
        }
    }
    block.push_str("---\n");
    format!("{block}\n{content}")
}

#[must_use]
pub fn check_frontmatter(content: &str, excluded: &[String]) -> Vec<Diagnostic> {
    if !content.starts_with("---") {
        return vec![Diagnostic {
            code: "fm::missing-frontmatter",
            message: "chapter has no frontmatter".to_string(),
        }];
    }
    let mut diags = Vec::new();

    let inner = content.trim_start_matches("---").trim_start_matches('\n');

    let Some(close) = inner.find("\n---") else {
        return vec![Diagnostic {
            code: "fm::unclosed-frontmatter",
            message: "unclosed YAML frontmatter fence".to_string(),
        }];
    };
    let yaml = &inner[..close];

    if !excluded.contains(&"date".to_string()) && !has_field(yaml, "date") {
        diags.push(Diagnostic {
            code: "fm::missing-date",
            message: "frontmatter has no 'date' field".to_string(),
        });
    }
    if !excluded.contains(&"author".to_string()) && !has_field(yaml, "author") {
        diags.push(Diagnostic {
            code: "fm::missing-author",
            message: "frontmatter has no 'author' field".to_string(),
        });
    }
    if !excluded.contains(&"title".to_string()) && !has_field(yaml, "title") {
        diags.push(Diagnostic {
            code: "fm::missing-title",
            message: "frontmatter has no 'title' field".to_string(),
        });
    }
    if !excluded.contains(&"lang".to_string()) && !has_field(yaml, "lang") {
        diags.push(Diagnostic {
            code: "fm::missing-lang",
            message: "frontmatter has no 'lang' field".to_string(),
        });
    }
    if !excluded.contains(&"tags".to_string()) && !has_field(yaml, "tags") {
        diags.push(Diagnostic {
            code: "fm::missing-tags",
            message: "frontmatter has no 'tags' field".to_string(),
        });
    }

    diags
}

/// Does the frontmatter have `field`?
fn has_field(yaml: &str, field: &str) -> bool {
    yaml.lines().any(|l| l.starts_with(&format!("{field}:")))
}

#[must_use]
pub fn fix_missing_tags(content: &str, tags: &[String]) -> String {
    let tags_yaml = if tags.is_empty() {
        "tags: []\n".to_string()
    } else {
        format!(
            "tags:\n{}\n",
            tags.iter()
                .map(|t| format!("  - {t}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    content.replacen("\n---", &format!("\n{tags_yaml}---"), 1)
}

#[must_use]
pub fn fix_missing_lang(content: &str, lang: &str) -> String {
    content.replacen("\n---", &format!("\nlang: {lang}\n---"), 1)
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags::infer_tags;

    #[test]
    fn missing_frontmatter_produces_diagnostic() {
        let content = "# Hello\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-frontmatter");
    }
    #[test]
    fn missing_date_produces_diagnostic() {
        let content =
            "---\ntitle: Hello\nauthor: Jr\nlang: en\ntags:\n -blog\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-date");
    }

    #[test]
    fn missing_author_produces_diagnostic() {
        let content =
            "---\ntitle: Hello\ndate: 2026-09-03\nlang: en\ntags:\n -blog\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-author");
    }

    #[test]
    fn missing_title_produces_diagnostic() {
        let content =
            "---\nauthor: Jr\ndate: 2026-09-03\nlang: en\ntags:\n -blog\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-title");
    }

    #[test]
    fn valid_frontmatter_produces_no_diagnostics() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\nlang: en\ntags:\n -blog\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert!(diags.is_empty());
    }

    #[test]
    fn unclosed_fm_fence_produces_diagnostic() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate:2026-09-03\nlang: en\ntags:\n -blog\n\nSome content\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::unclosed-frontmatter");
    }

    #[test]
    fn fix_missing_frontmatter_prepends_block() {
        let content = "# Hello\n\nSome content.\n";
        let path = "blog/rust/my-post.md";
        let tags = infer_tags(path);
        let fm = Frontmatter {
            title: "Hello",
            author: "Jr",
            date: "2026-09-03",
            lang: "en",
            tags,
        };
        let fixed = fix_frontmatter(content, &fm);
        assert!(fixed.starts_with("---\n"));
        assert!(fixed.contains("title: Hello"));
        assert!(fixed.contains("author: Jr"));
        assert!(fixed.contains("date: 2026-09-03"));
        assert!(fixed.contains("# Hello"));
        assert!(fixed.contains("tags:"));
        assert!(fixed.contains("  - blog"));
        assert!(fixed.contains("  - rust"));
    }

    #[test]
    fn missing_lang_produces_diagnostic() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\ntags:\n -blog\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-lang");
    }

    #[test]
    fn missing_tags_produces_diagnostic() {
        let content =
            "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\nlang: en\n---\n\nSome content.\n";
        let diags = check_frontmatter(content, &[]);
        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "fm::missing-tags");
    }

    #[test]
    fn fix_missing_tags_injects_into_existing_frontmatter() {
        let content =
            "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\nlang: en\n---\n\nSome content.\n";
        let tags = vec!["blog".to_string(), "rust".to_string()];
        let fixed = fix_missing_tags(content, &tags);
        assert!(fixed.contains("tags:"));
        assert!(fixed.contains("  - blog"));
        assert!(fixed.contains("  - rust"));
        let diags = check_frontmatter(&fixed, &[]);
        assert!(diags.is_empty());
    }

    #[test]
    fn extracts_frontmatter_block() {
        let content = "---\ntitle: Hello\nauthor: Jr\n---\n\nSome content.\n";
        let fm = extract_frontmatter(content);
        assert_eq!(fm, Some("---\ntitle: Hello\nauthor: Jr\n---\n".to_string()));
    }

    #[test]
    fn returns_none_when_no_frontmatter() {
        let content = "# Hello\n\nSome content.\n";
        let fm = extract_frontmatter(content);
        assert!(fm.is_none());
    }
}

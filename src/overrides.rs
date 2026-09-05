#[must_use]
fn parse_set(s: &str) -> Option<(&str, &str)> {
    let (key, value) = s.split_once('=')?;
    Some((key, value))
}

#[must_use]
pub fn apply_override(content: &str, key: &str, value: &str) -> String {
    content
        .lines()
        .map(|line| {
            if line.starts_with(&format!("{key}:")) {
                format!("{key}: {value}")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_pair() {
        let (key, value) = parse_set("title=My Title").unwrap();
        assert_eq!(key, "title");
        assert_eq!(value, "My Title");
    }

    #[test]
    fn parse_set_returns_none_for_invalid_input() {
        assert!(parse_set("notavalidpair").is_none());
    }

    #[test]
    fn parse_set_allows_empty_value() {
        let (key, value) = parse_set("title=").unwrap();
        assert_eq!(key, "title");
        assert_eq!(value, "");
    }

    #[test]
    fn applies_title_override() {
        let overrides = vec!["title=My Custom Title".to_string()];
        let mut fields = std::collections::HashMap::new();
        for s in &overrides {
            if let Some((k, v)) = parse_set(s) {
                fields.insert(k, v);
            }
        }
        assert_eq!(fields.get("title"), Some(&"My Custom Title"));
    }

    #[test]
    fn applies_override_to_existing_frontmatter() {
        let content =
            "---\ntitle: Old Title\nauthor: Jr\ndate: 2026-09-03\nlang: en\n---\n\nContent.\n";
        let result = apply_override(content, "title", "New Title");
        assert!(result.contains("title: New Title"));
        assert!(!result.contains("title: Old Title"));
    }
}

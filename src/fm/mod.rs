pub struct Diagnostic {
    pub code: &'static str,
    pub message: String,
}

pub fn check_frontmatter(content: &str) -> Vec<Diagnostic> {
    let has_frontmatter = content.starts_with("---");

    if !has_frontmatter {
        return vec![Diagnostic {
            code: "fm::missing-frontmatter",
            message: "chapter has no frontmatter".to_string(),
        }];
    }
    vec![]
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
}

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

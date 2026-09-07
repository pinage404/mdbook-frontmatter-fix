#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injects_comment_block_at_end_of_chapter() {
        let content = "---\ntitle: Hello\n---\n\n# Hello\n\nSome content.\n";
        let result = inject_comment_block(content);
        assert!(result.contains("<details>"));
        assert!(result.contains("<summary>Comments</summary>"));
        assert!(result.contains("</details>"));
        // comment block should be at the end
        let content_pos = result.find("Some content.").unwrap();
        let details_pos = result.find("<details>").unwrap();
        assert!(details_pos > content_pos);
    }
}

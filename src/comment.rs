#[must_use]
pub fn inject_comment_block(content: &str) -> String {
    if content.contains("<summary>Comments</summary>") {
        return content.to_string();
    }
    let block = "\n\n<details>\n<summary>Comments</summary>\n\n<!-- Add your questions or comments below -->\n\n</details>\n";
    format!("{content}{block}")
}

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

    #[test]
    fn skips_injection_if_comment_block_exists() {
        let content = "---\ntitle: Hello\n---\n\n# Hello\n\nContent.\n\n<details>\n<summary>Comments</summary>\n\n</details>\n";
        let result = inject_comment_block(content);
        assert_eq!(result.matches("<details>").count(), 1);
    }

    #[test]
    fn generates_giscus_block() {
        let config = CommentConfig {
            style: CommentStyle::Giscus {
                repo: "saylesss88/rust-gaps".to_string(),
                repo_id: "R_kgDO123".to_string(),
                category: "Comments".to_string(),
                category_id: "DIC_kwDO123".to_string(),
            },
        };
        let content = "---\ntitle: Hello\n---\n\n# Hello\n\nContent.\n";
        let result = inject_comment_block_with_config(content, &config);
        assert!(result.contains("giscus.app/client.js"));
        assert!(result.contains("saylesss88/rust-gaps"));
        assert!(result.contains("<details>"));
    }
}

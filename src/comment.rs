pub enum CommentStyle {
    Plain,
    Giscus {
        repo: String,
        repo_id: String,
        category: String,
        category_id: String,
    },
}

pub struct CommentConfig {
    pub style: CommentStyle,
}

#[must_use]
pub fn inject_comment_block_with_config(content: &str, config: &CommentConfig) -> String {
    if content.contains("<summary>Comments</summary>") {
        return content.to_string();
    }

    let inner = match &config.style {
        CommentStyle::Plain => "<!-- Add your questions or comments below -->\n".to_string(),
        CommentStyle::Giscus {
            repo,
            repo_id,
            category,
            category_id,
        } => {
            format!(
                r#"<script src="https://giscus.app/client.js"
    data-repo="{repo}"
    data-repo-id="{repo_id}"
    data-category="{category}"
    data-category-id="{category_id}"
    data-mapping="pathname"
    data-theme="dark"
    crossorigin="anonymous"
    async>
</script>
"#
            )
        }
    };

    let block = format!("\n\n<details>\n<summary>Comments</summary>\n\n{inner}\n</details>\n");
    format!("{content}{block}")
}

#[must_use]
pub fn inject_comment_block(content: &str) -> String {
    inject_comment_block_with_config(
        content,
        &CommentConfig {
            style: CommentStyle::Plain,
        },
    )
}

#[must_use]
pub fn parse_comment_config(content: &str) -> CommentConfig {
    let get = |key: &str| -> String {
        content
            .lines()
            .find(|l| l.starts_with(key))
            .and_then(|l| l.split('=').nth(1))
            .map(|v| v.trim().trim_matches('"').to_string())
            .unwrap_or_default()
    };

    let style = get("comment_style");
    if style == "giscus" {
        return CommentConfig {
            style: CommentStyle::Giscus {
                repo: get("giscus_repo"),
                repo_id: get("giscus_repo_id"),
                category: get("giscus_category"),
                category_id: get("giscus_category_id"),
            },
        };
    }

    CommentConfig {
        style: CommentStyle::Plain,
    }
}

#[must_use]
pub fn strip_comment_block(content: &str) -> String {
    if !content.contains("<summary>Comments</summary>") {
        return content.to_string();
    }

    let mut result = Vec::new();
    let mut in_block = false;

    for line in content.lines() {
        if line == "<details>" {
            // peek ahead — only skip if this is a comments block
            in_block = true;
            continue;
        }
        if in_block && line == "<summary>Comments</summary>" {
            continue;
        }
        if in_block && line == "</details>" {
            in_block = false;
            continue;
        }
        if in_block {
            continue;
        }
        result.push(line.to_string());
    }

    result.join("\n")
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

    #[test]
    fn parses_giscus_config_from_fmf_toml() {
        let content = "comment_style = \"giscus\"\ngiscus_repo = \"saylesss88/rust-gaps\"\ngiscus_repo_id = \"R_kgDO123\"\ngiscus_category = \"Comments\"\ngiscus_category_id = \"DIC_kwDO123\"\n";
        let config = parse_comment_config(content);
        match config.style {
            CommentStyle::Giscus { repo, repo_id, .. } => {
                assert_eq!(repo, "saylesss88/rust-gaps");
                assert_eq!(repo_id, "R_kgDO123");
            }
            CommentStyle::Plain => panic!("expected Giscus style"),
        }
    }

    #[test]
    fn strips_comment_block() {
        let content = "---\ntitle: Hello\n---\n\n# Hello\n\nContent.\n\n<details>\n<summary>Comments</summary>\n\n<!-- Add your questions or comments below -->\n\n</details>\n";
        let result = strip_comment_block(content);
        assert!(!result.contains("<details>"));
        assert!(!result.contains("<summary>Comments</summary>"));
        assert!(result.contains("Content."));
    }
}

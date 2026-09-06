#[must_use]
pub fn generate_toc(content: &str) -> String {
    let mut entries = Vec::new();

    for line in content.lines() {
        let (level, title) = if line.starts_with("### ") {
            (3, line.trim_start_matches("### ").trim())
        } else if line.starts_with("## ") {
            (2, line.trim_start_matches("## ").trim())
        } else {
            continue;
        };
        entries.push((level, title.to_string()));
    }

    let min_level = entries.iter().map(|(l, _)| l).min().copied().unwrap_or(2);

    entries
        .iter()
        .map(|(level, title)| {
            let anchor = title
                .to_lowercase()
                .replace(' ', "-")
                .replace(|c: char| !c.is_alphanumeric() && c != '-', "");

            let indent = "  ".repeat(level - min_level);
            format!("{indent}- [{title}](#{anchor})")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[must_use]
pub fn inject_toc(content: &str) -> String {
    let toc = generate_toc(content);
    if content.contains("## Table of Contents") {
        return content.to_string();
    }

    if toc.is_empty() {
        return content.to_string();
    }

    let toc_block = format!("## Table of Contents\n\n{toc}\n\n");

    if content.starts_with("---") {
        let inner = content.trim_start_matches("---").trim_start_matches('\n');
        if let Some(close) = inner.find("\n---") {
            let after_fm = &inner[close + 4..];
            let yaml = &inner[..close];
            return format!(
                "---\n{yaml}\n---\n\n{toc_block}{}",
                after_fm.trim_start_matches('\n')
            );
        }
    }
    format!("{toc_block}{content}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_toc_from_headings() {
        let content = "# My Chapter\n\n## First Section\n\nSome content.\n\n## Second Section\n\nMore content.\n";
        let toc = generate_toc(content);
        assert!(toc.contains("- [First Section](#first-section)"));
        assert!(toc.contains("- [Second Section](#second-section)"));
        assert!(!toc.contains("My Chapter")); // skip h1
    }

    #[test]
    fn generates_nested_toc() {
        let content = "# My Chapter\n\n## First Section\n\n### Subsection\n\nContent.\n";
        let toc = generate_toc(content);
        assert!(toc.contains("- [First Section](#first-section)"));
        assert!(toc.contains("  - [Subsection](#subsection)"));
    }

    #[test]
    fn injects_toc_after_frontmatter() {
        let content = "---\ntitle: Hello\n---\n\n## First Section\n\nContent.\n";
        let result = inject_toc(content);
        assert!(result.contains("## Table of Contents"));
        assert!(result.contains("- [First Section](#first-section)"));
        // TOC should come after frontmatter
        let fm_end = result.find("---\n\n").unwrap();
        let toc_pos = result.find("## Table of Contents").unwrap();
        assert!(toc_pos > fm_end);
    }

    #[test]
    fn skips_injection_if_toc_already_exists() {
        let content = "---\ntitle: Hello\n---\n\n## Table of Contents\n\n- [First](#first)\n\n## First\n\nContent.\n";
        let result = inject_toc(content);
        // should not add a second TOC
        assert_eq!(result.matches("## Table of Contents").count(), 1);
    }
}

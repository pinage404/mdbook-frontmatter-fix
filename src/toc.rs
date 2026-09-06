#[must_use]
pub fn generate_toc(content: &str) -> String {
    let mut lines = Vec::new();

    for line in content.lines() {
        let (level, title) = if line.starts_with("### ") {
            (3, line.trim_start_matches("### ").trim())
        } else if line.starts_with("## ") {
            (2, line.trim_start_matches("## ").trim())
        } else {
            continue;
        };

        let anchor = title
            .to_lowercase()
            .replace(' ', "-")
            .replace(|c: char| !c.is_alphanumeric() && c != '-', "");

        let indent = "  ".repeat(level - 2);
        lines.push(format!("{indent}- [{title}](#{anchor})"));
    }
    lines.join("\n")
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
}

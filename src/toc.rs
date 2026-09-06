#[must_use]
pub fn generate_toc(content: &str) -> String {
    let mut lines = Vec::new();

    for line in content.lines() {
        if line.starts_with("## ") {
            let title = line.trim_start_matches("## ").trim();
            let anchor = title
                .to_lowercase()
                .replace(' ', "-")
                .replace(|c: char| !c.is_alphanumeric() && c != '-', "");
            lines.push(format!("- [{title}](#{anchor})"));
        }
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
}

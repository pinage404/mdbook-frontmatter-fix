use std::path::Path;

/// Iterate over each line of `SUMMARY.md` as a `&str`
/// Find every `(something.md)` and return whats inside
/// `- [Intro](README.md)` -> `README.md`
/// Filters out section headers like `- [Part One]()`
#[must_use]
pub fn parse_summary(content: &str) -> Vec<String> {
    let mut paths = Vec::new();

    for line in content.lines() {
        if let Some(start) = line.find('(')
            && let Some(end) = line.find(')')
        {
            let path = &line[start + 1..end];
            if Path::new(path)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("md"))
            {
                let path = path.trim_start_matches("./");
                paths.push(path.to_string());
            }
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_chapter_paths_from_summary() {
        let content = "# Summary\n\n- [Introduction](README.md)\n- [Chapter One](chapter_one.md)\n";
        let paths = parse_summary(content);
        assert_eq!(paths, vec!["README.md", "chapter_one.md"]);
    }

    #[test]
    fn parses_chapter_paths_with_dot_slash_prefix() {
        let content = "# Summary\n\n- [Chapter One](./io/input_output.md)\n";
        let paths = parse_summary(content);
        assert_eq!(paths, vec!["io/input_output.md"]);
    }

    #[test]
    fn skips_section_headers_with_no_path() {
        let content = "# Summary\n\n# Error Handling\n- [Chapter](chapter.md)\n";
        let paths = parse_summary(content);
        assert_eq!(paths, vec!["chapter.md"]);
    }
}

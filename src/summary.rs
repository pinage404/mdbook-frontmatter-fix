#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_chapter_paths_from_summary() {
        let content = "# Summary\n\n- [Introduction](README.md)\n- [Chapter One](chapter_one.md)\n";
        let paths = parse_summary(content);
        assert_eq!(paths, vec!["README.md", "chapter_one.md"]);
    }
}

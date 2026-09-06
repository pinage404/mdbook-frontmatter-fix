#[must_use]
pub fn parse_language(content: &str) -> String {
    for line in content.lines() {
        if line.starts_with("language")
            && let Some(val) = line.split('=').nth(1)
        {
            return val.trim().trim_matches('"').to_string();
        }
    }
    "en".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_language_from_book_toml() {
        let content = "[book]\ntitle = \"My Book\"\nlanguage = \"en\"\n";
        let lang = parse_language(content);
        assert_eq!(lang, "en");
    }

    #[test]
    fn missing_language_defaults_to_en() {
        let content = "[book]\ntitle = \"My Book\"\nauthors = [\"Tom\"]\n";
        let lang = parse_language(content);
        assert_eq!(lang, "en");
    }

    #[test]
    fn parses_exclude_fields_from_book_toml() {
        let content = "[book]\ntitle = \"My Book\"\n\n[preprocessor.fmf]\nexclude_fields = [\"author\", \"lang\"]\n";
        let excluded = parse_excluded_fields(content);
        assert_eq!(excluded, vec!["author", "lang"]);
    }
}

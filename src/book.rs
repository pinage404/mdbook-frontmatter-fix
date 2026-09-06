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

#[must_use]
pub fn parse_excluded_fields(content: &str) -> Vec<String> {
    let mut in_fmf_section = false;
    let mut excluded = Vec::new();

    for line in content.lines() {
        if line.trim() == "[preprocessor.fmf]" {
            in_fmf_section = true;
            continue;
        }
        if line.starts_with('[') {
            in_fmf_section = false;
        }
        if in_fmf_section && line.trim().starts_with("exclude_fields") {
            // parse: exclude_fields = ["author", "lang"]
            if let Some(val) = line.split('=').nth(1) {
                let fields: Vec<String> = val
                    .trim()
                    .trim_start_matches('[')
                    .trim_end_matches(']')
                    .split(',')
                    .map(|s| s.trim().trim_matches('"').to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                excluded.extend(fields);
            }
        }
    }

    excluded
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

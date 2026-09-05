#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_language_from_book_toml() {
        let content = "[book]\ntitle = \"My Book\"\nlanguage = \"en\"\n";
        let lang = parse_language(content);
        assert_eq!(lang, "en");
    }
}

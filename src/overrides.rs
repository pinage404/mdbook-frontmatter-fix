#[must_use]
fn parse_set(s: &str) -> Option<(&str, &str)> {
    let (key, value) = s.split_once('=')?;
    Some((key, value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_pair() {
        let (key, value) = parse_set("title=My Title").unwrap();
        assert_eq!(key, "title");
        assert_eq!(value, "My Title");
    }

    #[test]
    fn parse_set_returns_none_for_invalid_input() {
        assert!(parse_set("notavalidpair").is_none());
    }

    #[test]
    fn parse_set_allows_empty_value() {
        let (key, value) = parse_set("title=").unwrap();
        assert_eq!(key, "title");
        assert_eq!(value, "");
    }
}

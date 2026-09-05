#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_pair() {
        let (key, value) = parse_set("title=My Title").unwrap();
        assert_eq!(key, "title");
        assert_eq!(value, "My Title");
    }
}

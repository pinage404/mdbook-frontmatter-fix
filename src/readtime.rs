#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn estimates_reading_time() {
        // average reading speed is ~200-250 wpm
        // 300 words should be ~2 minutes
        let content: String = "word ".repeat(300);
        let mins = estimate_reading_time(&content);
        assert_eq!(mins, 2);
    }
}

/// Average adult reading speed in words per minute.
const WPM: usize = 200;

#[must_use]
pub fn estimate_reading_time(content: &str) -> usize {
    let words = content.split_whitespace().count();
    let mins = words.div_ceil(WPM);
    mins.max(1) // minimum 1 minute
}

#[must_use]
pub fn inject_reading_time(content: &str) -> String {
    if content.contains("reading_time:") {
        return content.to_string();
    }

    let mins = estimate_reading_time(content);
    let field = format!("reading_time: ~{mins} min read");

    content.replacen("\n---", &format!("\n{field}\n---"), 1)
}

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

    #[test]
    fn injects_reading_time_into_frontmatter() {
        let content = "---\ntitle: Hello\nauthor: Tom\n---\n\n".to_string() + &"word ".repeat(300);
        let result = inject_reading_time(&content);
        assert!(result.contains("reading_time: ~2 min read"));
    }
}

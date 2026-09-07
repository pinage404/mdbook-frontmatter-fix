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
    if content.contains("⏱") {
        return content.to_string();
    }

    let mins = estimate_reading_time(content);
    let badge = format!("> ⏱ ~{mins} min read\n\n");

    if content.starts_with("---") {
        let inner = content.trim_start_matches("---").trim_start_matches('\n');
        if let Some(close) = inner.find("\n---") {
            let after_fm = inner[close + 4..].trim_start_matches('\n');
            let yaml = &inner[..close];
            return format!("---\n{yaml}\n---\n\n{badge}{after_fm}");
        }
    }

    format!("{badge}{content}")
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
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
    fn injects_badge_after_frontmatter() {
        let content = "---\ntitle: Hello\n---\n\n# Hello\n\nContent.\n";
        let result = inject_reading_time(content);
        assert!(result.contains("> ⏱"));
        // badge should come after frontmatter
        let fm_end = result.find("---\n\n").unwrap();
        let badge_pos = result.find("> ⏱").unwrap();
        assert!(badge_pos > fm_end);
    }
}

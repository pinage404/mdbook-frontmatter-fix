pub mod book;
pub mod comment;
pub mod error;
pub mod fm;
pub mod git;
pub mod html;
pub mod overrides;
pub mod summary;
pub mod tags;
pub mod toc;

#[allow(clippy::struct_excessive_bools)]
pub struct RunOptions<'a> {
    pub run_fm: bool,
    pub run_html: bool,
    pub fix: bool,
    pub dry_run: bool,
    pub lang: &'a str,
}

#[must_use]
pub fn run_on_chapter(
    path: &str,
    content: &str,
    opts: &RunOptions,
) -> (Vec<fm::Diagnostic>, Option<String>) {
    let mut diags = Vec::new();
    if opts.run_fm {
        diags.extend(fm::check_frontmatter(content, &[]));
    }
    if opts.run_html {
        diags.extend(html::check_html(content));
    }

    if !opts.fix && !opts.dry_run {
        return (diags, None);
    }

    let mut current = content.to_string();

    if diags.iter().any(|d| d.code == "fm::missing-lang") {
        current = fm::fix_missing_lang(&current, opts.lang);
    }

    if diags.iter().any(|d| d.code == "fm::missing-tags") {
        let t = tags::infer_tags(path);
        current = fm::fix_missing_tags(&current, &t);
    }

    let fixed = if current == content {
        None
    } else {
        Some(current)
    };

    (diags, fixed)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn dry_run_does_not_write_to_disk() {
        use std::fs;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.md");
        let original = "# Hello\n\nSome content.\n";
        fs::write(&path, original).unwrap();

        let opts = RunOptions {
            run_fm: true,
            run_html: false,
            fix: false,
            dry_run: true,
            lang: "en",
        };

        let _ = run_on_chapter("test.md", original, &opts);

        // run fix with dry_run = true
        // file should be unchanged
        let after = fs::read_to_string(&path).unwrap();
        assert_eq!(original, after);
    }

    #[test]
    fn dry_run_returns_fixed_content_without_writing() {
        let content = "---\ntitle: Hello\nauthor: Jr\ndate: 2026-09-03\n---\n\nSome content.\n";
        let opts = RunOptions {
            run_fm: true,
            run_html: false,
            fix: false,
            dry_run: true,
            lang: "en",
        };
        let (diags, fixed) = run_on_chapter("io/test.md", content, &opts);
        assert!(diags.iter().any(|d| d.code == "fm::missing-lang"));
        assert!(fixed.is_some());
        assert!(fixed.unwrap().contains("lang: en"));
    }
}

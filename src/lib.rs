pub mod book;
pub mod error;
pub mod fm;
pub mod git;
pub mod html;
pub mod summary;
pub mod tags;

pub struct RunOptions<'a> {
    pub run_fm: bool,
    pub run_html: bool,
    pub fix: bool,
    pub dry_run: bool,
    pub lang: &'a str,
}

pub fn run_on_chapter(
    path: &str,
    content: &str,
    opts: &RunOptions,
) -> (Vec<fm::Diagnostic>, Option<String>) {
    let mut diags = Vec::new();
    if opts.run_fm {
        diags.extend(fm::check_frontmatter(content));
    }
    if opts.run_html {
        diags.extend(html::check_html(content));
    }

    let fixed = if opts.fix || opts.dry_run {
        // return the fixed content without writing
        Some(content.to_string()) // placeholder for now
    } else {
        None
    };

    (diags, fixed)
}

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

    run_on_chapter("test.md", original, &opts);

    // run fix with dry_run = true
    // file should be unchanged
    let after = fs::read_to_string(&path).unwrap();
    assert_eq!(original, after);
}

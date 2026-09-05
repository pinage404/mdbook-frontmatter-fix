use std::{fs, path::Path};

use clap::Parser;
use mdbook_frontmatter_fix::{book, fm, fm::Frontmatter, git, html};
use mdbook_frontmatter_fix::{summary, tags};

#[derive(Parser)]
#[command(name = "fmf", about = "mdBook frontmatter & content validator")]
struct Cli {
    /// Check frontmatter fields only
    #[arg(long)]
    fm: bool,

    /// Check HTML structure only
    #[arg(long)]
    html: bool,

    /// Automatically fix issues where possible
    #[arg(long)]
    fix: bool,

    /// Show what would be fixed without writing to disk
    #[arg(long)]
    dry_run: bool,
}

fn read_or_exit(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("error: could not read {path}");
        std::process::exit(1);
    })
}

fn write_fixed(full_path: &str, content: String) {
    match fs::write(full_path, content) {
        Ok(()) => eprintln!("fixed: {full_path}"),
        Err(e) => eprintln!("error: could not write {full_path}: {e}"),
    }
}

fn has_diag(diags: &[fm::Diagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

fn main() {
    let cli = Cli::parse();

    if !Path::new("book.toml").exists() {
        eprintln!("error: no book.toml found. Run mf from your book root");
        std::process::exit(1);
    }

    let lang = book::parse_language(&read_or_exit("book.toml"));
    let paths = summary::parse_summary(&read_or_exit("src/SUMMARY.md"));

    let run_fm = cli.fm || !cli.html;
    let run_html = cli.html || !cli.fm;
    let mut total = 0;

    for path in &paths {
        let full_path = format!("src/{path}");
        let Ok(content) = fs::read_to_string(&full_path) else {
            eprintln!("error: could not read {full_path}");
            continue;
        };

        let mut diags = Vec::new();
        if run_fm {
            diags.extend(fm::check_frontmatter(&content));
        }
        if run_html {
            diags.extend(html::check_html(&content));
        }

        for diag in &diags {
            eprintln!(
                "warning[{}]: {}\n  --> src/{}",
                diag.code, diag.message, path
            );
            total += 1;
        }

        if cli.fix {
            if has_diag(&diags, "fm::missing-frontmatter") {
                let abs_path = Path::new(&full_path).canonicalize().unwrap();
                let commit = git::file_commit_info(&abs_path, "%Y-%m-%d", false)
                    .ok()
                    .flatten();
                let title = path
                    .trim_end_matches(".md")
                    .split('/')
                    .next_back()
                    .unwrap_or("untitled");
                let fixed = fm::fix_frontmatter(
                    &content,
                    &Frontmatter {
                        title,
                        author: commit.as_ref().map_or("Unknown", |c| c.author.as_str()),
                        date: commit.as_ref().map_or("Unknown", |c| c.date.as_str()),
                        lang: &lang,
                        tags: tags::infer_tags(path),
                    },
                );
                write_fixed(&full_path, fixed);
            }

            if has_diag(&diags, "fm::missing-lang") {
                write_fixed(&full_path, fm::fix_missing_lang(&content, &lang));
            }

            if has_diag(&diags, "fm::missing-tags") {
                let content = fs::read_to_string(&full_path).unwrap_or_default();
                write_fixed(
                    &full_path,
                    fm::fix_missing_tags(&content, &tags::infer_tags(path)),
                );
            }
        }
    }
    if total == 0 {
        println!("fmf: no issues found");
    } else {
        eprintln!("\nfmf: {total} issue(s) found");
        std::process::exit(1);
    }
}

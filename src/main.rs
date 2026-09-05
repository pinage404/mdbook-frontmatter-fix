use std::{fs, path::Path};

use clap::Parser;
use mdbook_frontmatter_fix::summary;
use mdbook_frontmatter_fix::{fm, html};

#[derive(Parser)]
#[command(name = "fmf", about = "mdBook frontmatter & content validator")]
struct Cli {
    /// Check frontmatter fields only
    #[arg(long)]
    fm: bool,

    /// Check HTML structure only
    #[arg(long)]
    html: bool,
}

fn main() {
    let cli = Cli::parse();

    if !Path::new("book.toml").exists() {
        eprintln!("error: no book.toml found. Run mf from your book root");
        std::process::exit(1);
    }

    let summary = fs::read_to_string("src/SUMMARY.md").unwrap_or_else(|_| {
        eprintln!("error: could not read src/SUMMARY.md");
        std::process::exit(1);
    });

    let run_fm = cli.fm || !cli.html;
    let run_html = cli.html || !cli.fm;
    let mut total = 0;

    let paths = summary::parse_summary(&summary);

    println!("found {} chapters", paths.len());
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
    }
    if total == 0 {
        println!("fmf: no issues found");
    } else {
        eprintln!("\nfmf: {total} issus(s) found");
        std::process::exit(1);
    }
}

use std::{fs, path::Path};

use clap::Parser;
use mdbook_frontmatter_fix::summary;

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

    let run_fm = cli.fm || !cli.html;
    let run_html = cli.html || !cli.fm;

    let summary = fs::read_to_string("src/SUMMARY.md").unwrap_or_else(|_| {
        eprintln!("error: could not read src/SUMMARY.md");
        std::process::exit(1);
    });

    let paths = summary::parse_summary(&summary);

    println!("found {} chapters", paths.len());
    for path in &paths {
        println!("  {path}");
    }

    println!("fm: {run_fm}, html: {run_html}");
}

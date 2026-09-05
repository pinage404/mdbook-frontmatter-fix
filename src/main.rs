use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[command(name = "mf", about = "mdBook frontmatter & content validator")]
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

    println!("fm: {run_fm}, html: {run_html}");
}

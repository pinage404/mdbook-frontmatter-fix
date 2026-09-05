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

    let run_fm = cli.fm || (!cli.fm && !cli.html);
    let run_html = cli.html || (!cli.fm && !cli.html);

    println!("fm: {run_fm}, html: {run_html}");
}

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    help_template = "{about-section}Author: {author-with-newline}\n{usage-heading}\n{usage}\n\n{all-args}"
)]
#[clap(author, version, about)]
/// Application configuration
struct Args {
    /// whether to be verbose
    #[arg(short = 'v')]
    verbose: bool,

    /// an optional name to greet
    #[arg()]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();
    if args.verbose {
        println!("DEBUG {args:?}");
    }
    println!(
        "Hello {} (from nixq)!",
        args.name.unwrap_or("world".to_string())
    );
}

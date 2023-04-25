use regex::Regex;
use clap::{arg, Parser, command};

/// Searches for patterns
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(short, long)]
    pattern: String
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let re = Regex::new(&pattern).unwrap();

    let quote = "Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. What do we seek through millions of pages?";

    for line in quote.lines() {
        match re.find(line) {
            Some(_) => println!("{}", line),
            None => ()
        }
    }
}

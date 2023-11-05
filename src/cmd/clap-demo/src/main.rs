use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Debug of start debug mode
    #[arg(short, long)]
    debug: bool,

    /// Name of the person to greet
    #[arg(short = 'a', long = "bb")]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

/// cargo add clap --features derive
fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
        // println!("Hello, world!");
    }
}

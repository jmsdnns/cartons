use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Some parameter
    something: Option<String>,

    /// This param is stored as a PathBuf
    /// https://doc.rust-lang.org/std/path/struct.PathBuf.html
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Allows using -v, -vv, or -vvv
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(something) = cli.something.as_deref() {
        println!("Something: {something}");
    } else {
        println!("Something: not provided")
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Config: {}", config_path.display());
    } else {
        println!("Config: not provided");
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.verbose {
        0 => println!("Verbose mode is off"),
        1 => println!("Verbose mode is kind of on"),
        2 => println!("Verbose mode is on"),
        3 => println!("Verbose mode is very on"),
        _ => println!("Verbose mode doesnt go that high..."),
    }

    // The rest of your program would go here
}

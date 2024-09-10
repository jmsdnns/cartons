use clap::{Parser, Subcommand};
use std::ffi::OsString;

#[derive(Debug, Parser)]
#[command(name = "kitty")]
#[command(about = "A CLI for managing kitties", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Pet a kitty
    #[command(arg_required_else_help = true)]
    Pet {
        /// The kitty to pet
        name: String,
    },
    /// Feed the kitties
    Feed {
        #[arg(required = true)]
        eats: Vec<OsString>,
    },
    /// Call a kitty over
    #[command(arg_required_else_help = true)]
    Call {
        /// The name to call them
        name: String,
    },
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Commands::Pet { name } => {
            println!("Petting {name}");
        }
        Commands::Feed { eats } => {
            let food_n_stuff = eats
                .iter()
                .enumerate()
                .map(|(i, item)| match i {
                    0 => item.to_str().unwrap().to_string(),
                    _ => format!(", {}", item.to_str().unwrap()),
                })
                .collect::<String>();
            println!("Feeding the kitties: {}", food_n_stuff);
        }
        Commands::Call { name } => {
            println!("Calling {name} to hang out");
        }
    }
}

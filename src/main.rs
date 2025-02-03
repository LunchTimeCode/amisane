use clap::{command, Parser, Subcommand};
use utils::Print;

mod init;
mod prepare;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = Cli::parse();
    let mut print = Print::default();
    match r.command {
        Some(Commands::Init {}) => {
            init::init(&mut print)?;
        }
        Some(Commands::Prepare {}) => {
            prepare::prepare(&mut print)?;
        }
        None => {
            println!("No command specified");
        }
    }

    print.flush();

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Inits the tooling and configuration file for amisane
    Init {},

    /// Prepares the the setup, similar to a baseline
    Prepare {},
}

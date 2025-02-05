mod utils;

use clap::{Parser, Subcommand};
use utils::*;

#[derive(Parser)]
#[command(name = "dotnix")]
#[command(version, about = "Manage Nixos")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Su,
    U,
    D,
    C,
    F,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Su) => {
            println!("Su cmd");
        }
        Some(Commands::D) => {
            nvd_diff();
        }
        _ => {
            git_add_all();
            build_system();
        }
    }
}

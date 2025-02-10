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

const GIT_ADD_ALL: &str = "git add -A";
const NVD_DIFF: &str = "nvd diff $(ls -d1v /nix/var/nix/profiles/system-*-link | tail -n 2)";
const BUILD_SYSTEM: &str = "nix build .#nixosConfigurations.nixos.config.system.build.toplevel";
const UPDATE_FLAKES: &str = "nix flake update";
const NIXOS_REBUILD: &str = "sudo nixos-rebuild switch --flake .#nixos";

fn main() {
    let cli = Cli::parse();
    let mut cmds = vec![];

    match &cli.command {
        Some(Commands::Su) => {
            cmds = [GIT_ADD_ALL, UPDATE_FLAKES, BUILD_SYSTEM, NIXOS_REBUILD].to_vec();
            git_add_all();
            update_flakes();
            build_system();
            nixos_rebuild();
        }
        Some(Commands::D) => {
            cmds = [NVD_DIFF].to_vec();
        }
        _ => {
            git_add_all();
            build_system();
        }
    }

    run_all(&cmds);
}

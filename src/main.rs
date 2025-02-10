use anyhow::{bail, Context, Ok, Result};
use clap::{Parser, Subcommand};
use std::process::Command;

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
const NIX_FMT: &str = "nix fmt";
const NVD_DIFF: &str = "nvd diff $(ls -d1v /nix/var/nix/profiles/system-*-link | tail -n 2)";
// Not needed??? const BUILD_SYSTEM: &str = "nix build .#nixosConfigurations.nixos.config.system.build.toplevel";
const UPDATE_FLAKES: &str = "nix flake update";
// TODO: handling sudo
const NIXOS_REBUILD: &str = "sudo nixos-rebuild switch --install-bootloader --flake .#nixos";

// Cleanup cmds
const NIX_GC: &str = "sudo nix-collect-garbage --delete-older-than 15d";
const NIX_STORE: &str = "sudo nix store optimise";

fn main() {
    let cli = Cli::parse();
    let mut cmds = vec![];
    let mut pkgs = vec![];

    match &cli.command {
        Some(Commands::Su) => {
            cmds = [GIT_ADD_ALL, NIX_FMT, UPDATE_FLAKES, NIXOS_REBUILD, NVD_DIFF].to_vec();
        }
        Some(Commands::U) => {
            cmds = [GIT_ADD_ALL, NIX_FMT, NIXOS_REBUILD, NVD_DIFF].to_vec();
        }
        Some(Commands::D) => {
            cmds = [NVD_DIFF].to_vec();
        }
        Some(Commands::F) => {
            cmds = [NIX_FMT].to_vec();
        }
        Some(Commands::C) => {
            cmds = [NIX_GC, NIX_STORE].to_vec();
        }
        _ => {
            println!("Not implemented yet.");
        }
    }

    run_nix_sh(&cmds, &pkgs);
}

pub fn run_nix_sh(cmds: &[&str], pkgs: &[&str]) -> Result<()> {
    let script = cmds.join(" && ");
    let inner_cmd = format!("bash -c 'set -x; {}'", script);

    let status = Command::new("nix-shell")
        .arg("-p")
        .args(pkgs)
        .arg("--run")
        .arg(inner_cmd)
        .current_dir("/home/atin/.dotfiles/")
        .status()
        .with_context(|| format!("Failed to execute."))?;

    if !status.success() {
        bail!("something failed: {}", status);
    }

    Ok(())
}

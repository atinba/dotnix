use anyhow::{bail, Context, Ok, Result};
use std::process::Command;

pub fn git_add_all() -> Result<()> {
    run_cmd("git add -A")?;
    Ok(())
}

pub fn nvd_diff() -> Result<()> {
    run_cmd("nvd diff $(ls -d1v /nix/var/nix/profiles/system-*-link | tail -n 2)")?;
    Ok(())
}

pub fn build_system() -> Result<()> {
    run_cmd("nix build .#nixosConfigurations.nixos.config.system.build.toplevel")?;
    Ok(())
}

pub fn update_flakes() -> Result<()> {
    run_cmd("nix flake update")?;
    Ok(())
}

pub fn nixos_rebuild() -> Result<()> {
    run_cmd("sudo nixos_rebuild switch --flake .#nixos")?;
    Ok(())
}

// TODO: add pkgs to nix-shell
pub fn run_all(cmds: &[&str]) -> Result<()> {
    let script = cmds.join(" && ");
    let inner_cmd = format!("bash -c 'set -x; {}'", script);

    let args = ["-p", "nvd", "--run", inner_cmd.as_str()];
    let status = Command::new("nix-shell")
        .args(args)
        .current_dir("/home/atin/.dotfiles/")
        .status()
        .with_context(|| format!("Failed to execute."))?;

    if !status.success() {
        bail!("something failed: {}", status);
    }

    Ok(())
}

pub fn run_cmd(cmd: &str) -> Result<()> {
    let mut args = vec!["-c", "-x"];
    args.push(cmd);
    let status = Command::new("bash")
        .args(args)
        .current_dir("/home/atin/.dotfiles/")
        .status()
        .with_context(|| format!("Failed to execute {}", cmd))?;

    if !status.success() {
        bail!("{} failed: {}", cmd, status);
    }

    Ok(())
}

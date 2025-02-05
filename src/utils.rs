use anyhow::{bail, Context, Ok, Result};
use std::process::Command;

pub fn git_add_all() -> Result<()> {
    run_cmd("git", &["add", "-A"], "git add -A")?;
    Ok(())
}

pub fn nvd_diff() -> Result<()> {
    run_cmd(
        "bash",
        &[
            "-c",
            "nvd diff $(ls -d1v /nix/var/nix/profiles/system-*-link | tail -n 2)",
        ],
        "nvd diff",
    )?;
    Ok(())
}

pub fn build_system() -> Result<()> {
    run_cmd(
        "nix",
        &[
            "build",
            ".#nixosConfigurations.nixos.config.system.build.toplevel",
        ],
        "`nix build`",
    )?;
    Ok(())
}

pub fn run_cmd(cmd: &str, args: &[&str], ctx_msg: &str) -> Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .current_dir("/home/atin/.dotfiles/")
        .status()
        .with_context(|| format!("Failed to execute {}", cmd))?;

    if !status.success() {
        bail!("{} failed: {}", ctx_msg, status);
    }

    Ok(())
}

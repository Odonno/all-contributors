use assert_cmd::Command;
use color_eyre::eyre::Result;
use std::path::Path;

pub fn create_cmd(path: &Path) -> Result<Command> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir(path);

    Ok(cmd)
}

pub fn add_contributions(path: &Path, login: &str, contributions_str: &str) -> Result<()> {
    let mut cmd = create_cmd(path)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg(login)
        .arg(contributions_str);

    cmd.assert().try_success()?;

    Ok(())
}

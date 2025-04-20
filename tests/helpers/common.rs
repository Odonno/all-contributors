use assert_cmd::Command;
use color_eyre::eyre::Result;
use std::path::Path;

pub fn create_cmd(path: &Path) -> Result<Command> {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))?;
    cmd.current_dir(path);

    Ok(cmd)
}

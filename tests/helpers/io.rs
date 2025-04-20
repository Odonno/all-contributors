use color_eyre::eyre::Result;
use std::{fs, path::Path};

pub fn copy_config_file(path: &Path) -> Result<()> {
    fs::copy(
        "test-files/.all-contributorsrc",
        path.join(".all-contributorsrc"),
    )?;
    Ok(())
}

use color_eyre::eyre::Result;
use std::{fs, path::Path};

pub fn copy_config_file(path: &Path) -> Result<()> {
    fs::copy(
        "test-files/.all-contributorsrc",
        path.join(".all-contributorsrc"),
    )?;
    Ok(())
}

pub fn copy_readme_file(path: &Path) -> Result<()> {
    fs::copy("test-files/readme.md", path.join("readme.md"))?;
    Ok(())
}

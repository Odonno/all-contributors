use color_eyre::eyre::{Result, eyre};
use std::fs;

pub fn main() -> Result<()> {
    const CONTRIBUTORS_CONFIG_FILENAME: &str = ".all-contributorsrc";

    let exists = fs::exists(CONTRIBUTORS_CONFIG_FILENAME)?;

    if !exists {
        return Err(eyre!(
            "The configuration file '{}' does not exist",
            CONTRIBUTORS_CONFIG_FILENAME
        ));
    }

    Ok(())
}

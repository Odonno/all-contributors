use color_eyre::eyre::{Result, eyre};
use std::fs;

use crate::models::ContributorsConfig;

pub fn main() -> Result<()> {
    const CONTRIBUTORS_CONFIG_FILENAME: &str = ".all-contributorsrc";

    let exists = fs::exists(CONTRIBUTORS_CONFIG_FILENAME)?;

    if !exists {
        return Err(eyre!(
            "The configuration file '{}' does not exist",
            CONTRIBUTORS_CONFIG_FILENAME
        ));
    }

    let config_str = fs::read_to_string(CONTRIBUTORS_CONFIG_FILENAME)?;
    let config: ContributorsConfig = serde_json::from_str(&config_str)?;

    for filename in config.files {
        let exists = fs::exists(&filename)?;

        if !exists {
            return Err(eyre!("The file '{}' does not exist", &filename));
        }

        // TODO
    }

    Ok(())
}

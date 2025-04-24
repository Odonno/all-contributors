use color_eyre::eyre::{Result, eyre};
use std::fs;

use crate::{
    constants::CONTRIBUTORS_CONFIG_FILENAME,
    models::{ContributorsConfig, ContributorsConfigInput},
};

pub fn retrieve_config() -> Result<ContributorsConfig> {
    let exists = fs::exists(CONTRIBUTORS_CONFIG_FILENAME)?;

    if !exists {
        return Err(eyre!(
            "The configuration file '{}' does not exist",
            CONTRIBUTORS_CONFIG_FILENAME
        ));
    }

    let config_str = fs::read_to_string(CONTRIBUTORS_CONFIG_FILENAME)?;
    let input: ContributorsConfigInput = serde_json::from_str(&config_str)?;

    input.try_into()
}

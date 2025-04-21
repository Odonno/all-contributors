use color_eyre::eyre::{Result, eyre};
use std::fs;

use crate::{constants::CONTRIBUTORS_CONFIG_FILENAME, models::ContributionKind};

pub fn main(login: Option<String>, contributions: Vec<ContributionKind>) -> Result<()> {
    let exists = fs::exists(CONTRIBUTORS_CONFIG_FILENAME)?;

    if !exists {
        return Err(eyre!(
            "The configuration file '{}' does not exist",
            CONTRIBUTORS_CONFIG_FILENAME
        ));
    }

    let Some(login) = login else {
        todo!("PREVIEW: A login must be provided, for now.")
    };

    if contributions.is_empty() {
        todo!("PREVIEW: At least one contribution must be provided, for now.");
    }

    Ok(())
}

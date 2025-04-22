use color_eyre::eyre::{OptionExt, Result, eyre};
use serde_json::Value;
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

    let config_str = fs::read_to_string(CONTRIBUTORS_CONFIG_FILENAME)?;
    let mut config_value: Value = serde_json::from_str(&config_str)?;

    let config_object = config_value
        .as_object_mut()
        .ok_or_eyre("The configuration file must be a JSON object")?;

    let contributors_value = config_object
        .get_mut("contributors")
        .ok_or_eyre("The 'contributors' property must exist in the configuration file")?;

    let contributors_array = contributors_value
        .as_array_mut()
        .ok_or_eyre("The 'contributors' property must be a JSON array in the configuration file")?;

    let mut login_found = false;

    for contributor_value in contributors_array {
        let contributor_object = contributor_value
            .as_object_mut()
            .ok_or_eyre("A contributor of the 'contributors' property must be a JSON object")?;

        let contributor_login = contributor_object.get("login").and_then(|v| v.as_str());

        if contributor_login == Some(&login) {
            login_found = true;

            let contributor_contribs = contributor_object
                .get_mut("contributions")
                .unwrap()
                .as_array_mut()
                .unwrap();

            for contribution in contributions {
                let contribution = contribution.to_string();
                let contribution_value = Value::String(contribution);

                if !contributor_contribs.contains(&contribution_value) {
                    contributor_contribs.push(contribution_value);
                }
            }

            break;
        }
    }

    if !login_found {
        todo!("Add a new item to the array...");
    }

    let output = serde_json::to_string_pretty(&config_value)?;
    fs::write(CONTRIBUTORS_CONFIG_FILENAME, output)?;

    Ok(())
}

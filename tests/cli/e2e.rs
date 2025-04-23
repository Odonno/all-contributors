use assert_fs::TempDir;
use color_eyre::eyre::{Error, Result};
use insta::{Settings, assert_snapshot};
use std::fs;

use crate::helpers::{
    InstaSettingsExtensions, add_contributions, copy_config_file, copy_simple_readme_file,
    create_cmd,
};

#[test]
fn add_new_contribution_to_existing_contributor_and_generate() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;
    copy_simple_readme_file(&temp_dir)?;

    add_contributions(temp_dir.path(), "tekacs", "ideas,bug")?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1").arg("generate");

    cmd.assert().try_success()?;

    let updated_readme = fs::read_to_string(temp_dir.join("readme.md"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(updated_readme);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn add_new_contributor_and_its_contributions_and_generate() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;
    copy_simple_readme_file(&temp_dir)?;

    add_contributions(temp_dir.path(), "odonno", "code,ideas")?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1").arg("generate");

    cmd.assert().try_success()?;

    let updated_readme = fs::read_to_string(temp_dir.join("readme.md"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(updated_readme);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

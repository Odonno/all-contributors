use assert_fs::TempDir;
use color_eyre::eyre::{Error, Result};
use insta::{Settings, assert_snapshot};
use std::fs;

use crate::helpers::{
    InstaSettingsExtensions, copy_config_file, copy_readme_file, create_cmd, get_stderr_str,
    get_stdout_str,
};

#[test]
fn fails_to_add_if_no_config_file() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1").arg("add");

    let assert = cmd.assert().try_failure()?;
    let stderr = get_stderr_str(assert)?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stderr);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn add_an_existing_contribution_should_do_nothing() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg("tekacs")
        .arg("code");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_config_file = fs::read_to_string(temp_dir.join(".all-contributorsrc"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_config_file);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn should_add_new_contribution_to_existing_contributor() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg("tekacs")
        .arg("ideas,bug");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_config_file = fs::read_to_string(temp_dir.join(".all-contributorsrc"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_config_file);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn should_add_new_contributor_and_its_contributions() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg("odonno")
        .arg("code,ideas");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_config_file = fs::read_to_string(temp_dir.join(".all-contributorsrc"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_config_file);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn should_add_new_contributor_and_its_contributions_and_then_apply_generate() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;
    copy_readme_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg("odonno")
        .arg("code,ideas")
        .arg("--generate");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_config_file = fs::read_to_string(temp_dir.join(".all-contributorsrc"))?;
    let updated_readme = fs::read_to_string(temp_dir.join("readme.md"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_config_file);
        assert_snapshot!(updated_readme);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

#[test]
fn should_add_new_contributor_and_its_contributions_and_then_apply_generate_alt() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;
    copy_readme_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1")
        .arg("add")
        .arg("odonno")
        .arg("code,ideas")
        .arg("--gen");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_config_file = fs::read_to_string(temp_dir.join(".all-contributorsrc"))?;
    let updated_readme = fs::read_to_string(temp_dir.join("readme.md"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_config_file);
        assert_snapshot!(updated_readme);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

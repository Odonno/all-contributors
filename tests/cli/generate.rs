use assert_fs::TempDir;
use color_eyre::eyre::{Error, Result};
use insta::{Settings, assert_snapshot};
use std::fs;

use crate::helpers::{
    InstaSettingsExtensions, copy_config_file, copy_readme_file, create_cmd, get_stderr_str,
    get_stdout_str,
};

#[test]
fn fails_to_generate_if_no_config_file() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1").arg("generate");

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
fn fails_to_generate_if_one_file_does_not_exist() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;

    let mut cmd = create_cmd(temp_dir.path())?;

    cmd.env("NO_COLOR", "1").arg("generate");

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
fn should_generate_with_same_input() -> Result<()> {
    let temp_dir = TempDir::new()?;

    copy_config_file(&temp_dir)?;
    copy_readme_file(&temp_dir)?;

    let mut cmd = create_cmd(&temp_dir)?;

    cmd.env("NO_COLOR", "1").arg("generate");

    let assert = cmd.assert().try_success()?;
    let stdout = get_stdout_str(assert)?;

    let updated_readme = fs::read_to_string(temp_dir.join("readme.md"))?;

    let mut insta_settings = Settings::new();
    insta_settings.add_cli_location_filter();
    insta_settings.bind(|| {
        assert_snapshot!(stdout);
        assert_snapshot!(updated_readme);
        Ok::<(), Error>(())
    })?;

    temp_dir.close()?;

    Ok(())
}

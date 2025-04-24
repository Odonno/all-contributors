use color_eyre::eyre::{Error, Result, eyre};

#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryType {
    GitHub,
    GitLab,
}

impl TryFrom<Option<String>> for RepositoryType {
    type Error = Error;

    fn try_from(input: Option<String>) -> Result<Self, Error> {
        input.as_deref().try_into()
    }
}

impl TryFrom<Option<&str>> for RepositoryType {
    type Error = Error;

    fn try_from(input: Option<&str>) -> Result<Self, Error> {
        match input {
            None | Some("github") => Ok(RepositoryType::GitHub),
            Some("gitlab") => Ok(RepositoryType::GitLab),
            Some(input) => Err(eyre!(
                "'{}' is not a currently supported repository type.",
                input
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn default_value_if_none_provided() -> Result<()> {
        let value: Option<String> = None;
        let value: RepositoryType = value.try_into()?;
        assert_eq!(value, RepositoryType::GitHub);

        Ok(())
    }

    #[test]
    fn should_recognize_github() -> Result<()> {
        let value: RepositoryType = Some("github").try_into()?;
        assert_eq!(value, RepositoryType::GitHub);

        Ok(())
    }

    #[test]
    fn should_recognize_gitlab() -> Result<()> {
        let value: RepositoryType = Some("gitlab").try_into()?;
        assert_eq!(value, RepositoryType::GitLab);

        Ok(())
    }

    #[test]
    fn fails_to_recognized_any_other_git_provider() -> Result<()> {
        let value: Result<RepositoryType> = Some("azure-devops").try_into();
        assert!(value.is_err());

        let err = value.unwrap_err();
        assert_snapshot!(err, @"'azure-devops' is not a currently supported repository type.");

        Ok(())
    }
}

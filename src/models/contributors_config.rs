use color_eyre::eyre::{Error, Result};

use super::{Contributor, ContributorsConfigInput, RepositoryType};

#[derive(Debug, Clone)]
pub struct ContributorsConfig {
    pub project_name: String,
    pub project_owner: String,
    pub repo_type: RepositoryType,
    pub repo_host: String,
    pub files: Vec<String>,
    pub image_size: u32,
    pub contributors_per_line: u8,
    pub contributors: Vec<Contributor>,
}

impl TryFrom<ContributorsConfigInput> for ContributorsConfig {
    type Error = Error;

    fn try_from(input: ContributorsConfigInput) -> Result<Self, Error> {
        let ContributorsConfigInput {
            project_name,
            project_owner,
            repo_type,
            repo_host,
            files,
            image_size,
            contributors_per_line,
            contributors,
            ..
        } = input;

        let repo_type = repo_type.try_into()?;
        let repo_host = repo_host.unwrap_or(String::from(default_repo_host(&repo_type)));
        let files = files.unwrap_or(vec![String::from("README.md")]);
        let image_size = image_size.unwrap_or(100);
        let contributors_per_line = contributors_per_line.unwrap_or(7);

        Ok(ContributorsConfig {
            project_name,
            project_owner,
            repo_type,
            repo_host,
            files,
            image_size,
            contributors_per_line,
            contributors,
        })
    }
}

fn default_repo_host(repo_type: &RepositoryType) -> &'static str {
    match repo_type {
        RepositoryType::GitHub => "https://github.com",
        RepositoryType::GitLab => "https://gitlab.com",
    }
}

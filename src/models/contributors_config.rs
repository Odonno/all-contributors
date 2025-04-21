use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsConfig {
    pub project_name: String,
    pub project_owner: String,
    repo_type: String,
    pub repo_host: String,
    pub files: Vec<String>,
    pub image_size: u32,
    commit: bool,
    commit_convention: String,
    pub contributors_per_line: u8,
    skip_ci: bool,
    pub contributors: Vec<Contributor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Contributor {
    pub login: String,
    pub name: String,
    pub avatar_url: String, // not in camelCase
    pub profile: String,
    pub contributions: Vec<String>,
}

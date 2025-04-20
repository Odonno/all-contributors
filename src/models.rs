use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsConfig {
    project_name: String,
    project_owner: String,
    repo_type: String,
    repo_host: String,
    pub files: Vec<String>,
    image_size: u32,
    commit: bool,
    commit_convention: String,
    contributors_per_line: u8,
    skip_ci: bool,
    contributors: Vec<Contributor>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Contributor {
    login: String,
    name: String,
    avatar_url: String, // not in camelCase
    profile: String,
    contributions: Vec<String>,
}

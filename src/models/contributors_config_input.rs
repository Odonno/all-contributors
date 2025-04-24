use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributorsConfigInput {
    pub project_name: String,
    pub project_owner: String,
    pub repo_type: Option<String>,
    pub repo_host: Option<String>,
    pub files: Option<Vec<String>>,
    pub image_size: Option<u32>,
    pub contributors_per_line: Option<u8>,
    pub contributors: Vec<super::Contributor>,
}

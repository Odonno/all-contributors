use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Contributor {
    pub login: String,
    pub name: String,
    pub avatar_url: String, // not in camelCase
    pub profile: String,
    pub contributions: Vec<String>,
}

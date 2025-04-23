use super::{ContributionKind, contribution_type_gen::get_contribution_type_from_kind};

#[derive(Debug, Clone)]
pub struct ContributionType {
    #[allow(dead_code)]
    pub kind: ContributionKind,
    #[allow(dead_code)]
    pub code: String,
    pub emoji: String,
    pub title: String,
    #[allow(dead_code)]
    pub description: String,
}

impl From<ContributionKind> for ContributionType {
    fn from(kind: ContributionKind) -> Self {
        get_contribution_type_from_kind(kind)
    }
}

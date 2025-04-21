use color_eyre::eyre::{Error, Result, eyre};

use super::{ContributionKind, contribution_type_gen::get_contribution_type_from_kind};

#[derive(Debug, Clone)]
pub struct ContributionType {
    pub kind: ContributionKind,
    pub code: String,
    pub emoji: String,
    pub title: String,
    pub description: String,
}

impl TryFrom<ContributionKind> for ContributionType {
    type Error = Error;

    fn try_from(kind: ContributionKind) -> Result<Self> {
        let Some(value) = get_contribution_type_from_kind(kind.clone()) else {
            return Err(eyre!("The contribution type '{}' does not exist.", kind));
        };

        Ok(value.clone())
    }
}

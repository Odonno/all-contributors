use color_eyre::eyre::{Error, Result};

use super::ContributionKind;

impl TryFrom<String> for ContributionKind {
    type Error = Error;

    fn try_from(code: String) -> Result<Self> {
        Self::try_from(code.as_ref())
    }
}

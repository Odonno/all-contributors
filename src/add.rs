use color_eyre::eyre::Result;

use crate::models::ContributionKind;

pub fn main(login: Option<String>, contributions: Vec<ContributionKind>) -> Result<()> {
    let Some(login) = login else {
        todo!("PREVIEW: A login must be provided, for now.")
    };

    if contributions.is_empty() {
        todo!("PREVIEW: At least one contribution must be provided, for now.");
    }

    Ok(())
}

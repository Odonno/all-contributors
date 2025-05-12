use crate::models::ContributionKind;
use clap::Args;

#[derive(Args, Debug)]
pub struct AddArgs {
    /// The username/login of the user
    pub login: Option<String>,
    /// The list of contributions of this user you want to (separator: ',')
    #[clap(value_delimiter = ',')]
    pub contributions: Vec<ContributionKind>,
    #[clap(long, aliases = vec!["gen"])]
    pub generate: bool,
}

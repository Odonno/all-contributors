use clap::{Parser, Subcommand, command};

use super::AddArgs;

#[derive(Parser, Debug)]
#[clap(name = "all-contributors", version, author = "Odonno")]
pub struct Args {
    #[command(subcommand)]
    pub command: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Use generate to read the contributors list from your `.all-contributorsrc` file
    /// and update the contributor tables specified by the files key.
    #[clap(aliases = vec!["gen"])]
    Generate,
    /// Add a new contributor as long as its contributions by updating the existing `.all-contributorsrc` file
    Add(AddArgs),
}

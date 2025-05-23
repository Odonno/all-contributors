use clap::Parser;
use cli::{Action, AddArgs, Args};
use color_eyre::{
    config::{HookBuilder, Theme},
    eyre::Result,
};
use std::env;

mod add;
mod cli;
mod config;
mod constants;
mod generate;
mod models;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    if env::var("NO_COLOR").unwrap_or(String::from("0")) == "1" {
        HookBuilder::default()
            .theme(Theme::new()) // disable colors
            .install()?;
    } else {
        color_eyre::install()?;
    }

    let args = Args::parse();

    match args.command {
        Action::Generate => generate::main(),
        Action::Add(AddArgs {
            login,
            contributions,
            generate,
        }) => add::main(login, contributions, generate).await,
    }
}

use clap::Parser;
use cli::{Action, Args};
use color_eyre::eyre::Result;

mod cli;

fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    match args.command {
        Action::Generate => println!("Hello, world!"),
    };

    Ok(())
}

use anyhow::Result;
use libpt::log::{debug, trace};

mod error;
mod config;
use config::{cli::Cli, Config};

fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let config = Config::load(cli.clone())?;

    todo!()
}

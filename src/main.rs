use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    error::*,
};

fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let cfg = Config::load(cli.clone())?;

    match cli.command {
        Commands::Changelog { .. } => {
            println!("{}", Changelog::build(&cfg)?);
            Ok(())
        }
        Commands::Release { .. } => {
            todo!()
        }
        Commands::Publish { .. } => {
            todo!()
        }
    }
}

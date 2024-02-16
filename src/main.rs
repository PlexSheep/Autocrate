use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    error::*,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let cfg = Config::load(cli.clone())?;

    match cli.command {
        Commands::Changelog { .. } => {
            println!("{}", Changelog::build(&cfg)?.to_string());
            // Ok(())
        }
        Commands::Release { .. } => {
            todo!()
        }
        Commands::Publish { .. } => {
            todo!()
        }
    };

    println!("foo");
    Ok(())
}

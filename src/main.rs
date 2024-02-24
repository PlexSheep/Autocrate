use autocrate::{
    changelog::*,
    config::{
        cli::{Cli, Commands},
        Config,
    },
    error::*,
    publish::publish,
    release::release,
    serverapi::ApiCollection,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::cli_parse();
    let cfg = Config::load(&cli)?;

    match cli.command {
        Commands::Changelog { .. } => {
            println!("{}", Changelog::build(&cfg)?);
        }
        Commands::Release { .. } => {
            // TODO: check if repo is dirty and create a commit with a given option
            let mut apis = ApiCollection::build(&cfg).await?;
            release(&cfg, &mut apis).await?;
        }
        Commands::Publish { .. } => {
            // TODO: check if repo is dirty and create a commit with a given option
            publish(&cfg).await?;
        }
        Commands::Version {} => {
            // TODO: version bump
            // TODO: version select interactive
            // TODO: version select automated
            todo!()
        }
        Commands::Init { .. } => {
            // TODO: create a basic autocrate yaml
            todo!()
        }
    };
    Ok(())
}

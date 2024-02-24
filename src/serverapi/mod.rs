use async_trait::async_trait;
use reqwest::ClientBuilder;

use crate::{
    config::{ApiType, Config},
    error::*,
    publish::PublishContext,
    release::ReleaseContext,
};

pub mod forgejo;
pub mod gitea;
pub mod github;
pub mod gitlab;
use forgejo::*;
use gitea::*;
use github::*;
use gitlab::*;

pub static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);
pub type ApiCollection = Vec<Box<dyn ServerApi>>;

// NOTE: in stable rust, traits can normally not contain async methods,
// see [here](https://stackoverflow.com/questions/65921581/how-can-i-define-an-async-method-in-a-trait).
// The `async_trait` crate can be used to work around this limitation.
#[async_trait]
pub trait ServerApi {
    async fn init(&mut self, cfg: &Config) -> Result<()>;
    async fn push_release(&mut self, rc: &ReleaseContext) -> Result<()>;
    async fn push_release_artifact(&mut self, rc: &ReleaseContext) -> Result<()>;
    async fn push_pkg(&mut self, pc: &PublishContext) -> Result<()>;
}

pub fn client_builder() -> ClientBuilder {
    ClientBuilder::new().user_agent(USER_AGENT)
}

pub async fn init_servers(cfg: &Config) -> Result<ApiCollection> {
    let mut collection: ApiCollection = ApiCollection::new();
    for api in &cfg.yaml.api {
        match api.1.server_type {
            ApiType::Gitea => {
                collection.push(Box::new(Gitea::build(api.1).await?));
            }
            ApiType::Gitlab => {
                collection.push(Box::new(Gitlab::build(api.1).await?));
            }
            ApiType::Github => {
                collection.push(Box::new(Github::build(api.1).await?));
            }
            ApiType::Forgejo => {
                collection.push(Box::new(Forgejo::build(api.1).await?));
            }
        }
    }
    for api in collection.iter_mut() {
        api.init(&cfg).await?;
    }
    Ok(collection)
}

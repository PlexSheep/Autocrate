use async_trait::async_trait;

use crate::{
    config::{packages::PackageType, ApiType, Config},
    error::*,
};

pub mod forgejo;
pub mod gitea;
pub mod github;
pub mod gitlab;
use forgejo::*;
use gitea::*;
use github::*;
use gitlab::*;

pub type ApiCollection = Vec<Box<dyn ServerApi>>;

// NOTE: in stable rust, traits can normally not contain async methods,
// see [here](https://stackoverflow.com/questions/65921581/how-can-i-define-an-async-method-in-a-trait).
// The `async_trait` crate can be used to work around this limitation.
#[async_trait]
pub trait ServerApi {
    async fn init(&mut self, cfg: &Config) -> Result<()>;
    async fn push_release(&mut self) -> Result<()>;
    async fn push_release_artifact(&mut self) -> Result<()>;
    async fn push_pkg(&mut self, pkg_type: PackageType) -> Result<()>;
}

pub async fn init_servers(cfg: &Config) -> Result<ApiCollection> {
    let mut collection: ApiCollection = ApiCollection::new();
    for api in &cfg.yaml.api {
        match api.1.server_type {
            ApiType::Gitea => {
                collection.push(Box::new(Gitea::build(cfg).await?));
            }
            ApiType::Gitlab => {
                collection.push(Box::new(Gitlab::build(cfg).await?));
            }
            ApiType::Github => {
                collection.push(Box::new(Github::build(cfg).await?));
            }
            ApiType::Forgejo => {
                collection.push(Box::new(Forgejo::build(cfg).await?));
            }
        }
    }
    Ok(collection)
}

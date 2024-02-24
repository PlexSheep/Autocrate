use std::ops::{Deref, DerefMut};

use async_trait::async_trait;
use reqwest::ClientBuilder;

use crate::{
    config::{self, ApiType, Config},
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

// NOTE: in stable rust, traits can normally not contain async methods,
// see [here](https://stackoverflow.com/questions/65921581/how-can-i-define-an-async-method-in-a-trait).
// The `async_trait` crate can be used to work around this limitation.
#[async_trait]
pub trait ServerApi {
    async fn init(&mut self, cfg: &Config) -> Result<()>;
    async fn push_release(&mut self, rc: ReleaseContext) -> Result<()>;
    async fn push_release_artifact(&mut self, rc: ReleaseContext) -> Result<()>;
    async fn push_pkg(&mut self, pc: PublishContext) -> Result<()>;
    fn get_cfg(&self) -> &config::Api;
}

pub(crate) type ApiCollectionInner = Vec<Box<dyn ServerApi>>;

pub struct ApiCollection {
    collection: ApiCollectionInner,
}

impl ApiCollection {
    pub async fn build(cfg: &Config) -> Result<Self> {
        let mut collection: ApiCollectionInner = ApiCollectionInner::new();
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
            api.init(cfg).await?;
        }
        Ok(ApiCollection { collection })
    }

    pub fn collection(&self) -> &ApiCollectionInner {
        self.collection.as_ref()
    }

    pub fn collection_mut(&mut self) -> &mut ApiCollectionInner {
        self.collection.as_mut()
    }
}

pub fn client_builder() -> ClientBuilder {
    ClientBuilder::new().user_agent(USER_AGENT)
}

// trait iimplementations for easy use of ApiCollection follow
impl IntoIterator for ApiCollection {
    fn into_iter(self) -> Self::IntoIter {
        self.collection.into_iter()
    }
    type Item = Box<dyn ServerApi>;
    type IntoIter = std::vec::IntoIter<Self::Item>;
}

impl Deref for ApiCollection {
    type Target = [Box<dyn ServerApi>];
    fn deref(&self) -> &Self::Target {
        &self.collection[..]
    }
}

impl DerefMut for ApiCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.collection[..]
    }
}

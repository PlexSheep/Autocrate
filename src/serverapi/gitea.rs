use async_trait::async_trait;

use super::{PublishContext, ReleaseContext, ServerApi};
use crate::{
    config::{Api, Config},
    error::*,
};
pub struct Gitea {
    cfg: Api,
}

#[async_trait]
impl ServerApi for Gitea {
    async fn init(&mut self, _cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self, _rc: &ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_release_artifact(&mut self, _rc: &ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, _pc: &PublishContext) -> Result<()> {
        todo!()
    }
}

impl Gitea {
    pub async fn build(api: &Api) -> Result<Self> {
        Ok(Self { cfg: api.clone() })
    }
}

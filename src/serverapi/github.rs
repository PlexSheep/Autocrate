use async_trait::async_trait;

use super::{PublishContext, ReleaseContext, ServerApi};
use crate::{
    config::{packages::PackageType, Api, ApiType, Config},
    error::*,
};
pub struct Github {
    cfg: Api,
}

#[async_trait]
impl ServerApi for Github {
    async fn init(&mut self, cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self, rc: &ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_release_artifact(&mut self, rc: &ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, pc: &PublishContext) -> Result<()> {
        todo!()
    }
}

impl Github {
    pub async fn build(api: &Api) -> Result<Self> {
        Ok(Self { cfg: api.clone() })
    }
}

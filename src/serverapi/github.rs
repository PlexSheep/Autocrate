use async_trait::async_trait;

use super::{PublishContext, ReleaseContext, ServerApi};
use crate::{
    config::{Api, Config},
    error::*,
};
pub struct Github {
    cfg: Api,
}

#[async_trait]
impl ServerApi for Github {
    async fn init(&mut self, _cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self, _rc: ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_release_artifact(&mut self, _rc: ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, _pc: PublishContext) -> Result<()> {
        todo!()
    }
    fn get_cfg(&self) -> &Api {&self.cfg}
}

impl Github {
    pub async fn build(api: &Api) -> Result<Self> {
        Ok(Self { cfg: api.clone() })
    }
}

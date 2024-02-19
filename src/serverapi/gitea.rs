use async_trait::async_trait;

use super::ServerApi;
use crate::{
    config::{packages::PackageType, ApiType, Config},
    error::*,
};
pub struct Gitea;

#[async_trait]
impl ServerApi for Gitea {
    async fn init(&mut self, cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, pkg_type: PackageType) -> Result<()> {
        todo!()
    }
}

impl Gitea {
    pub async fn build(cfg: &Config) -> Result<Self> {
        todo!()
    }
}

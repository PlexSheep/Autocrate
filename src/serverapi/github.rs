use async_trait::async_trait;

use super::ServerApi;
use crate::{
    config::{packages::PackageType, ApiType, Config},
    error::*,
};
pub struct Github;

#[async_trait]
impl ServerApi for Github {
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

impl Github {
    pub async fn build(cfg: &Config) -> Result<Self> {
        todo!()
    }
}

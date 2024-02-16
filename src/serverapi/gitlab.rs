use async_trait::async_trait;

use super::ServerApi;
use crate::{
    config::{ApiType, Config},
    error::*,
};
pub struct Gitlab;

#[async_trait]
impl ServerApi for Gitlab {
    async fn init(&mut self, cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self) -> Result<()> {
        todo!()
    }
}

impl Gitlab {
    pub async fn build(cfg: &Config) -> Result<Self> {
        todo!()
    }
}

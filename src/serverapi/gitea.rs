use async_trait::async_trait;

use super::ServerApi;
use crate::{config::Config, error::*};
pub struct Gitea;

#[async_trait]
impl ServerApi for Gitea {
    async fn init(&mut self, _cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self) -> Result<()> {
        todo!()
    }
}

impl Gitea {
    pub async fn build(_cfg: &Config) -> Result<Self> {
        todo!()
    }
}

use std::str::FromStr;

use crate::{
    config::{Api, Config},
    error::*,
    serverapi::{PublishContext, ReleaseContext, ServerApi},
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, Url,
};

pub struct Forgejo {
    cfg: Api,
    client: Client,
}

impl Forgejo {
    pub async fn build(api: &Api) -> Result<Self> {
        let mut headers: HeaderMap = HeaderMap::new();
        // may be left empty if we only do reads from publically accessible urls
        if api.auth.is_some() {
            let _ = headers.insert(
                "Authorization",
                // HeaderValue::from_str(api.auth.clone().unwrap().pass.get_pass()?.as_str())
                //     .map_err(ServerApiError::from)?,
                HeaderValue::from_str("hardcoded").map_err(ServerApiError::from)?
            );
        }
        let client = super::client_builder()
            .default_headers(headers)
            .build()
            .map_err(ServerApiError::from)?;
        Ok(Self {
            cfg: api.clone(),
            client,
        })
    }
}

#[async_trait]
impl ServerApi for Forgejo {
    async fn init(&mut self, _cfg: &Config) -> Result<()> {
        Ok(())
    }
    async fn push_release(&mut self, rc: ReleaseContext) -> Result<()> {
        let raw_url = format!(
            "{}/api/v1/repos/{}/{}/releases",
            self.cfg.endpoint, rc.username, rc.repository
        );
        let url = Url::parse(&raw_url).map_err(ServerApiError::from)?;
        let body = format!(
            r#"
        {{
            "body": "{}",
            "draft": {},
            "name": "{}",
            "prerelease": {},
            "tag_name": "{}",
            "target_commitish": "{}"
        }}
        "#,
            rc.text, rc.draft, rc.tag, rc.prerelease, rc.tag, rc.commit_sig
        );

        let request = self
            .client
            .post(url)
            .body(body)
            .build()
            .map_err(ServerApiError::from)?;
        let _response = self
            .client
            .execute(request)
            .await
            .map_err(ServerApiError::from)?;
        Ok(())
    }
    async fn push_release_artifact(&mut self, _rc: ReleaseContext) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, _pc: PublishContext) -> Result<()> {
        todo!()
    }
    fn get_cfg(&self) -> &Api {
        &self.cfg
    }
}

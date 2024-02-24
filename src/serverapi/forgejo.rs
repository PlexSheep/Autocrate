use super::ServerApi;
use crate::{
    config::{packages::PackageType, Api, ApiType, Config},
    error::*,
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue}, Client, Method, Request, RequestBuilder, Url
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
                HeaderValue::from_str(api.auth.clone().unwrap().pass.get_pass()?.as_str())
                    .map_err(ServerApiError::from)?,
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
    async fn init(&mut self, cfg: &Config) -> Result<()> {
        todo!()
    }
    async fn push_release(&mut self) -> Result<()> {
        let raw_url = format!(
            "{}/api/v1/repos/{user}/{repository}/releases",
            self.cfg.endpoint
        );
        let body = format!(r#"
        {{
            "body": "{text}",
            "draft": {draft},
            "name": "{name}",
            "prerelease": {prerelease},
            "tag_name": "{tag}",
            "target_commitish": "{commit_sig}"
        }}
        "#);

        let request = self.client.post().body(body).build()?;
        let response = self.client.execute(request).await?;
        Ok(())
    }
    async fn push_release_artifact(&mut self) -> Result<()> {
        todo!()
    }
    async fn push_pkg(&mut self, pkg_type: PackageType) -> Result<()> {
        todo!()
    }
}

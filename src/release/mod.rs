use crate::{config::Config, error::*, serverapi::ApiCollection};

pub struct ReleaseContext {
    pub draft: bool,
    pub prerelease: bool,
    pub username: String,
    pub repository: String,
    pub text: String,
    pub tag: String,
    pub commit_sig: String,
}

pub async fn release(cfg: &Config, _apis: &mut ApiCollection) -> Result<()> {
    // TODO: git tag
    // TODO: push to each server

    // TODO: release to each server
    tag(cfg).await?;
    todo!();

    // TODO: check that the release is made
    // TODO: generate artifacts
    // TODO: upload artifacts
    // TODO: upload artifact signatures

    Ok(())
}

async fn tag(_cfg: &Config) -> Result<()> {
    todo!()
}

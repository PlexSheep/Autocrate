use crate::{
    config::Config,
    error::*,
    git::{get_commit_sig, push, tag},
    serverapi::ApiCollection,
};

use futures::{self, stream::FuturesUnordered, StreamExt};

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ReleaseContext {
    pub draft: bool,
    pub prerelease: bool,
    pub username: String,
    pub repository: String,
    pub text: String,
    pub tag: String,
    pub commit_sig: String,
}

pub async fn release(cfg: &Config, apis: &mut ApiCollection) -> Result<()> {
    let tag: String = tag(cfg).await?;
    let commit_sig = get_commit_sig(cfg).await?;
    push(cfg).await?; // we assume that we only need to push the current branch to the singular
                      // remote, expecting that the repositories are somehow mirrored
                      // TODO: push to multiple remotes?

    let mut results = FuturesUnordered::new();
    for api in apis.iter_mut() {
        // TODO: check that auth exists
        let specific_rc = ReleaseContext {
            draft: true,
            prerelease: true,
            username: api
                .get_cfg()
                .clone()
                .auth
                .expect("no auth but trying to publish")
                .user,
            repository: api.get_cfg().repository.clone(),
            text: String::from("TODO: ADD TEXT VARIABLE SOMEHOW"),
            tag: tag.clone(),
            commit_sig: commit_sig.clone(),
        };
        results.push(api.push_release(specific_rc));
    }

    // wait for the release requests to finish
    while let Some(result) = results.next().await {
        if result.is_err() {
            return Err(result.unwrap_err());
        }
    }

    // TODO: check that the release is made
    // TODO: generate artifacts
    // TODO: upload artifacts
    // TODO: upload artifact signatures

    Ok(())
}

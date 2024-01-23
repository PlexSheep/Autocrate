#!/bin/bash
set -e
cargo check --all-features
echo ">>>>>>>> SELECT A NEW VERSION"
cargo ws version --no-git-commit -- -p autocrate
NEW_VERSION=$(cat Cargo.toml | rg '^\s*version\s*=\s*"([^"]*)"\s*$' -or '$1')
bash scripts/set_all_versions.sh $NEW_VERSION
git add -A
git commit -m "Release v$NEW_VERSION" || (echo ">>>>>>>> COMMIT FAILED OR THERE WAS NOTHING TO COMMIT"; sleep 5)
echo ">>>>>>>> SKIP!!!!!"
cargo ws version --amend -- -p autocrate
echo ">>>>>>>> PUBLISHING RELEASE FOR REPO"
bash scripts/release.sh
echo ">>>>>>>> PUBLISHING TO CRATES.IO NEXT"
sleep 10
cargo publish
echo ">>>>>>>> PUBLISHING TO CSCHERR.DE NEXT"
sleep 3
cargo publish --registry cscherr

# Autocrate

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/autocrate)
![Gitea Release](https://img.shields.io/gitea/v/release/PlexSheep/autocrate?gitea_url=https%3A%2F%2Fgit.cscherr.de)
![Gitea language count](https://img.shields.io/gitea/languages/count/PlexSheep/autocrate?gitea_url=https%3A%2F%2Fgit.cscherr.de)
[![cargo checks and tests](https://github.com/PlexSheep/Autocrate/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/Autocrate/actions/workflows/cargo.yaml)

![logo](data/media/autocrate.jpeg)

Autocrate simplifies the creation and maintenance of releases for your Rust
projects hosted on fancy git servers. By providing functionalities
like creating releases uploading artifacts, publishing crates, and managing changelogs,
Autocrate tries to streamline the release process. Although initially built for Forgejo,
I plan to extend support to other platforms such as GitHub and GitLab.

Autocrate can then be used in CI/CD, or in projects without
continuous integration to release software.

The software is built in Rust, and offers integration for Rust Projects with Cargo.
In the future, using other tools and specifying custom scripts will become possible.

* [Original Repository](https://git.cscherr.de/PlexSheep/Autocrate)
* [GitHub Mirror](https://github.com/PlexSheep/Autocrate)
* [crates.io](https://crates.io/crates/autocrate)
* [docs.rs](https://docs.rs/crate/autocrate/)

Take a look at the [scripts](./scripts) directory! [publish.sh](scripts/publish.sh)
and [release.sh](scripts/release.sh) are what I'm trying to get rid of.

## Features

* Create and update releases on your Git platform
* Publish crates to crates.io or other repositories
* Upload artifacts, including binaries and signatures alongside your releases
* Generate changelogs and release notes
* Configure with a simple yaml file

### Upcoming Features

Autocrate is still in pre-alpha, so the features listed above are still being
worked on. For the future, the following Features are planned:

* Support for platforms other than Forgejo
* Custom artifact build scripts
* Version bumping
* Interactive and scriptable CLI interface
* Publish a cargo workspace (that depends on it's own crates)

## Getting Started

Before getting started with Autocrate, make sure you have the necessary
prerequisites covered:

You can use `autocrate init` to set your workspace up with a basic
`.autocrate.yaml`.

* **Access to a supported Git Server** (such as [git.cscherr.de](https://git.cscherr.de)
and [codeberg.org](https://codeberg.org))

### Pre-requisites

* Git

#### If you want to compile it yourself

Install Rust, the officially recommended way is through [rustup.rs](https://rustup.rs/).
Your distribution may offer a Rust distribution in your package manager as an alternative

### Installing

Once the above pre-requisites are met, begin setting up Autocrate by running
the following command in your terminal:

``` $ cargo install autocrate ```

This command downloads from [crates.io](https://crates.io) and compiles Autocrate
locally, making it readily accessible through your command line interfaces.

### Configuring Autocrate

Create a YAML file named `.autocrate.yml` (or `.yaml`) in the root of your Git
repository. It should contain the following parameters (replace the placeholders):

| Parent               | Key          | Value                                                                            | Explanation                                                                  |
|----------------------|--------------|----------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| (root)               | `changelog`  | list of keys with this as parent (`git-log` etc)                                 | information on how a changelog is generated                                  |
| `changelog`          | `enable`     | `true`/`false`                                                                   | If false, no changelog will be generated                                     |
| `changelog`          | `git-log`    | `true`/`false`                                                                   | should a changelog be generated with `git log`?                              |
| (root)               | `uses`       | list of keys with this as parent (`cargo` etc)                                   | Marks features to be used by Autocrate                                       |
| `uses`               | `cargo`      | list of keys with this as parent (`publish` etc)                                 | tells us that your project uses cargo                                        |
| `cargo`              | `publish`    | `true`/`false`                                                                   | should we publish crates?                                                    |
| `cargo`              | `registries` | registries see [this](https://doc.rust-lang.org/cargo/reference/registries.html) | A list of registries we should publish to. If empty defaults to `crates.io`. |
| (root)               | `api`        | list of names, which each have the same keys                                     | defines the api we talk to                                                   |
| `api.NAME`           | `type`       | one of `gitea`,`github`,`gitlab` (currently only support for `gitea`             | Let's us know which api type we are talking to                               |
| `api.NAME`           | `endpoint`   | Base URL of the target server                                                    | Let's us know which api type we are talking to                               |
| `api.NAME`           | `auth`       | list of keys with this as parent (`user` and `pass`)                             | We probably need authentication on the target server                         |
| `api.NAME.auth`      | `user`       | a string                                                                         | Which user should we try to authenticate as                                  |
| `api.NAME.auth`      | `pass`       | contains either of `text`, `env` or `file`                                       | sets the secret for authentication with this server                          |
| `api.NAME.auth.pass` | `text`       | a authentication pass as clear text                                              | A secret for authentication of the server, probably a token                   |
| `api.NAME.auth.pass` | `env`        | env var which contains the token                                                 | A secret for authentication of the server, probably a token                   |
| `api.NAME.auth.pass` | `file`       | file var which contains the token                                                | A secret for authentication of the server, probably a token                   |

An example `.autocrate.yaml` could look like this:

```yaml
changelog:
  enable: true
  git-log: true

uses:
  cargo:
    publish: true
  # tokens are loaded from ~/.cargo/config.toml
    registries:
    - default
    - cscherr

api:
  github:
    type: github
    endpoint: https://github.com
    auth:
      user: PlexSheep
      pass:
        text: token_superimportantsecret
  cscherr:
    type: gitea
    endpoint: https://git.cscherr.de
    auth:
      user: PlexSheep
      pass:
        file: secrettoken.txt
```

After Autocrate has been bootstrapped, it will be released and published
with itself, so you can take a look at this repositories
[`.autocrate.yaml`](./.autocrate.yaml).

## Using Autocrate

After you have your workspace with a `.autocrate.yaml` file, you can:

* `autocrapte release` to create a release on your git server(s), optionally publishing too
* `autocrate publish` to publish your crate to the specified registries(s) (default is crates.io)
* `autocrate changelog` to generate a changelog since the last tag

## Licensing

Autocrate is free software.

The Autocrate project is distributed under the terms of the GPL-3
License. Please refer to [`LICENSE`](./LICENSE) for complete licensing details.

## Project status

The project has started recently and is currently in pre-alpha. Many features
are still missing or experimental

## Contributing

I'd be very happy to get contributions! Although the master repository is on
my self hosted git server, you're free to create issues, PRs and so on on
GitHub. If enough activity comes around, moving to GitHub Codeberg might be a 
good idea.

If you have any questions, use issues and discussions tabs or write me an email
to [software@cscherr.de](mailto:software@cscherr.de)

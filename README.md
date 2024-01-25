# Autocrate

![Project badge](https://img.shields.io/badge/language-Rust-blue.svg)
![Crates.io License](https://img.shields.io/crates/l/autocrate)
![Gitea Release](https://img.shields.io/gitea/v/release/PlexSheep/autocrate?gitea_url=https%3A%2F%2Fgit.cscherr.de)
![Gitea language count](https://img.shields.io/gitea/languages/count/PlexSheep/autocrate?gitea_url=https%3A%2F%2Fgit.cscherr.de)
[![cargo checks and tests](https://github.com/PlexSheep/Autocrate/actions/workflows/cargo.yaml/badge.svg)](https://github.com/PlexSheep/Autocrate/actions/workflows/cargo.yaml)

![logo](data/media/autocrate.jpeg)

**Disclaimer**: I've generated the Readme and logo with the help of so called AI
tools and modified them afterwards.

Autocrate simplifies the creation and maintenance of releases for your Rust
projects hosted on Gitea servers. By providing essential functionalities
like uploading artifacts, publishing crates, and managing changelogs,
Autocrate streamlines the release process, allowing developers to focus on
their work. Although initially built for Gitea, we plan to extend support
for additional platforms such as GitHub and GitLab.


* [Original Repository](https://git.cscherr.de/PlexSheep/Autocrate)
* [GitHub Mirror](https://github.com/PlexSheep/Autocrate)
* [crates.io](https://crates.io/crates/autocrate)
* [docs.rs](https://docs.rs/crate/autocrate/)

Take a look at the [scripts](./scripts) directory! [publish.sh](scripts/publish.sh)
and [release.sh](scripts/release.sh) are exactly what I'm trying to get rid of.

## Features

* Create and update releases on your Gitea server
* Publish crates to Cargo.rs
or other repositories directly from your Rust projects
* Upload artifacts, including documentation and binaries, alongside your releases
* Generate and maintain changelogs and release notes.

### Upcoming Features

My goal is to continuously enhance Autocrate to better serve the developer
community. Some planned improvements include supporting other popular hosting
platforms, enabling even greater flexibility and convenience.

## Getting Started

Before getting started with Autocrate, make sure you have the necessary
prerequisites covered:

* **A Rust Environment**: Install the latest stable Rust compiler and
associated tools via the official website: <https://www.rust-lang.org/>
* **Access to a Gitea Server** (such as [git.cscherr.de](https://git.cscherr.de)
and [codeberg.org](https://codeberg.org) (strictly speaking, uses a Gitea fork)

### Installing

Once the above pre-requisites are met, begin setting up Autocrate by running
the following command in your terminal:

``` $ cargo install autocrate ```

This command downloads from [crates.io](https://crates.io) and compiles Autocrate
locally, making it readily accessible through your command line interfaces.

### Configuring Autocrate

Create a YAML file named `.autocrate.yml` (or `.yaml`) in the root of your Git
repository. It should contain the following parameters (replace the placeholders):

| Parent          | Key          | Value                                                                            | Explanation                                                                  |
|-----------------|--------------|----------------------------------------------------------------------------------|------------------------------------------------------------------------------|
| (root)          | `changelog`  | list of keys with this as parent (`git-log` etc)                                 | information on how a changelog is generated                                  |
| `changelog`     | `enable`     | `true`/`false`                                                                   | If false, no changelog will be generated                                     |
| `changelog`     | `git-log`    | `true`/`false`                                                                   | should a changelog be generated with `git log`?                              |
| (root)          | `uses`       | list of keys with this as parent (`cargo` etc)                                   | Marks features to be used by Autocrate                                       |
| `uses`          | `cargo`      | list of keys with this as parent (`publish` etc)                                 | tells us that your project uses cargo                                        |
| `cargo`         | `publish`    | `true`/`false`                                                                   | should we publish crates?                                                    |
| `cargo`         | `registries` | registries see [this](https://doc.rust-lang.org/cargo/reference/registries.html) | A list of registries we should publish to. If empty defaults to `crates.io`. |
| (root)          | `api`        | list of names, which each have the same keys                                     | defines the api we talk to                                                   |
| `api.NAME`      | `type`       | one of `gitea`,`github`,`gitlab` (currently only support for `gitea`             | Let's us know which api type we are talking to                               |
| `api.NAME`      | `endpoint`   | Base URL of the target server                                                    | Let's us know which api type we are talking to                               |
| `api.NAME`      | `auth`       | list of keys with this as parent (`user` and `pass`)                             | We probably need authentication on the target server                         |
| `api.NAME.auth` | `user`       | a string                                                                         | Which user should we try to authenticate as                                  |
| `api.NAME.auth` | `pass`       | a string                                                                         | A secret for authentication o the server, probably a token                   |

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
    - crates.io
    - example.com

api:
  github:
    type: github
    endpoint: https://github.com
    auth:
      user: myUserName
      pass: token_superimportantsecret
  myserv:
    type: gitea
    endpoint: https://git.example.com
    auth:
      user: myUserName
      pass: importantsecrettoken
```

## Using Autocrate

TBD

## Licensing

Autocrate is free software.

The Autocrate project is distributed under the terms of the GPL-3
License. Please refer to [`LICENSE`](./LICENSE) for complete licensing details.

## Project status

The project has started recently and is currently in pre-alpha.

## Contributing

I'd be very happy to get contributions! Although the master repository is on
my self hosted git server, you're free to create issues, PRs and so on on
GitHub. If enough activity comes around, moving to GitHub might be a good idea.

If you have any questions, use issues and discussions tabs or write me an email
to [software@cscherr.de](mailto:software@cscherr.de)

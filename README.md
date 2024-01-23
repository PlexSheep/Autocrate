# autocrate

![logo](data/media/autocrate.jpeg)

Release Manager for Your Projects on Gitea, GitHub, and GitLab

## Purpose
Releasing my cargo workspaces was tedious, so I decided to create a project
that would help me with that. It should create a release with on the git server,
run whatever scripts specified by the maintainers, perhaps integrate with CI/CD,
and upload compiled and otherwise generated artifacts.

For Rust projects specifically, it should generate binaries and upload them to
the release of the git server. It should also provide the option to generate
the documentation and deploy it to a specified location. (The documentation of
published crates is automatically available on [docs.rs](https://docs.rs)).

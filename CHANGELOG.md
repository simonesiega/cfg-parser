# Changelog

All notable changes to CFG Parser are documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and version numbers follow [Semantic Versioning](https://semver.org/spec/v2.0.0.html) once tagged releases begin.

## [Unreleased]

### Added

- GitHub Actions checks for formatting, Clippy, the Rust 1.85 minimum, cross-platform locked tests, release smoke tests, documentation links, and Docker execution.
- Structured issue forms, pull-request template, code ownership, and automated dependency-update configuration.
- Documentation hub with dedicated setup, usage, grammar, architecture, Docker, and troubleshooting guides.
- Security policy, Code of Conduct, and release-process documentation.

### Changed

- Reworked the project README around a concise product overview, verified quick start, language reference, architecture summary, and project policies.
- Normalized documentation paths and the `CONTRIBUTING.md` filename.
- Clarified the implemented grammar for adjacent numbers, unary minus precedence, and mixed right-associative power/root chains.
- Added package metadata and hardened the Docker runtime configuration.

### Fixed

- Reject decimal literals outside the finite `f64` range instead of printing an infinite result.
- Reject unsupported Unicode whitespace without panicking on a non-character byte boundary.

## Release status

The crate manifest currently uses version `0.1.0`, but the repository has not published a tagged release. The first release section will be cut from **Unreleased** when a version is tagged.

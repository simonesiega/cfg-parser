# Release process

[← Documentation hub](../README.md) · [Changelog](../../CHANGELOG.md) · [Security](../../SECURITY.md)

CFG Parser does not currently have a tagged release or an automated publishing workflow. This checklist defines the manual process for the first and subsequent GitHub releases.

## Before release

1. Confirm the target behavior on the latest `master` branch.
2. Choose a version that follows [Semantic Versioning](https://semver.org/).
3. Update `version` in [`Cargo.toml`](../../Cargo.toml).
4. Move relevant entries from **Unreleased** in [`CHANGELOG.md`](../../CHANGELOG.md) into a dated version section.
5. Leave a fresh **Unreleased** section at the top of the changelog.
6. Confirm that `Cargo.lock` is current and contains no unintended dependency changes.

## Validate

Run the complete local release gate:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
cargo test --locked
cargo build --release --locked
node scripts/check-doc-links.mjs
git diff --check
```

If the release includes container changes, also run:

```bash
docker build -t cfg-parser:release-candidate .
docker run --rm --network none cfg-parser:release-candidate "2 + 3 * 4 ="
```

Confirm that repository CI passes on Linux, macOS, and Windows before creating the tag.

## Create the release commit and tag

Use an annotated tag matching the Cargo version:

```bash
git add Cargo.toml Cargo.lock CHANGELOG.md
git commit -m "chore(release): prepare vX.Y.Z"
git tag -a vX.Y.Z -m "CFG Parser vX.Y.Z"
git push origin master
git push origin vX.Y.Z
```

Never move or reuse a published version tag. If a release is wrong, prepare a new patch version.

## Publish on GitHub

Create a GitHub Release from the tag and use the matching changelog section as the release notes. Mark a version as a pre-release when its Semantic Versioning identifier contains a suffix such as `-alpha.1` or `-rc.1`.

CFG Parser is currently installed from Git and is not published to crates.io or a container registry. Do not claim availability through either channel until publishing automation and consumer validation have been added.

## After release

- Verify the release page and source archives.
- Test the documented `cargo install --git ... --tag vX.Y.Z --locked` path.
- Update installation documentation if distribution channels changed.
- Keep the security support table synchronized with the release policy.
- Open follow-up issues for deferred work rather than silently adding it to the release.

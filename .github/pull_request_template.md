## Summary

<!-- Explain what changed and why. Keep the pull request focused. -->

## Related issue

<!-- Use "Closes #123" when this pull request should close an issue. -->

## Behavior and grammar impact

<!-- List accepted/rejected examples and precedence changes, or write "None". -->

## Validation

<!-- Check what you ran. Explain any check you could not run. -->

- [ ] `cargo test --locked`
- [ ] `cargo build --release --locked`
- [ ] `cargo fmt --all -- --check` (Rust changes)
- [ ] `cargo clippy --all-targets --all-features --locked -- -D warnings` (Rust changes)
- [ ] `node scripts/check-doc-links.mjs` (documentation changes)
- [ ] Docker build and smoke test (Docker changes)

## Checklist

- [ ] The title and description explain the change and its rationale.
- [ ] The pull request contains no unrelated cleanup.
- [ ] Tests were added or updated for behavior changes.
- [ ] Grammar and usage documentation were updated for language changes.
- [ ] User-visible changes were added under `CHANGELOG.md` → **Unreleased**.
- [ ] Existing behavior remains compatible, or the breaking change is explicit.
- [ ] No credentials, personal data, editor state, or unrelated generated files are included.

## Screenshots

<!-- Add screenshots for visual documentation changes, or write "Not applicable". -->

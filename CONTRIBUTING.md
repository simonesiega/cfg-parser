# Contributing to CFG Parser

[← Project README](README.md) · [Documentation hub](docs/README.md) · [Code of Conduct](CODE_OF_CONDUCT.md)

Contributions are welcome when they are focused, testable, and consistent with the documented grammar.

## Before you start

- Read the [project overview](README.md#overview).
- Use the [documentation hub](docs/README.md) to find the canonical guide for your change.
- Search [existing issues](https://github.com/simonesiega/cfg-parser/issues).
- Discuss substantial syntax, precedence, or architecture changes before implementation.
- Report vulnerabilities through [`SECURITY.md`](SECURITY.md), never through a public issue.

## Finding work

Issues labeled [`good first issue`](https://github.com/simonesiega/cfg-parser/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) are intended to be bounded entry points.

Issues labeled [`help wanted`](https://github.com/simonesiega/cfg-parser/issues?q=is%3Aissue+is%3Aopen+label%3A%22help+wanted%22) are accepted tasks where external help would be useful.

These lists may be empty. Comment before beginning substantial work so scope and approach can be aligned.

## Development setup

Requirements:

- Git;
- Rust 1.85 or newer;
- Docker only when changing or validating the container workflow.

Clone your fork and enter the repository:

```bash
git clone https://github.com/<your-user>/cfg-parser.git
cd cfg-parser
git remote add upstream https://github.com/simonesiega/cfg-parser.git
```

Create a focused branch from `master`:

```bash
git fetch upstream
git switch master
git pull --ff-only upstream master
git switch -c fix/short-description
```

Suggested branch prefixes:

| Change | Prefix | Example |
| --- | --- | --- |
| Feature | `feat/` | `feat/add-modulo-operator` |
| Bug fix | `fix/` | `fix/reject-malformed-decimal` |
| Documentation | `docs/` | `docs/clarify-root-syntax` |
| Tests | `test/` | `test/add-precedence-cases` |
| Maintenance | `chore/` | `chore/update-actions` |

## Run locally

Evaluate an expression in release mode:

```bash
cargo run --release -- "2 + 3 * 4 ="
```

Debug builds enable parser logs. Set `RUST_LOG=off` when clean debug output is useful.

## Validation

Run the core repository checks before opening a pull request:

```bash
cargo test --locked
cargo build --release --locked
cargo run --quiet --release --locked -- "2 + 3 * 4 ="
```

For Rust changes, also format and review lints:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features --locked -- -D warnings
```

For documentation changes, validate local links and whitespace:

```bash
node scripts/check-doc-links.mjs
git diff --check
```

For Docker changes:

```bash
docker build -t cfg-parser:local .
docker run --rm cfg-parser:local "2 + 3 * 4 ="
```

CI enforces formatting and Clippy, tests the Rust 1.85 minimum, runs locked tests across Linux, macOS, and Windows, and smoke-tests the release binary, documentation links, and Docker image.

## Change guidelines

### Parser behavior

The [grammar specification](docs/guides/grammar.md) is the canonical language reference.

Any syntax change should include:

1. the intended production and precedence;
2. accepted examples;
3. rejected or boundary examples;
4. tests that protect the behavior;
5. synchronized user and grammar documentation;
6. an entry under [`CHANGELOG.md`](CHANGELOG.md) → **Unreleased**.

Keep tokenization, parsing, and mathematical validation conceptually separate.

Read the [architecture guide](docs/guides/architecture.md) before moving responsibilities between them.

### Tests

Tests should be deterministic and explain the behavior they protect. Prefer focused assertions over output-only smoke tests.

Include relevant cases:

- a normal accepted expression;
- precedence or associativity interactions;
- malformed syntax;
- arithmetic boundaries and error variants;
- regression input for a bug fix.

Do not weaken an existing assertion only to make a new implementation pass.

### Documentation

- Use relative links for repository files.
- Write commands from the directory stated by the guide.
- Quote shell-sensitive expressions.
- Keep `=` in complete expression examples.
- Describe current behavior, not planned behavior.
- Put detailed information in one canonical guide and link to it elsewhere.
- Give images descriptive alternative text.

Visual assets belong under [`docs/assets`](docs/assets/).

## Commits

Short, imperative commit messages are easiest to review. Conventional prefixes are encouraged:

```text
feat(parser): add operator support
fix(tokenizer): reject repeated decimal points
docs(grammar): clarify unary minus precedence
test(parser): cover right-associative powers
chore(ci): update Rust matrix
```

A pull request does not need one commit, but every commit should leave the branch understandable.

## Pull requests

Open a [pull request](https://github.com/simonesiega/cfg-parser/compare) against `master` and complete the repository template.

A review-ready pull request should:

- explain the problem and why the approach was chosen;
- stay focused on one change;
- link the relevant issue when one exists;
- list the validation actually performed;
- include tests for behavior changes;
- update documentation and the changelog when user-visible behavior changes;
- contain no secrets, unrelated generated files, or local editor state.

Maintainers may ask for a smaller scope or a different design. Review feedback should be resolved through follow-up commits or a clear discussion.

## Community

Participation is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).

Be respectful, specific, and constructive in issues, pull requests, and reviews.

## Contact

For contribution questions that do not fit an issue:

- GitHub: [@simonesiega](https://github.com/simonesiega)
- Email: [simonesiega1@gmail.com](mailto:simonesiega1@gmail.com)

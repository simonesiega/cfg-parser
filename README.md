<h1 align="center">CFG Parser</h1>

<p align="center">
  <strong>A compact Rust CLI for tokenizing, parsing, and evaluating arithmetic expressions with a hand-written recursive-descent parser.</strong>
</p>

<p align="center">
  <a href="#quick-start">Quick start</a> ·
  <a href="#installation">Installation</a> ·
  <a href="#usage">Usage</a> ·
  <a href="docs/README.md">Documentation</a> ·
  <a href="CONTRIBUTING.md">Contributing</a>
</p>

<p align="center">
  <a href="https://github.com/simonesiega/cfg-parser/actions/workflows/ci.yml"><img src="https://github.com/simonesiega/cfg-parser/actions/workflows/ci.yml/badge.svg?branch=master" alt="CI status" /></a>
  <img src="https://img.shields.io/badge/rust-2024%20edition-000000?logo=rust" alt="Rust 2024 edition" />
  <a href="https://github.com/simonesiega/cfg-parser/commits/master"><img src="https://img.shields.io/github/last-commit/simonesiega/cfg-parser" alt="Last commit" /></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/simonesiega/cfg-parser" alt="MIT license" /></a>
</p>

## Overview

**CFG Parser** is a small arithmetic-language implementation written in Rust. It tokenizes source text, parses it with a hand-written recursive-descent parser, and evaluates the expression according to an explicit context-free grammar.

```console
$ cfg-parser "2(3 + 4) ="
Result: 14.000
```

The project is intentionally compact and educational: precedence, associativity, implicit multiplication, exponentiation, roots, syntax validation, and mathematical errors are handled directly by the parser without a parser generator or intermediate AST.

For the complete language and implementation details, use the [documentation hub](docs/README.md).

## Quick start

Clone the repository and run an expression:

```bash
git clone https://github.com/simonesiega/cfg-parser.git
cd cfg-parser
cargo run --release -- "2 + 3 * 4 ="
```

Expected output:

```text
Result: 14.000
```

## Installation

Install directly from Git:

```bash
cargo install --git https://github.com/simonesiega/cfg-parser.git --locked
```

Then run:

```bash
cfg-parser "(1 + 2) * 3 ="
```

Building from source requires Rust 1.85 or newer.

See [Getting started](docs/guides/getting-started.md) for the complete native and Docker setup.

## Usage

Pass one complete expression to the CLI:

```bash
cfg-parser "2 ^ 8 ="
cfg-parser "(1 + 2)(3 + 4) ="
cfg-parser "27 $ 3 ="
```

Every complete expression ends with `=`.

| Syntax | Operation |
| --- | --- |
| `a + b` | Addition |
| `a - b` | Subtraction |
| `a * b` | Multiplication |
| `a / b` | Division |
| `-a` | Unary negation |
| `a ^ b` | Exponentiation |
| `a $ b` | `b`-th root of `a` |
| `( ... )` | Grouping |
| adjacency | Implicit multiplication |

Successful evaluations are printed to three decimal places.

See [CLI usage](docs/guides/usage.md) for input sources, shell quoting, output, exit behavior, and current CLI boundaries.

See [Grammar](docs/guides/grammar.md) for the canonical syntax, precedence, associativity, implicit-multiplication rules, and numeric semantics.

## Architecture overview

CFG Parser uses a two-stage pipeline:

```text
source text
    │
    ▼
Tokenizer
    │
    ▼
Vec<Token>
    │
    ▼
recursive-descent parser
    │
    ├── syntax validation
    ├── precedence / associativity
    └── evaluation
    │
    ▼
f64 or CalcError
```

Parsing and evaluation happen in the same traversal; the current implementation does not construct an intermediate abstract syntax tree.

See [Architecture](docs/guides/architecture.md) for the tokenizer, grammar layers, error flow, numeric model, and test boundaries.

## Documentation

Start with the [documentation hub](docs/README.md), or jump directly to:

- [Getting started](docs/guides/getting-started.md) — installation and verification.
- [CLI usage](docs/guides/usage.md) — command-line behavior and input sources.
- [Grammar](docs/guides/grammar.md) — language syntax and semantics.
- [Architecture](docs/guides/architecture.md) — tokenizer, parser, evaluator, and errors.
- [Docker](docs/guides/docker.md) — image and Compose workflow.
- [Troubleshooting](docs/guides/troubleshooting.md) — common setup and runtime problems.
- [Release process](docs/guides/releasing.md) — versioning, validation, tagging, and publication.

Additional contributor, security, changelog, and project-policy documentation is indexed from the documentation hub.

## Local development

```bash
git clone https://github.com/simonesiega/cfg-parser.git
cd cfg-parser
cargo test --locked
```

See [Contributing](CONTRIBUTING.md) for the full development, validation, testing, documentation, and pull-request workflow.

## Security

Report suspected vulnerabilities privately rather than through a public issue.

See [`SECURITY.md`](SECURITY.md) for supported versions, the security model, and private reporting channels.

## License

CFG Parser is licensed under the [MIT License](LICENSE).

## Contributors

<p align="center">
  <a href="https://github.com/simonesiega/cfg-parser/graphs/contributors">
    <img src="https://contrib.rocks/image?repo=simonesiega/cfg-parser&max=24&columns=12" alt="CFG Parser contributors" />
  </a>
</p>

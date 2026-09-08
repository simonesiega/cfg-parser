# CFG Parser Documentation Hub

[← Project README](../README.md) · [Contributing](../CONTRIBUTING.md) · [Security](../SECURITY.md)

This documentation is organized by task. The root README provides the project overview and quick start, while the guides below are the canonical references for setup, CLI behavior, grammar, architecture, containers, troubleshooting, and releases.

[Grammar](guides/grammar.md) owns language semantics, [Architecture](guides/architecture.md) owns implementation structure, [CLI usage](guides/usage.md) owns public input and output behavior, and [Troubleshooting](guides/troubleshooting.md) owns common setup and runtime failures.

## Start here

- **Running CFG Parser for the first time?** Follow [Getting started](guides/getting-started.md).
- **Looking for commands, examples, or input behavior?** Read [CLI usage](guides/usage.md).
- **Studying the arithmetic language?** Open the [Grammar specification](guides/grammar.md).
- **Understanding or changing the parser?** Read [Architecture](guides/architecture.md), then [Contributing](../CONTRIBUTING.md).
- **Running without a local Rust toolchain?** Follow the [Docker guide](guides/docker.md).
- **Something is not working?** Use [Troubleshooting](guides/troubleshooting.md).
- **Preparing a versioned release?** Follow the [Release process](guides/releasing.md).

## Documentation index

### Using CFG Parser

| Guide | Use it when |
| --- | --- |
| [Getting started](guides/getting-started.md) | Installing Rust, cloning the repository, installing the binary, or verifying the setup. |
| [CLI usage](guides/usage.md) | Learning input sources, operators, output behavior, examples, and shell-sensitive syntax. |
| [Docker](guides/docker.md) | Building or running CFG Parser in a container instead of using a local Rust toolchain. |
| [Troubleshooting](guides/troubleshooting.md) | Resolving setup, shell quoting, input, build, or evaluation problems. |

### Language and internals

| Guide | Covers |
| --- | --- |
| [Grammar](guides/grammar.md) | Formal productions, precedence, associativity, implicit multiplication, and semantic constraints. |
| [Architecture](guides/architecture.md) | Input resolution, tokenization, recursive descent, direct evaluation, errors, numeric behavior, and test boundaries. |

### Development and project policies

| Document | Covers |
| --- | --- |
| [Contributing](../CONTRIBUTING.md) | Repository setup, development workflow, validation, documentation rules, and pull-request expectations. |
| [Code of Conduct](../CODE_OF_CONDUCT.md) | Community behavior, conduct reporting, and enforcement expectations. |
| [Security](../SECURITY.md) | Supported versions and private vulnerability disclosure. |
| [Changelog](../CHANGELOG.md) | Released behavior and current unreleased changes. |
| [Release process](guides/releasing.md) | Version updates, validation, tagging, publication, and post-release checks. |
| [License](../LICENSE) | MIT license terms. |

### Assets

| Resource | Purpose |
| --- | --- |
| [`assets/`](assets/) | Visual assets used by the documentation. |
| [Diagram assets](assets/diagrams/README.md) | Tokenizer and parser diagrams, editable sources, and regeneration instructions. |

The PNG diagrams are generated from the adjacent Graphviz `.dot` source files.

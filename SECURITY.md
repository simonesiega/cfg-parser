# Security Policy

[← Project README](README.md) · [Contributing](CONTRIBUTING.md) · [Documentation hub](docs/README.md)

This policy explains which versions of CFG Parser receive security fixes, how to report vulnerabilities privately, and which behaviors fall within the project's security model.

## Supported versions

CFG Parser has not yet published a tagged stable release. Security fixes are applied to the current `master` branch.

| Version | Support |
| --- | --- |
| Current `master` branch | Supported |
| Older commits or third-party builds | Best effort only |

## Reporting a vulnerability

Please **do not open a public issue** for a suspected vulnerability.

Report security issues privately through either channel:

- [GitHub private vulnerability reporting](https://github.com/simonesiega/cfg-parser/security/advisories/new)
- [simonesiega1@gmail.com](mailto:simonesiega1@gmail.com)

When possible, include:

- a concise description of the issue;
- expected and actual behavior;
- the impact and a realistic attack scenario;
- minimal reproduction steps or a proof of concept;
- operating system, Rust version, and invocation method;
- a suggested mitigation, if you have one.

Do not include secrets or unrelated personal data.

Please allow time to investigate and coordinate a fix before public disclosure. No fixed response or remediation deadline is promised, but reports will be handled as promptly as reasonably possible.

## Security model

CFG Parser is a local, one-shot command-line program.

It:

- accepts expression text from process arguments or `CFGPARSER_INPUT`;
- uses a built-in expression when neither source is present;
- performs no intentional network requests;
- requires no credentials;
- reads no user files;
- writes no user files;
- evaluates expressions with Rust `f64` operations rather than executing expression text as code.

The Docker runtime similarly requires no ports, volumes, credentials, or network access.

## Relevant security reports

Security reports are especially useful for behavior that could:

- execute code or shell commands from an expression;
- read or modify files unexpectedly;
- trigger memory unsafety during safe, supported usage;
- cause an avoidable denial of service from a bounded expression;
- expose environment values other than the documented input variable;
- compromise the build, dependency, CI, or container supply chain.

Ordinary parse errors, unsupported syntax, floating-point rounding, and documented mathematical restrictions are generally bugs rather than vulnerabilities.

Use the public bug-reporting workflow for those cases unless they have a credible security impact.

## Dependency and build hygiene

The repository:

- commits `Cargo.lock` for reproducible application builds;
- uses `--locked` in CI and documented release checks;
- limits default GitHub Actions permissions to read-only repository contents;
- uses Dependabot for Cargo, Docker, and GitHub Actions updates;
- builds a separate minimal runtime container stage;
- runs the runtime container as an unprivileged user.

Dependency updates still require review. Automated update tooling is not a substitute for evaluating upstream changes.

# Getting started

[← Documentation hub](../README.md) · [CLI usage](usage.md) · [Docker](docker.md) · [Troubleshooting](troubleshooting.md)

This guide covers the supported ways to build, run, install, and verify CFG Parser.

## Choose a workflow

| Workflow | Requirement | Best for |
| --- | --- | --- |
| Cargo | Rust 1.85+ with Cargo | Development and local installation |
| Docker | Docker Desktop or Docker Engine | Running the CLI without installing Rust |

CFG Parser uses the Rust 2024 edition, so the native workflow requires Rust 1.85 or newer.

## Cargo

### Check the toolchain

Verify that Rust and Cargo are available:

```bash
rustc --version
cargo --version
```

If either command is missing, install Rust with [rustup](https://www.rust-lang.org/tools/install).

### Clone the repository

```bash
git clone https://github.com/simonesiega/cfg-parser.git
cd cfg-parser
```

### Run the parser

```bash
cargo run -- "2 + 3 * 4 ="
```

Expected output:

```text
Result: 14.000
```

The first run downloads and compiles the project's dependencies. Later runs reuse Cargo's build cache.

For an optimized build:

```bash
cargo run --release -- "2 + 3 * 4 ="
```

## Install the command

CFG Parser is not currently distributed as a published crate. Install the binary directly from the Git repository:

```bash
cargo install --git https://github.com/simonesiega/cfg-parser.git --locked
```

Then run it from any directory:

```bash
cfg-parser "(1 + 2) * 3 ="
```

Cargo normally installs binaries into:

```text
$HOME/.cargo/bin
```

If `cfg-parser` is not found after installation, make sure that directory is on `PATH`. A terminal restart may also be required after a new rustup installation.

To replace an existing installation with the current repository version:

```bash
cargo install \
  --git https://github.com/simonesiega/cfg-parser.git \
  --locked \
  --force
```

## Windows

PowerShell can run quoted expressions directly:

```powershell
cargo run -- "27 $ 3 ="
```

Single quotes are also useful for fixed expressions containing shell-sensitive characters:

```powershell
cfg-parser '27 $ 3 ='
```

The default Windows MSVC Rust toolchain requires the Microsoft C++ build tools. If linking fails because the required linker is unavailable, install the **Desktop development with C++** workload from Visual Studio Build Tools.

See [Troubleshooting](troubleshooting.md) for setup and build failures.

## Docker

The repository Docker image uses the same Rust 1.85 baseline as the native workflow and produces a small Debian runtime image.

Build it from the repository root:

```bash
docker build -t cfg-parser:local .
```

Run an expression:

```bash
docker run --rm cfg-parser:local "2(3 + 4) ="
```

Expected output:

```text
Result: 14.000
```

The repository also provides a hardened Compose configuration:

```bash
docker compose build
docker compose run --rm cfgparser "2(3 + 4) ="
```

The Compose service runs without networking, with a read-only root filesystem, and with privilege escalation disabled.

See [Docker](docker.md) for the complete image design, Compose behavior, security boundaries, rebuilds, and Docker-specific troubleshooting.

## Verify the checkout

Run the test suite:

```bash
cargo test --locked
```

Build the optimized binary using the committed lockfile:

```bash
cargo build --release --locked
```

Then smoke-test the release configuration:

```bash
cargo run --quiet --release --locked -- "2 + 3 * 4 ="
```

Expected output:

```text
Result: 14.000
```

These checks verify the current source, locked dependencies, test suite, and release build without modifying `Cargo.lock`.

## Next steps

- [CLI usage](usage.md) — input sources, operators, output behavior, and examples.
- [Grammar](grammar.md) — formal productions, precedence, associativity, and language semantics.
- [Architecture](architecture.md) — tokenizer, recursive descent, direct evaluation, errors, and test boundaries.
- [Docker](docker.md) — container image and Compose configuration.
- [Troubleshooting](troubleshooting.md) — common setup, shell, build, and runtime failures.
- [Release process](releasing.md) — versioning, validation, tagging, and publication.

# Troubleshooting

[← Documentation hub](../README.md) · [Getting started](getting-started.md) · [CLI usage](usage.md) · [Docker](docker.md)

Use this guide to diagnose setup, shell, input, evaluation, and container problems. Language semantics remain canonical in [Grammar](grammar.md), and container design remains canonical in [Docker](docker.md).

## Quick checks

From the repository root, run:

```bash
cargo test --locked
cargo build --release --locked
cargo run --quiet --release --locked -- "2 + 3 * 4 ="
```

Expected output from the final command:

```text
Result: 14.000
```

If all three commands succeed, continue with the symptom that matches the failing invocation.

## Setup and installation

### `cargo` is not recognized

**Fix:** Verify the toolchain:

```bash
rustc --version
cargo --version
```

If either command is missing, install Rust with [rustup](https://www.rust-lang.org/tools/install), restart the terminal, and retry.

Cargo-installed binaries normally live in `$HOME/.cargo/bin`; on Windows they live in `%USERPROFILE%\.cargo\bin`. Ensure that directory is on `PATH`.

See [Getting started](getting-started.md).

### Linking fails on Windows

**Fix:** Install the **Desktop development with C++** workload from Visual Studio Build Tools, then retry:

```powershell
cargo build --release --locked
```

The default Windows MSVC Rust toolchain requires Microsoft's linker and C++ build tools.

### The installed command is old

**Fix:** Replace the Git installation:

```bash
cargo install \
  --git https://github.com/simonesiega/cfg-parser.git \
  --locked \
  --force
```

Then check which executable is being resolved:

```bash
which cfg-parser
```

In PowerShell, use `Get-Command cfg-parser`. Remove an obsolete binary or correct `PATH` if the wrong executable appears first.

### The repository checkout is stale

**Fix:** Preserve local work, then update and retest:

```bash
git pull
cargo test --locked
```

Do not discard uncommitted work merely to update the checkout.

## Input and shell behavior

### The expression ends unexpectedly

**Cause:** A complete formula must end with `=`.

**Fix:** Add the terminator:

```bash
cfg-parser "2 + 3 ="
```

See [Grammar: formula termination](grammar.md#formula-termination).

### Tokens appear after `=`

**Cause:** The parser rejects anything after the formula terminator.

**Fix:** Remove the trailing content. Use `2 + 3 =`, not `2 + 3 = 5`.

See [Grammar: formula termination](grammar.md#formula-termination).

### The shell changes the expression

**Cause:** `$`, `*`, and parentheses may be interpreted before CFG Parser receives them.

**Fix:** Quote the complete expression.

| Shell | Example |
| --- | --- |
| Bash or Zsh | `cfg-parser '27 $ 3 ='` |
| PowerShell | `cfg-parser '27 $ 3 ='` |
| Command Prompt | `cfg-parser "27 $ 3 ="` |

If quoted and unquoted invocations behave differently, the shell—not the parser—is changing the input.

See [CLI usage: shell quoting](usage.md#shell-quoting).

### `CFGPARSER_INPUT` is ignored

**Cause:** Command-line arguments have higher priority than the environment variable, and whitespace-only environment values are ignored.

**Fix:** Invoke `cfg-parser` without expression arguments and provide a non-empty value:

```bash
CFGPARSER_INPUT='27 $ 3 =' cfg-parser
```

See [CLI usage: input selection](usage.md#input-selection).

### The built-in expression runs unexpectedly

**Cause:** Neither command-line arguments nor a non-empty `CFGPARSER_INPUT` value reached the process.

**Fix:** Pass one quoted argument or set the environment variable in the same shell that launches CFG Parser.

The fallback is intentional and is not an interactive prompt. See [CLI usage: built-in demonstration](usage.md#built-in-demonstration).

### Debug lines appear around the result

**Cause:** Development builds initialize debug logging.

**Fix:** Use a release build for clean output:

```bash
cargo run --quiet --release -- "2 + 3 ="
```

Or disable debug logging explicitly:

```bash
RUST_LOG=off cargo run -- "2 + 3 ="
```

In PowerShell, set `$env:RUST_LOG = 'off'` before running Cargo.

See [CLI usage: development logging](usage.md#development-logging).

## Parsing and evaluation

### An operator gives an unexpected result

**Fix:** Check the canonical precedence and associativity table, then add explicit parentheses if the intended grouping is unclear.

Two rules that commonly surprise users are:

- unary negation binds before `^` and `$`;
- powers and roots associate from the right and share one precedence level.

See [Grammar: precedence and associativity](grammar.md#precedence-and-associativity).

### Adjacent values multiply unexpectedly

**Cause:** Supported adjacent token pairs represent implicit multiplication.

**Fix:** Separate expressions with an operator, or use explicit `*` when multiplication is intended.

For example, `2 3 =` evaluates as `2 * 3`. See [Grammar: implicit multiplication](grammar.md#implicit-multiplication) for every accepted adjacency.

### A root expression fails

**Fix:** Confirm that the expression uses `base $ index` and that the index is valid for the base.

The evaluator rejects zero indices, fractional indices for negative bases, and even integer roots of negative bases. Use [Grammar: n-th roots](grammar.md#n-th-roots) for exact semantics and examples.

### A decimal literal is rejected

**Fix:** Use ASCII digits with no more than one decimal point. Scientific notation is not supported.

Accepted shapes include `42`, `3.14`, `.5`, and `1.`. See [Grammar: numbers](grammar.md#numbers).

### Parentheses are reported as unmatched

**Fix:** Balance every opening and closing parenthesis and place the final `=` after the complete expression.

```text
(2 + 3) * 4 =
```

If grouping is still unclear, simplify nested powers, roots, or implicit multiplication before rebuilding the expression.

### Division by zero fails

**Cause:** Division by zero is a mathematical error. A zero root index uses the same error path.

**Fix:** Change the divisor or root index. See [Grammar: operator semantics](grammar.md#operator-semantics).

### An unsupported character is rejected

**Fix:** Use the ASCII-oriented token vocabulary documented in [Grammar: lexical tokens](grammar.md#lexical-tokens).

Variables, functions, scientific notation, and unary `+` are not part of the current language.

### The displayed result appears rounded

**Cause:** Successful CLI output always uses three digits after the decimal point, while evaluation uses `f64` internally.

**Fix:** Treat the printed value as display output rather than an exact decimal representation. CFG Parser does not currently expose a higher-precision or machine-readable output mode.

See [CLI usage: output](usage.md#output).

### A failure prints no result line

**Cause:** Invalid input exits through the process error path instead of writing `Result: ...` to standard output.

**Fix:** Check the non-zero exit status and diagnostic output. Automation should not depend on exact error wording.

See [CLI usage: exit behavior](usage.md#exit-behavior).

## Docker

### Docker cannot connect to the daemon

**Fix:** Start Docker Desktop or the system Docker service, then verify connectivity:

```bash
docker info
```

On Windows, wait until Docker Desktop reports that the engine is ready.

### Docker runs old code

**Fix:** Rebuild the image:

```bash
docker build -t cfg-parser:local .
```

If an ordinary rebuild still reuses stale layers:

```bash
docker build --no-cache -t cfg-parser:local .
```

For Compose, use `docker compose build --no-cache`.

### The container exits immediately

**Cause:** CFG Parser is a one-shot CLI, not a server. It evaluates one expression and exits.

**Fix:** Run a new container for each expression. `--rm` removes the stopped container automatically; omit it only when the stopped container itself must be inspected.

### Compose runs the built-in expression

**Cause:** `docker compose up` starts the service without expression arguments, so the CLI uses its normal input fallback.

**Fix:** Use `docker compose run` when passing an expression:

```bash
docker compose run --rm cfgparser "2 + 3 ="
```

### The image build says the lockfile is outdated

**Cause:** The Dockerfile builds with `cargo build --release --locked`.

**Fix:** Update and commit `Cargo.toml` and `Cargo.lock` together outside Docker, then rebuild. The image build intentionally does not rewrite dependency metadata.

### Network access fails under Compose

**Cause:** The service sets `network_mode: none`.

**Fix:** No change is needed for normal evaluation. Use plain `docker run` only if a separate debugging workflow genuinely requires network access.

### A write fails under Compose

**Cause:** The service sets `read_only: true`.

**Fix:** Normal evaluation requires no writes. Use plain `docker run` without `--read-only` only for an intentional writable debugging workflow.

See [Docker](docker.md) for image design, Compose restrictions, inspection commands, and container boundaries.

## Reporting a reproducible problem

Before opening a bug report:

1. reproduce the problem on the latest `master` branch;
2. run `cargo test --locked`;
3. record the exact expression and expected result;
4. record whether input came from arguments or `CFGPARSER_INPUT`;
5. record the operating system and invocation method;
6. record `rustc --version` and `cargo --version`;
7. reduce the problem to the smallest reproducible example;
8. search the [existing issues](https://github.com/simonesiega/cfg-parser/issues).

Then open the repository's [bug report form](https://github.com/simonesiega/cfg-parser/issues/new?template=bug-report.yml).

Do not report a vulnerability publicly. Follow the private process in [`SECURITY.md`](../../SECURITY.md).

# CLI usage

[← Documentation hub](../README.md) · [Getting started](getting-started.md) · [Grammar](grammar.md) · [Troubleshooting](troubleshooting.md)

CFG Parser evaluates one complete arithmetic expression per process. This guide defines how the CLI receives input and reports results. The [Grammar specification](grammar.md) is the canonical reference for operators, precedence, associativity, and mathematical restrictions.

## Invocation

When installed:

```bash
cfg-parser "2 + 3 * 4 ="
```

From a source checkout:

```bash
cargo run --release -- "2 + 3 * 4 ="
```

The `--` separates Cargo arguments from arguments passed to CFG Parser.

Every complete expression must end with `=`. Quote the expression so the shell passes operators and parentheses to the program unchanged.

## Input selection

CFG Parser chooses the first available input source in this order:

| Priority | Source | Used when |
| ---: | --- | --- |
| 1 | Command-line arguments | At least one argument is present. |
| 2 | `CFGPARSER_INPUT` | No arguments are present and the variable is not empty. |
| 3 | Built-in demonstration | Neither of the preceding sources provides input. |

Once selected, the expression follows the same tokenization and parsing rules regardless of its source.

### Command-line arguments

The recommended form is one quoted argument:

```bash
cfg-parser "(3 + 5) / 2 ="
```

If multiple arguments are supplied, CFG Parser joins them with single spaces before tokenization. For example:

```bash
cfg-parser 3 + 5 =
```

becomes:

```text
3 + 5 =
```

The quoted form is more portable because shells may interpret `$`, `*`, and parentheses.

### Environment variable

When no command-line arguments are present, set `CFGPARSER_INPUT` to the complete expression.

Bash or Zsh:

```bash
CFGPARSER_INPUT='27 $ 3 =' cfg-parser
```

PowerShell:

```powershell
$env:CFGPARSER_INPUT = '27 $ 3 ='
cfg-parser
```

An empty or whitespace-only value is ignored. Command-line arguments always take precedence over the environment variable.

```bash
CFGPARSER_INPUT='1 + 1 =' cfg-parser "6 * 7 ="
```

This evaluates the command-line expression and prints `Result: 42.000`.

### Built-in demonstration

Running the command without arguments and without a non-empty environment value evaluates the built-in demonstration expression:

```bash
cfg-parser
```

This fallback is not an interactive prompt. CFG Parser does not read expressions from standard input.

## Shell quoting

Prefer one quoted expression:

```bash
cfg-parser '27 $ 3 ='
cfg-parser '(1 + 2)(3 + 4) ='
```

Single quotes are suitable for fixed expressions in Bash, Zsh, and PowerShell. Use double quotes in Windows Command Prompt.

If an expression changes before reaching the parser, follow [Troubleshooting: the shell changes the expression](troubleshooting.md#the-shell-changes-the-expression).

## Output

A successful evaluation writes one line to standard output:

```text
Result: 14.000
```

Results are displayed with three digits after the decimal point. Evaluation itself uses `f64`; output formatting does not change the internal value.

The CLI does not provide JSON or another machine-readable output format.

## Exit behavior

A successful evaluation returns a successful process status. Invalid syntax or arithmetic returns a non-zero status and no result line.

Failures include malformed input, unsupported characters, incomplete formulas, invalid arithmetic, and checked numeric limits. Exact language failure conditions belong to [Grammar](grammar.md); corrective steps belong to [Troubleshooting](troubleshooting.md).

Error text is intended for people and is not a stable machine-readable interface. Automation should use the process status rather than parse error wording.

## Development logging

Debug builds initialize `env_logger` and may write parser diagnostics to standard error. For clean output from a source checkout, use a release build:

```bash
cargo run --quiet --release -- "2 + 3 ="
```

To retain a debug build while suppressing logs, set `RUST_LOG=off`:

```bash
RUST_LOG=off cargo run -- "2 + 3 ="
```

## Container invocation

The container entry point uses the same argument contract:

```bash
docker run --rm cfg-parser:local "2 + 3 * 4 ="
docker compose run --rm cfgparser "2 + 3 * 4 ="
```

Image construction and Compose restrictions are documented in [Docker](docker.md).

## Current boundaries

CFG Parser currently:

- evaluates one expression per process;
- accepts command-line arguments or `CFGPARSER_INPUT`;
- falls back to a built-in expression when neither source is available;
- writes a human-readable result to standard output;
- reports failures with a non-zero process status.

It does not currently provide:

- standard-input expression handling;
- an interactive REPL;
- variables, assignment, or functions;
- arbitrary-precision arithmetic;
- machine-readable output;
- a stable reusable Rust library API.

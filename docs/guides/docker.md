# Docker

[← Documentation hub](../README.md) · [Getting started](getting-started.md) · [CLI usage](usage.md) · [Troubleshooting](troubleshooting.md)

CFG Parser provides a multi-stage Docker image for running the CLI without installing Rust on the host. The Compose configuration adds stricter runtime controls for local container execution.

## Quick start

Build from the repository root:

```bash
docker build -t cfg-parser:local .
```

Run an expression:

```bash
docker run --rm cfg-parser:local "2 + 3 * 4 ="
```

Expected output:

```text
Result: 14.000
```

The image entry point is the CFG Parser binary, so arguments after the image name are passed directly to the CLI.

## Image design

The [`Dockerfile`](../../Dockerfile) uses separate build and runtime stages:

```text
rust:1.85-slim-bookworm
        │
        ├─ copy Cargo.toml and Cargo.lock
        ├─ copy src/
        └─ cargo build --release --locked
        │
        ▼
debian:bookworm-slim
        │
        ├─ create unprivileged cfgparser user
        ├─ copy the release binary
        └─ run cfg-parser as cfgparser
```

The builder resolves dependencies from the committed lockfile and compiles with:

```bash
cargo build --release --locked
```

`--locked` makes the build fail rather than modify `Cargo.lock` when dependency metadata is inconsistent.

The runtime stage contains no Rust toolchain. It creates the non-login `cfgparser` user with UID `10001`, copies the executable to `/usr/local/bin/cfg-parser`, and switches to that user before execution.

The image entry point is:

```text
/usr/local/bin/cfg-parser
```

## Image metadata

The runtime image includes these OCI labels:

| Label | Value |
| --- | --- |
| `org.opencontainers.image.title` | `CFG Parser` |
| `org.opencontainers.image.description` | `Context-free grammar arithmetic parser and evaluator` |
| `org.opencontainers.image.source` | `https://github.com/simonesiega/cfg-parser` |
| `org.opencontainers.image.licenses` | `MIT` |

Inspect labels and image configuration with:

```bash
docker image inspect cfg-parser:local
```

## Run expressions

Pass one complete, quoted expression after the image name:

```bash
docker run --rm cfg-parser:local "(1 + 2)(3 + 4) ="
docker run --rm cfg-parser:local "2 ^ 8 ="
docker run --rm cfg-parser:local "27 $ 3 ="
```

`--rm` removes the stopped container, not the image.

To use the environment input source instead of arguments:

```bash
docker run --rm -e CFGPARSER_INPUT='27 $ 3 =' cfg-parser:local
```

Input selection and shell quoting are documented in [CLI usage](usage.md).

## Docker Compose

The repository's [`docker-compose.yml`](../../docker-compose.yml) defines the `cfgparser` service and names its image `cfg-parser:local`.

Build it:

```bash
docker compose build
```

Run an expression:

```bash
docker compose run --rm cfgparser "2 + 3 * 4 ="
```

Compose inherits the binary entry point, so expression arguments are forwarded directly to CFG Parser.

## Compose restrictions

The service applies three runtime controls:

| Setting | Effect |
| --- | --- |
| `network_mode: none` | Removes external network access. |
| `read_only: true` | Mounts the container root filesystem read-only. |
| `no-new-privileges:true` | Prevents processes from gaining additional privileges. |

CFG Parser needs no network or filesystem writes during normal evaluation. These controls complement the image's unprivileged runtime user.

A plain `docker run` uses the image's non-root user but does not inherit Compose-only controls. Apply the equivalent profile explicitly when needed:

```bash
docker run --rm \
  --network none \
  --read-only \
  --security-opt no-new-privileges:true \
  cfg-parser:local "2 + 3 ="
```

PowerShell and Command Prompt can use the same options on one line.

## Rebuild

Rebuild after changing Rust source, dependency metadata, or the Dockerfile:

```bash
docker build -t cfg-parser:local .
```

With Compose:

```bash
docker compose build
```

Docker reuses cached layers where possible. Bypass the cache only when necessary:

```bash
docker build --no-cache -t cfg-parser:local .
docker compose build --no-cache
```

## Inspect the runtime

List the image:

```bash
docker image ls cfg-parser:local
```

Confirm the configured user:

```bash
docker image inspect \
  --format '{{.Config.User}}' \
  cfg-parser:local
```

Expected value:

```text
cfgparser
```

Confirm the entry point:

```bash
docker image inspect \
  --format '{{json .Config.Entrypoint}}' \
  cfg-parser:local
```

Expected value:

```text
["/usr/local/bin/cfg-parser"]
```

## Docker-specific troubleshooting

| Symptom | Fix |
| --- | --- |
| Docker cannot connect to the daemon. | Start Docker Desktop or the system Docker service, then run `docker info`. |
| The image runs old code. | Rebuild it; use `--no-cache` only if ordinary rebuilding does not refresh it. |
| The locked build rejects dependency metadata. | Update and commit `Cargo.toml` and `Cargo.lock` together outside the image build. |
| A write fails under Compose. | Use plain `docker run` without `--read-only` only for a workflow that genuinely needs writes. |
| Network access fails under Compose. | This is expected under `network_mode: none`; normal evaluation needs no network. |

For expanded diagnosis, see [Troubleshooting](troubleshooting.md).

## Container boundaries

The container setup intentionally has a narrow scope:

- the final image excludes Rust and Cargo;
- the binary runs as a dedicated non-root user;
- no ports are exposed;
- no volume is required for expression evaluation;
- Compose disables networking and filesystem writes;
- Compose prevents privilege escalation.

The Dockerfile defines image contents, the entry point, and the runtime user. Compose defines the additional network, filesystem, and privilege restrictions.

FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime

LABEL org.opencontainers.image.title="CFG Parser" \
      org.opencontainers.image.description="Context-free grammar arithmetic parser and evaluator" \
      org.opencontainers.image.source="https://github.com/simonesiega/cfg-parser" \
      org.opencontainers.image.licenses="MIT"

RUN useradd --uid 10001 --user-group --no-create-home --shell /usr/sbin/nologin cfgparser

COPY --from=builder /app/target/release/cfg-parser /usr/local/bin/cfg-parser

USER cfgparser

ENTRYPOINT ["/usr/local/bin/cfg-parser"]

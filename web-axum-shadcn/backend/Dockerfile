# syntax=docker/dockerfile:1

# ---- Build -------------------------------------------------------------
FROM rust:1.95-slim-trixie AS build
WORKDIR /app

# Build dependencies first with placeholder sources, so this layer stays
# cached until Cargo.toml / Cargo.lock change.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo 'fn main() {}' > src/main.rs \
    && touch src/lib.rs \
    && cargo build --release --locked \
    && rm -rf src

COPY src ./src
# Make sure cargo rebuilds our crate instead of reusing the placeholder.
RUN touch src/main.rs src/lib.rs && cargo build --release --locked

# ---- Runtime -----------------------------------------------------------
FROM debian:trixie-slim
RUN useradd --system --uid 10001 --no-create-home app
COPY --from=build /app/target/release/backend /usr/local/bin/backend
USER app

ENV HOST=0.0.0.0 \
    PORT=8080 \
    RUST_LOG=info
EXPOSE 8080
CMD ["backend"]

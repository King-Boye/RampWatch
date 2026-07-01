FROM mcr.microsoft.com/devcontainers/base:bookworm

# Install curl and build deps
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl ca-certificates build-essential git pkg-config \
    && rm -rf /var/lib/apt/lists/*

# Install Rust via rustup
ENV CARGO_HOME=/usr/local/cargo \
    RUSTUP_HOME=/usr/local/rustup
ENV PATH="${CARGO_HOME}/bin:${PATH}"

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable

# Add WASM targets required for Soroban
RUN rustup target add wasm32-unknown-unknown wasm32-wasi

# Install Node.js (LTS)
# Use NodeSource via the devcontainer feature when possible; keep this Dockerfile minimal.

CMD ["sleep", "infinity"]


# Tariro AGI Dockerfile
# Multi-stage build for optimized production image

# Stage 1: Build
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create dummy source for dependency caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs && echo "mod lib;" > src/lib.rs

# Build dependencies first (layer caching)
RUN cargo build --release && rm -rf src

# Copy actual source
COPY src ./src
COPY benches ./benches

# Build release binary
RUN touch src/main.rs && cargo build --release --features full

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    tini \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user
RUN groupadd -r tariro && useradd -r -g tariro tariro

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/release/tariro /app/

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Set ownership
RUN chown -R tariro:tariro /app

# Switch to non-root user
USER tariro

# Use tini as init system
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

# Default command
CMD ["--help"]

# Labels
LABEL org.opencontainers.image.title="Tariro AGI"
LABEL org.opencontainers.image.description="Universal Meta-Compiler System implementing POCO-REAF"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Tariro"
LABEL org.opencontainers.image.version="1.0.12"

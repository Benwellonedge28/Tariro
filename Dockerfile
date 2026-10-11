# Tariro AGI Dockerfile
# Multi-stage build for optimized production image
# Supports multi-arch: amd64, arm64, arm/v7

# ============================================
# Stage 1: Build
# ============================================
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    pkg-config \
    libssl-dev \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /build

# Set environment variables for optimized build
ENV CARGO_PROFILE_RELEASE_LTO=fat \
    CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1 \
    CARGO_PROFILE_RELEASE_OPT_LEVEL=3 \
    RUST_BACKTRACE=1

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy source for dependency caching
RUN mkdir -p src benches && \
    echo "fn main() {}" > src/main.rs && \
    echo "mod lib;" > src/lib.rs

# Build dependencies first (important for layer caching)
RUN cargo build --release && rm -rf src benches

# Copy actual source
COPY src ./src
COPY benches ./benches
COPY rustfmt.toml clippy.toml ./
COPY Tariro_ide_idpte ./Tariro_ide_idpte

# Touch source files to invalidate cache
RUN find src -name "*.rs" -exec touch {} \;

# Build release binary with all features
RUN cargo build --release --features full --target x86_64-unknown-linux-gnu

# ============================================
# Stage 2: Runtime (Production)
# ============================================
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    tini \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Create non-root user and group
RUN groupadd --system --gid 1000 tariro && \
    useradd --system --uid 1000 --gid tariro tariro

# Create app directory
WORKDIR /app

# Copy binary from builder
COPY --from=builder /build/target/x86_64-unknown-linux-gnu/release/tariro /app/

# Copy entrypoint script
COPY docker-entrypoint.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

# Create directories for runtime data
RUN mkdir -p /app/data /app/logs /app/config && \
    chown -R tariro:tariro /app

# Set ownership
RUN chown -R tariro:tariro /app

# Use tini as init system and entrypoint
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]

# Default command
CMD ["--help"]

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD /app/tariro health || exit 1

# Environment variables
ENV TARIRO_CONFIG_PATH=/app/config \
    RUST_LOG=info \
    RUST_BACKTRACE=1

# Labels
LABEL org.opencontainers.image.title="Tariro AGI"
LABEL org.opencontainers.image.description="Universal Meta-Compiler System implementing POCO-REAF"
LABEL org.opencontainers.image.source="https://github.com/Benwellonedge28/Tariro"
LABEL org.opencontainers.image.version="1.0.15"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"
LABEL org.opencontainers.image.maintainer="Tariro AGI Contributors <contributors@tariro.dev>"
LABEL org.opencontainers.image.vendor="Tariro AGI"

# Expose default ports
EXPOSE 8080 8081

# Switch to non-root user
USER tariro

# ============================================
# Multi-arch Build Notes
# ============================================
# To build for multiple architectures:
# 1. Enable Docker BuildKit: export DOCKER_BUILDKIT=1
# 2. Set up QEMU: docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
# 3. Build with: docker buildx build --platform linux/amd64,linux/arm64,linux/arm/v7 -t tariro:latest .
#
# Or use GitHub Actions with the cd.yml workflow for automatic multi-arch builds.

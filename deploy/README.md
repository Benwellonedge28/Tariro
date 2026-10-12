# Tariro AGI Deployment Configurations

This directory contains deployment configurations for running Tariro AGI everywhere from tiny embedded devices to multi-planetary distributed systems.

## Directory Structure

```
deploy/
├── edge/                    # Edge computing deployments
│   ├── k8s-edge-deployment.yaml  # Kubernetes edge deployment
│   └── README.md
├── embedded/                 # Embedded and IoT configurations
│   ├── embedded-config.toml  # MCU/IoT gateway configuration
│   └── README.md
└── README.md                # This file
```

## Deployment Targets

### 1. Standard Servers (Cloud)
- **Platforms**: Linux (x86_64, ARM64), macOS, Windows
- **Features**: Full feature set
- **Deployment**: CD workflow auto-deploys to cloud

### 2. Edge Computing
- **Use Case**: CDN edge nodes, IoT gateways, satellite systems
- **Configuration**: See `edge/k8s-edge-deployment.yaml`
- **Features**: Optimized for limited resources, offline capability

### 3. Embedded Systems
- **Use Case**: Microcontrollers, IoT devices
- **Platforms**: ARM Cortex-M, RISC-V
- **Configuration**: See `embedded/embedded-config.toml`
- **Features**: Minimal footprint, no-std support

### 4. WebAssembly
- **Use Case**: Browser, serverless functions, edge workers
- **Platforms**: wasm32-unknown-unknown, wasm32-wasi
- **Build**: `cargo build --target wasm32-unknown-unknown`

### 5. Static Binaries (musl)
- **Use Case**: Containers, unikernels, isolated environments
- **Platforms**: Linux x86_64-musl, ARM64-musl, ARMv7-musl
- **Build**: `cargo build --target x86_64-unknown-linux-musl`

## Docker Images

| Image | Size | Use Case |
|-------|------|----------|
| `tariro:latest` | ~50MB | Standard containers |
| `tariro:alpine` | ~15MB | Alpine/musl containers |
| `tariro:distroless` | ~30MB | Minimal attack surface |
| `tariro:scratch` | ~10-50MB | Maximum minimal |

## Building

### Standard Build
```bash
cargo build --release --features full
```

### Static Build
```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
```

### Docker
```bash
# Standard
docker build -t tariro:latest .

# Alpine (smaller)
docker build -f Dockerfile.alpine -t tariro:alpine .

# Distroless (minimal)
docker build -f Dockerfile.distroless -t tariro:distroless .

# Scratch (binary only)
docker build -f Dockerfile.scratch -t tariro:scratch .
```

## Kubernetes Deployment

### Edge Deployment
```bash
kubectl apply -f deploy/edge/k8s-edge-deployment.yaml
```

### Configuration
Edit `deploy/edge/k8s-edge-deployment.yaml` to customize:
- Replica count
- Resource limits
- Regional settings
- Storage configuration

## Cross-Compilation Targets

| Target | Architecture | Use Case |
|--------|---------------|----------|
| `x86_64-unknown-linux-gnu` | x86_64 | Standard Linux |
| `aarch64-unknown-linux-gnu` | ARM64 | Server ARM |
| `armv7-unknown-linux-gnueabihf` | ARMv7 | ARM servers |
| `x86_64-unknown-linux-musl` | x86_64 | Static linking |
| `aarch64-unknown-linux-musl` | ARM64 | Static ARM |
| `thumbv7em-none-eabihf` | Cortex-M4 | Embedded |
| `riscv32i-unknown-none-elf` | RISC-V 32 | Embedded |
| `wasm32-unknown-unknown` | WebAssembly | Browser |
| `wasm32-wasi` | WASI | Serverless |

## Feature Flags

| Feature | Description |
|---------|-------------|
| `std` | Standard library (default) |
| `async-runtime` | Tokio async runtime |
| `logging` | Tracing/logging support |
| `metrics` | Prometheus metrics |
| `profiling` | Performance profiling |
| `telemetry` | OpenTelemetry tracing |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Log level |
| `RUST_BACKTRACE` | `1` | Backtrace enable |
| `TARIRO_CONFIG_PATH` | - | Config file path |
| `NODE_ID` | auto | Distributed node ID |
| `REGION` | `unknown` | Planetary region |

## Multi-Planetary Configuration

For cross-planetary deployments, set:
```bash
export REGION="earth"    # earth, mars, moon, europa
export NODE_ID="unique-id"
```

## Security

- All containers run as non-root user
- Distroless and Scratch images have minimal attack surface
- SBOM and vulnerability scanning in CI/CD
- TLS support for all network communications

## License

MIT OR Apache-2.0

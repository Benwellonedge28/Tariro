# Platform support

This document records which targets Tariro actually builds for, which ones it
does not, and why. It is the reference for the CI workflows in
`.github/workflows/`, so that a green build means the same thing as the
documentation claims.

## Supported targets

These targets are built by CI on every push and are expected to keep working.

| Target | Kind | Built by | Notes |
|---|---|---|---|
| `x86_64-unknown-linux-gnu` | Host, glibc | `ci.yml`, `cd.yml` | Primary release target. |
| `aarch64-unknown-linux-gnu` | Cross, glibc | `ci.yml` (`cross-build`) | Cross-compiled with the system toolchain. |
| `armv7-unknown-linux-gnueabihf` | Cross, glibc | `ci.yml` (`cross-build`) | Cross-compiled with the system toolchain. |
| `x86_64-unknown-linux-musl` | Cross, static | `embedded.yml` (`static-binaries`) | Fully static, no glibc dependency. |
| `aarch64-unknown-linux-musl` | Cross, static | `embedded.yml` (`static-binaries`) | Fully static. |
| `armv7-unknown-linux-musleabihf` | Cross, static | `embedded.yml` (`static-binaries`) | Fully static. |
| `i686-unknown-linux-musl` | Cross, static | `embedded.yml` (`static-binaries`) | Fully static, 32-bit x86. |
| `wasm32-wasip1` | WebAssembly, WASI | `embedded.yml` (`wasm-wasi`) | Standard library is available on this target. |

Container images are built for Alpine (musl), Distroless and a scratch image in
`embedded.yml`; the release pipeline publishes them from `cd.yml`.

## Not yet supported

### Bare-metal targets (Cortex-M, RISC-V)

Targets such as `thumbv7em-none-eabihf`, `thumbv6m-none-eabi` and
`riscv32imc-unknown-none-elf` are **not** supported yet, and the crate cannot
be built for them today.

The reason is the dependency set. Tariro is a `std` application and pulls in:

* `tokio` with the `full` feature (networking, filesystem, process support),
* `serde` with `std`,
* `rayon` and `num_cpus` for parallelism,
* the MongoDB driver for the storage backend.

None of these exist for bare-metal targets. Supporting them requires a
`no_std` port: the crate root would need `#![cfg_attr(not(feature = "std"), no_std)]`,
the `std`-only dependencies would have to become optional, and the modules
that use them would need to be feature-gated out. The AST in
`src/tumc_compiler/ast.rs` and the frontend CST in
`src/tumc_compiler/frontend.rs` are the natural starting points, because they
only need `alloc`.

The `bare-metal-readiness` job in `embedded.yml` keeps the bare-metal targets
installed and reports the gap instead of pretending to produce a firmware
binary that cannot exist.

### FreeBSD

`x86_64-unknown-freebsd` needs a FreeBSD cross-linker, which the Linux CI
runners do not provide. It is not part of the build matrix until a cross
toolchain is wired up.

## Minimum supported Rust version

The MSRV is declared in `Cargo.toml` as `rust-version` and is currently
**1.88**. The floor comes from the dependency set, not from the crate's own
code: `mongodb` 3.9 and `time` 0.3 both require Rust 1.88.

The `msrv` job in `ci.yml` and the `msrv-nightly` job in `nightly.yml` build
with exactly that toolchain, and both read the version from the manifest so
the two cannot drift apart.

## Feature flags

| Feature | Default | Purpose |
|---|---|---|
| `std` | yes | Standard library support. |
| `alloc` | – | Enables `alloc`-only builds. |
| `async-runtime` | yes | Enables the tokio multi-threaded runtime. |
| `logging` | yes | `tracing` and `tracing-subscriber`. |
| `database` | yes | MongoDB storage backend and the SDK entity types. |
| `metrics` | no | Prometheus and `metrics` exporters. |
| `profiling` | no | `tracing-timing` spans. |
| `telemetry` | no | OpenTelemetry tracing with a Jaeger exporter. |
| `full` | no | Everything above, including `database`. |

`--no-default-features` is **not** a supported configuration: the modules are
not feature-gated, so dropping `std` or `async-runtime` breaks the build. The
`default` feature set is the supported baseline.

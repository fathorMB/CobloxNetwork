# Coblox

Coblox is a native-node network. The shared Rust core is consumed by a headless
node, an Android Kotlin shell through UniFFI, and a Tauri desktop shell.

## Build

The complete local build instructions, platform prerequisites, and diagnostics
are maintained in [the build-toolchain runbook](.lmbrain/knowledge/build-toolchain.md).

Quick Rust verification:

```powershell
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo deny check
```

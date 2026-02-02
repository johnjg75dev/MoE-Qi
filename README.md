# MoE-Qi

Rust libraries and bindings for the `moeqi` codec/core implementation, plus helper build scripts for native and WASM outputs.

## Structure

- `moeqi-core/` — core Rust library (`moeqi-core`)
- `moeqi/` — convenience wrapper crate (`moeqi`)
- `moeqi-ffi/` — FFI crate that builds a DLL (`moeqi-ffi`)
- `moeqi-wasm/` — WASM bindings (`moeqi-wasm`)
- `dist/` — build artifacts copied by the scripts (gitignored)
- `common.bat` — all-in-one script for build/test/doc/dll/wasm outputs

## Prerequisites

- Rust toolchain (stable)
- `wasm-pack` (only for the WASM build)

## Documentation

```powershell
cargo doc --workspace --no-deps
```

## Build

```powershell
# Core library
cargo build -p moeqi-core --release

# FFI DLL
.\common.bat dll

# WASM package
.\common.bat wasm
```

Artifacts are copied into `dist/`.

## Notes

- Build outputs, IDE files, and generated artifacts are excluded via `.gitignore`.

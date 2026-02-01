# MoE-EncDec-gpt / lib

Rust libraries and bindings for the `moeqi` codec/core implementation, plus helper build scripts for native and WASM outputs.

## Structure

- `ol/moeqi-core/` — core Rust library (`moeqi-core`)
- `ol/moeqi-ffi/` — FFI crate that builds a DLL (`moeqi-ffi`)
- `ol/moeqi-wasm/` — WASM bindings (`moeqi-wasm`)
- `ol/dist/` — build artifacts copied by the scripts (gitignored)
- `ol/compile-dll.bat` — Windows build script for the DLL
- `ol/compile-wasm.bat` — WASM build script (uses `wasm-pack`)
- `moeqi/` — workspace manifest + sample crate

## Prerequisites

- Rust toolchain (stable)
- `wasm-pack` (only for the WASM build)

## Build

From `C:\Users\John\Desktop\AI Gens\MoE-EncDec-gpt\lib\ol`:

```powershell
# Core library
cargo build --release --manifest-path moeqi-core\Cargo.toml

# FFI DLL
.\compile-dll.bat

# WASM package
.\compile-wasm.bat
```

Artifacts are copied into `ol/dist/`.

## Notes

- Build outputs, IDE files, and generated artifacts are excluded via `.gitignore`.

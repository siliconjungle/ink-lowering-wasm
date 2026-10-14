# Ink WebAssembly lowering

WebAssembly target configuration for checked typed Rust emission. Complete pure/stateful programs, constructor definitions and standalone representations reuse ink-lowering-rust and compile with Rust/LLVM to wasm32-unknown-unknown. Runtime owns host assembly and browser adapters. This package adds no optimisation or proof authority.

GPU bundles use Rust CPU fallback projects with scalar/packed host adapters. WGSL/WebGPU device execution and JavaScript execution remain primary supported paths. No C compiler or C lowering package is required.

Current execution coverage and GPU exclusions are documented in the [lowering parity contract](https://github.com/siliconjungle/ink-lang/blob/main/docs/lowering-parity.md).

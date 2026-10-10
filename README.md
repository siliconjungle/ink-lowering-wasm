# Ink WebAssembly lowering

WebAssembly target configuration and checked pure C lowering for Ink. `lower(&CheckedModule)` returns C and the required export symbols; `TARGET` selects the portable Zig/LLVM Wasm target. The shared `ink-runtime` invokes the toolchain and owns browser host ABIs, CPU fallback assembly and state host support.

Complete pure/stateful programs compile through literal C and shared Rust primitives, or through the Rust backend, using LLVM's wasm32 target. Their host adapter is runtime-owned. This package adds no optimisation or proof authority. The checked semantic core, literal emission, target toolchains and physical ABI remain trusted.

Current complete execution coverage, GPU exclusions and integration evidence are documented in the [lowering parity contract](https://github.com/siliconjungle/ink-lang/blob/main/docs/lowering-parity.md).
`lower_complete` describes the complete C primitive-ABI target. The runtime assembles and compiles shared primitives and supplies lifecycle/buffer hosts. Packed pure emission remains an explicit separate ABI.

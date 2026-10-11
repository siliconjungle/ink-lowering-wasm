# Ink WebAssembly lowering

WebAssembly target configuration and checked pure C lowering for Ink. `lower(&CheckedModule)` returns C and the required export symbols; `TARGET` selects the portable Zig/LLVM Wasm target. The shared `ink-runtime` invokes the toolchain and owns browser host ABIs, CPU fallback assembly and state host support.

Complete pure/stateful programs compile through literal C and shared Rust primitives, or through the Rust backend, using LLVM's wasm32 target. Their host adapter is runtime-owned. This package adds no optimisation or proof authority. The checked semantic core, literal emission, target toolchains and physical ABI remain trusted.

Current complete execution coverage, GPU exclusions and integration evidence are documented in the [lowering parity contract](https://github.com/siliconjungle/ink-lang/blob/main/docs/lowering-parity.md).
`lower_complete` describes the complete C primitive-ABI target. The runtime assembles and compiles shared primitives and supplies lifecycle/buffer hosts. Packed pure emission remains an explicit separate ABI.

`lower_complete_selected(&CheckedSelection)` delegates to the selected complete
C emitter, retaining the original checked checkpoint namespace. Runtime-owned
C/Rust Wasm host assembly uses that namespace; target configuration grants no
proof or checkpoint identity authority of its own.

All targets consume [ink-abi](https://github.com/siliconjungle/ink-abi) for shared
type/layout contracts. Early termination, owned-buffer transforms and helper
unfolding are checked database choices. The backend emits the resulting general
operations literally; it does not infer those transformations from source patterns.

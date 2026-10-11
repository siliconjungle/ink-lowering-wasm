# Ink WebAssembly lowering

WebAssembly target configuration and checked pure C lowering for Ink. `lower(&CheckedModule)` returns C and the required export symbols; `TARGET` selects the portable Zig/LLVM Wasm target. The shared `ink-runtime` invokes the toolchain and owns browser host ABIs, CPU fallback assembly and state host support.

Complete pure/stateful programs compile through literal C and shared Rust primitives, or through the Rust backend, using LLVM's wasm32 target. Their host adapter is runtime-owned. This package adds no optimisation or proof authority. The checked semantic core, literal emission, target toolchains and physical ABI remain trusted.

Current complete execution coverage, GPU exclusions and integration evidence are documented in the [lowering parity contract](https://github.com/siliconjungle/ink-lang/blob/main/docs/lowering-parity.md).
`lower_complete` describes the complete C primitive-ABI target. The runtime assembles and compiles shared primitives and supplies lifecycle/buffer hosts. Packed pure emission remains an explicit separate ABI.

`lower_complete_selected(&CheckedSelection)` delegates to the selected complete
C emitter, retaining the original checked checkpoint namespace. Runtime-owned
C/Rust Wasm host assembly uses that namespace; target configuration grants no
proof or checkpoint identity authority of its own.

Standalone proof-selected representations use the compile-time checked witness
and emit literal functions with native state retained between calls. Generated
applications carry no proof package, checker context, source AST or Ink runtime
dependency. See ink-lang/docs/database-representations.md and the shared
`representation_parity` conformance gate. Recursive GPU state uses the complete
host lane; device eligibility remains separate.


Generated entry points trust the caller's declared argument types, arity, shapes,
tags and canonical ranges. Bad data is unsafe and may crash, trap, corrupt state
or produce incorrect results; raw native pointers also require valid alignment,
length and lifetime. No malformed-input diagnostic or rollback is promised.
Representation conversion/JSON parsing and explicit portable checkpoint integrity
checks remain. Source/proof checking happens during compilation. Standalone
representation calls skip whole-state checkpoint validation; read-only observation
envelopes are erased when existing definitional computation establishes unchanged
logical state/version and no events.

# Ink WebAssembly lowering

WebAssembly target configuration and checked pure C lowering for Ink. `lower(&CheckedModule)` returns C and the required export symbols; `TARGET` selects the portable Zig/LLVM Wasm target. The shared `ink-runtime` invokes the toolchain and owns browser host ABIs, CPU fallback assembly and state host support.

Pure programs compile through C/LLVM. State programs compile through the Rust backend and LLVM's wasm32 target; their host adapter is runtime-owned. This package adds no optimisation or proof authority. The checked semantic core, literal emission, target toolchains and physical ABI remain trusted.

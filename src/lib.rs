//! WebAssembly target configuration for checked C lowering. Host ABI is runtime-owned.
use ink_core::{core::CheckedModule, LangResult};
pub struct Emission {
    pub c: String,
    pub exports: Vec<String>,
}
pub fn lower(module: &CheckedModule) -> LangResult<Emission> {
    let p = module.pure_program()?;
    let c = ink_lowering_c::native::emit(p)?;
    let exports = if ink_core::compute_values::needed(p) {
        vec![
            "ink_compute_call".into(),
            "ink_compute_reset".into(),
            "ink_alloc".into(),
        ]
    } else {
        p.functions
            .iter()
            .map(|f| format!("lang_fn_{}", f.name))
            .collect()
    };
    Ok(Emission { c, exports })
}
pub const TARGET: &str = "wasm32-freestanding";
pub const RUST_TARGET: &str = "wasm32-unknown-unknown";

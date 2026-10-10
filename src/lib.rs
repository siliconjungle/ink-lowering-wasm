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

/// Full value/state lowering. The runtime assembles and links the literal C
/// artifact with the shared primitive ABI for wasm32; no AST is embedded.
pub fn lower_complete(module: &CheckedModule) -> LangResult<ink_lowering_c::complete::Emission> {
    ink_lowering_c::complete::lower(module)
}
pub const VALUE_ABI: &str = "ink-values-v1";

/// Complete selected programs retain the original checked snapshot namespace.
pub fn lower_complete_selected(
    selected: &ink_core::optimisation::CheckedSelection,
) -> LangResult<ink_lowering_c::complete::Emission> {
    ink_lowering_c::complete::lower_selected(selected)
}

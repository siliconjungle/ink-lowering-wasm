//! WebAssembly target configuration for checked C lowering. Host ABI is runtime-owned.
use ink_core::{core::CheckedModule, LangResult};
pub struct Emission {
    pub c: String,
    pub exports: Vec<String>,
}
pub fn lower(module: &CheckedModule) -> LangResult<Emission> {
    lower_with_abi(module, ink_core::compute_values::needed(module.pure_program()?))
}
/// Explicit physical ABI configuration for bundles assembled by the runtime.
pub fn lower_with_abi(module: &CheckedModule, packed: bool) -> LangResult<Emission> {
    let p = module.pure_program()?;
    let c = if packed { ink_lowering_c::compute::emit_c(p)? } else { ink_lowering_c::native::emit(p)? };
    let exports = if packed {
        vec![
            "ink_compute_call".into(),
            "ink_compute_into".into(),
            "ink_compute_mark".into(),
            "ink_compute_rewind".into(),
            "ink_compute_scalar32".into(),
            "ink_compute_scalar64".into(),
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

/// Literal constructor definitions share C semantics and resource bounds.
pub fn lower_definition(checked: &ink_core::definition::CheckedDefinition) -> LangResult<ink_lowering_c::definition::Emission> {
    ink_lowering_c::definition::lower(checked)
}

/// Complete selected programs retain the original checked snapshot namespace.
pub fn lower_complete_selected(
    selected: &ink_core::optimisation::CheckedSelection,
) -> LangResult<ink_lowering_c::complete::Emission> {
    ink_lowering_c::complete::lower_selected(selected)
}

/// Standalone recursive representations execute in the literal C/native host lane.
pub fn lower_representation(checked: &ink_core::representation::CheckedRepresentation) -> LangResult<ink_lowering_c::representation_program::Emission> {
    ink_lowering_c::representation_program::lower(checked)
}

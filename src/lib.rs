//! WebAssembly target configuration for the ordinary typed Rust lowering.
use ink_core::{core::CheckedModule, LangResult};
pub struct Emission {
    pub rust: String,
}
pub const RUST_TARGET: &str = "wasm32-unknown-unknown";
pub const VALUE_ABI: &str = ink_abi::VALUE_ABI;
pub fn lower(module: &CheckedModule) -> LangResult<Emission> {
    Ok(Emission {
        rust: ink_lowering_rust::state_native::emit(module.program())?,
    })
}
pub fn lower_with_abi(module: &CheckedModule, packed: bool) -> LangResult<Emission> {
    Ok(Emission {
        rust: ink_lowering_rust::packed::lower(module, packed)?,
    })
}
pub fn lower_complete(module: &CheckedModule) -> LangResult<String> {
    ink_lowering_rust::state_native::emit(module.program())
}
pub fn lower_complete_selected(
    selected: &ink_core::optimisation::CheckedSelection,
) -> LangResult<String> {
    ink_lowering_rust::state_native::emit_selected(selected)
}
pub fn lower_definition(
    checked: &ink_core::definition::CheckedDefinition,
) -> LangResult<ink_lowering_rust::definition_native::Emission> {
    ink_lowering_rust::definition_native::lower(checked)
}
pub fn lower_representation(
    checked: &ink_core::representation::CheckedRepresentation,
) -> LangResult<String> {
    ink_lowering_rust::representation_program::lower(checked)
}

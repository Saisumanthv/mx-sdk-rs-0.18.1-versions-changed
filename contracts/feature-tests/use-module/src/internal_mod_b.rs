dharitri_wasm::imports!();

/// Contains all events that can be emitted by the contract.
#[dharitri_wasm::module]
pub trait InternalModuleB {
    #[view]
    fn call_mod_b(&self) {}
}

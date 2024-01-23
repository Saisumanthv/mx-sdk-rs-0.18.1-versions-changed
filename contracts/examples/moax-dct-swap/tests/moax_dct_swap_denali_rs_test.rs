use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/moax-dct-swap.wasm",
        Box::new(|context| Box::new(moax_dct_swap::contract_obj(context))),
    );
    contract_map
}

#[test]
fn unwrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/unwrap_moax.scen.json", &contract_map());
}

#[test]
fn wrap_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/wrap_moax.scen.json", &contract_map());
}

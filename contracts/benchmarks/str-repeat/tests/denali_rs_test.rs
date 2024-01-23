use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/str-repeat.wasm",
        Box::new(|context| Box::new(str_repeat::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_str_repeat_denali_rs() {
    dharitri_wasm_debug::denali_rs("denali/str_repeat.scen.json", &contract_map());
}

use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/payable-features.wasm",
        Box::new(|context| Box::new(payable_features::contract_obj(context))),
    );
    contract_map
}

#[test]
fn payable_any_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_1.scen.json", &contract_map());
}

#[test]
fn payable_any_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_2.scen.json", &contract_map());
}

#[test]
fn payable_any_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_3.scen.json", &contract_map());
}

#[test]
fn payable_any_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_any_4.scen.json", &contract_map());
}

#[test]
fn payable_moax_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_1.scen.json", &contract_map());
}

#[test]
fn payable_moax_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_2.scen.json", &contract_map());
}

#[test]
fn payable_moax_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_3.scen.json", &contract_map());
}

#[test]
fn payable_moax_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_moax_4.scen.json", &contract_map());
}

#[test]
fn payable_token_1_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_1.scen.json", &contract_map());
}

#[test]
fn payable_token_2_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_2.scen.json", &contract_map());
}

#[test]
fn payable_token_3_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_3.scen.json", &contract_map());
}

#[test]
fn payable_token_4_rs() {
    dharitri_wasm_debug::denali_rs("denali/payable_token_4.scen.json", &contract_map());
}

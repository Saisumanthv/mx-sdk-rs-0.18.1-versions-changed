use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../../output/multisig.wasm",
        Box::new(|context| Box::new(multisig::contract_obj(context))),
    );
    contract_map
}

#[test]
fn test_change_board_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeBoard.scen.json", &contract_map());
}

#[test]
fn test_change_quorum_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum.scen.json", &contract_map());
}

#[test]
fn test_change_quorum_too_big_rs() {
    dharitri_wasm_debug::denali_rs("denali/changeQuorum_tooBig.scen.json", &contract_map());
}

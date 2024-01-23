use dharitri_wasm::*;
use dharitri_wasm_debug::*;

#[allow(dead_code)]
fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/erc1155-marketplace.wasm",
        Box::new(|context| Box::new(erc1155_marketplace::contract_obj(context))),
    );
    contract_map.register_contract(
        "file:../../erc1155/output/erc1155.wasm",
        Box::new(|context| Box::new(erc1155::contract_obj(context))),
    );

    contract_map
}

/* is_smart_contract not yet implemented, uncomment later

#[test]
fn auction_single_token_moax_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/auction_single_token_moax.scen.json", &contract_map());
}

#[test]
fn auction_batch_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/auction_batch.scen.json", &contract_map());
}

#[test]
fn bid_first_moax_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_first_moax.scen.json", &contract_map());
}

#[test]
fn bid_second_moax_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_second_moax.scen.json", &contract_map());
}

#[test]
fn bid_third_moax_test_rs() {
    dharitri_wasm_debug::denali_rs("denali/bid_third_moax.scen.json", &contract_map());
}

#[test]
fn end_auction_test_rs() {
    dharitri_wasm_debug::denali_rs("/home/dharitri/dharitri-wasm-rs/contracts/examples/erc1155-marketplace/denali/end_auction.scen.json", &contract_map());
}

*/

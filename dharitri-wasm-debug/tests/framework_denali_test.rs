use dharitri_wasm_debug::*;

// These tests don't really test any contract, but the testing framework itslef.

fn contract_map() -> ContractMap<TxContext> {
    ContractMap::new()
}

/// Checks that externalSteps work fine.
#[test]
fn external_steps_rs() {
    dharitri_wasm_debug::denali_rs(
        "tests/denali/external_steps/external_steps.scen.json",
        &contract_map(),
    );
}

#[test]
fn transfer_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/transfer.scen.json", &contract_map());
}

#[test]
fn validator_reward_rs() {
    dharitri_wasm_debug::denali_rs("tests/denali/validatorReward.scen.json", &contract_map());
}

use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();

    contract_map.register_contract(
        "file:../forwarder/output/forwarder.wasm",
        Box::new(|context| Box::new(forwarder::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../forwarder-raw/output/forwarder-raw.wasm",
        Box::new(|context| Box::new(forwarder_raw::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../proxy-test-first/output/proxy-test-first.wasm",
        Box::new(|context| Box::new(proxy_test_first::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../proxy-test-second/output/proxy-test-second.wasm",
        Box::new(|context| Box::new(proxy_test_second::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../vault/output/vault.wasm",
        Box::new(|context| Box::new(vault::contract_obj(context))),
    );

    contract_map
}
#[test]
fn forw_raw_async_accept_moax_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forw_raw_async_accept_moax.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn forw_raw_async_accept_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forw_raw_async_accept_dct.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn forw_raw_async_echo_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_async_echo.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_direct_moax.scen.json", &contract_map());
}

#[test]
fn forw_raw_direct_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/forw_raw_direct_dct.scen.json", &contract_map());
}

// #[test]
// fn forw_raw_sync_echo_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forw_raw_sync_moax_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/forw_raw_sync_moax.scen.json", &contract_map());
// }

#[test]
fn forwarder_call_async_accept_moax_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/forwarder_call_async_accept_moax.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn forwarder_call_async_accept_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_async_accept_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_accept_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_async_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_async_retrieve_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_async_retrieve_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_async_retrieve_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_async_retrieve_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_then_read_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_then_read_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_accept_then_read_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_accept_then_read_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_retrieve_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_retrieve_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_sync_retrieve_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_sync_retrieve_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_moax_twice_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_moax_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_dct_twice_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_dct_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_nft_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_nft.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_return_values_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_return_values.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_call_transf_exec_accept_sft_twice_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_call_transf_exec_accept_sft_twice.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_change_owner_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_contract_change_owner.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_deploy_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_contract_deploy.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_contract_upgrade_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_contract_upgrade.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_nft_create_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/forwarder_nft_create.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_nft_transfer_async_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_nft_transfer_async.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_nft_transfer_exec_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_nft_transfer_exec.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_send_twice_moax_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_send_twice_moax.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_send_twice_dct_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_send_twice_dct.scen.json",
// 		&contract_map(),
// 	);
// }

// #[test]
// fn forwarder_sync_echo_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/forwarder_sync_echo.scen.json", &contract_map());
// }

// #[test]
// fn forwarder_sync_echo_range_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/forwarder_sync_echo_range.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn proxy_test_message_othershard_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_otherShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_othershard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_otherShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_sameshard_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_sameShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_message_sameshard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_message_sameShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_othershard_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_otherShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_othershard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_otherShard_callback.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_sameshard_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_sameShard.scen.json",
        &contract_map(),
    );
}

#[test]
fn proxy_test_payment_sameshard_callback_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/proxy_test_payment_sameShard_callback.scen.json",
        &contract_map(),
    );
}

// #[test]
// fn recursive_caller_moax_1_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_moax_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_moax_2_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_moax_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_moax_x_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_moax_x.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dct_1_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_dct_1.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dct_2_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_dct_2.scen.json", &contract_map());
// }

// #[test]
// fn recursive_caller_dct_x_rs() {
// 	dharitri_wasm_debug::denali_rs("denali/recursive_caller_dct_x.scen.json", &contract_map());
// }

#[test]
fn send_moax_rs() {
    dharitri_wasm_debug::denali_rs("denali/send_moax.scen.json", &contract_map());
}

#[test]
fn send_dct_rs() {
    dharitri_wasm_debug::denali_rs("denali/send_dct.scen.json", &contract_map());
}

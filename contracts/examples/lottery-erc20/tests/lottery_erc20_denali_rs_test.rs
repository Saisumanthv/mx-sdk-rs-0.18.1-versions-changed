use dharitri_wasm::*;
use dharitri_wasm_debug::*;

fn contract_map() -> ContractMap<TxContext> {
    let mut contract_map = ContractMap::new();
    contract_map.register_contract(
        "file:../output/lottery-erc20.wasm",
        Box::new(|context| Box::new(lottery_erc20::contract_obj(context))),
    );

    contract_map.register_contract(
        "file:../../erc20/output/erc20.wasm",
        Box::new(|context| Box::new(erc20::contract_obj(context))),
    );

    contract_map
}

#[test]
fn buy_all_tickets_different_accounts_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-all-tickets-different-accounts.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_more_tickets_than_allowed_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-more-tickets-than-allowed.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_rs() {
    dharitri_wasm_debug::denali_rs("denali/buy-ticket.scen.json", &contract_map());
}

#[test]
fn buy_ticket_after_deadline_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-after-deadline.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_after_determined_winner_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-after-determined-winner.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_after_sold_out_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-after-sold-out.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_all_options_rs() {
    dharitri_wasm_debug::denali_rs("denali/buy-ticket-all-options.scen.json", &contract_map());
}

#[test]
fn buy_ticket_another_account_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-another-account.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_not_on_whitelist_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-not-on-whitelist.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_same_account_rs() {
    dharitri_wasm_debug::denali_rs("denali/buy-ticket-same-account.scen.json", &contract_map());
}

#[test]
fn buy_ticket_second_lottery_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/buy-ticket-second-lottery.scen.json",
        &contract_map(),
    );
}

#[test]
fn buy_ticket_wrong_fee_rs() {
    dharitri_wasm_debug::denali_rs("denali/buy-ticket-wrong-fee.scen.json", &contract_map());
}

#[test]
fn determine_winner_different_ticket_holders_winner_acc1_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/determine-winner-different-ticket-holders-winner-acc1.scen.json",
        &contract_map(),
    );
}

#[test]
fn determine_winner_early_rs() {
    dharitri_wasm_debug::denali_rs("denali/determine-winner-early.scen.json", &contract_map());
}

#[test]
fn determine_winner_same_ticket_holder_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/determine-winner-same-ticket-holder.scen.json",
        &contract_map(),
    );
}

// TODO: uncomment after rust-denali supports chaining async calls
// #[test]
// fn determine_winner_split_prize_pool_rs() {
// 	dharitri_wasm_debug::denali_rs(
// 		"denali/determine-winner-split-prize-pool.scen.json",
// 		&contract_map(),
// 	);
// }

#[test]
fn lottery_init_rs() {
    dharitri_wasm_debug::denali_rs("denali/lottery-init.scen.json", &contract_map());
}

#[test]
fn start_after_announced_winner_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-after-announced-winner.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_all_options_bigger_whitelist_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-all-options-bigger-whitelist.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_alternative_function_name_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-alternative-function-name.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_fixed_deadline_rs() {
    dharitri_wasm_debug::denali_rs("denali/start-fixed-deadline.scen.json", &contract_map());
}

#[test]
fn start_limited_tickets_rs() {
    dharitri_wasm_debug::denali_rs("denali/start-limited-tickets.scen.json", &contract_map());
}

#[test]
fn start_limited_tickets_and_fixed_deadline_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-limited-tickets-and-fixed-deadline.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_deadline_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-limited-tickets-and-fixed-deadline-invalid-deadline.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_limited_tickets_and_fixed_deadline_invalid_ticket_price_arg_rs() {
    dharitri_wasm_debug::denali_rs(
        "denali/start-limited-tickets-and-fixed-deadline-invalid-ticket-price-arg.scen.json",
        &contract_map(),
    );
}

#[test]
fn start_second_lottery_rs() {
    dharitri_wasm_debug::denali_rs("denali/start-second-lottery.scen.json", &contract_map());
}

#[test]
fn start_with_all_options_rs() {
    dharitri_wasm_debug::denali_rs("denali/start-with-all-options.scen.json", &contract_map());
}

#[test]
fn start_with_no_options_rs() {
    dharitri_wasm_debug::denali_rs("denali/start-with-no-options.scen.json", &contract_map());
}

use dharitri_wasm::types::Address;
use num_bigint::BigUint;

use crate::*;

const DCT_TRANSFER_FUNC: &[u8] = b"DCTTransfer";
const SET_USERNAME_FUNC: &[u8] = b"SetUserName";

pub fn try_execute_builtin_function(
    tx_input: &TxInput,
    state: &mut BlockchainMock,
) -> Option<TxResult> {
    match tx_input.func_name.as_slice() {
        DCT_TRANSFER_FUNC => Some(execute_dct_transfer(tx_input, state)),
        SET_USERNAME_FUNC => Some(execute_set_username(tx_input, state)),
        _ => None,
    }
}

fn execute_dct_transfer(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
    let from = tx_input.from.clone();
    let to = tx_input.to.clone();
    let dct_token_identifier = tx_input.dct_token_identifier.clone();
    let dct_value = tx_input.dct_value.clone();

    state.substract_dct_balance(&from, &dct_token_identifier, &dct_value);
    state.increase_dct_balance(&to, &dct_token_identifier, &dct_value);
    TxResult {
        result_status: 0,
        result_message: Vec::new(),
        result_values: Vec::new(),
        result_logs: vec![dct_transfer_event_log(
            from.clone(),
            to.clone(),
            dct_token_identifier,
            &dct_value,
        )],
    }
}

pub fn dct_transfer_event_log(
    from: Address,
    to: Address,
    dct_token_identifier: Vec<u8>,
    dct_value: &BigUint,
) -> TxLog {
    TxLog {
        address: from,
        endpoint: b"DCTTransfer".to_vec(),
        topics: vec![dct_token_identifier, dct_value.to_bytes_be(), to.to_vec()],
        data: vec![],
    }
}

fn execute_set_username(tx_input: &TxInput, state: &mut BlockchainMock) -> TxResult {
    assert_eq!(tx_input.args.len(), 1, "SetUserName expects 1 argument");
    if state.try_set_username(&tx_input.to, tx_input.args[0].as_slice()) {
        TxResult::empty()
    } else {
        TxResult {
            result_status: 10,
            result_message: b"username already set".to_vec(),
            result_values: Vec::new(),
            result_logs: Vec::new(),
        }
    }
}

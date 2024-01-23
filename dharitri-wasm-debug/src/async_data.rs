use crate::tx_context::*;
use dharitri_wasm::dharitri_codec::*;
use dharitri_wasm::hex_call_data::HexCallDataDeserializer;
use dharitri_wasm::types::{Address, H256};

use num_bigint::BigUint;

use alloc::vec::Vec;

const DCT_TRANSFER_STRING: &[u8] = b"DCTTransfer";

#[derive(Debug)]
pub struct AsyncCallTxData {
    pub to: Address,
    pub call_data: Vec<u8>,
    pub call_value: BigUint,
    pub tx_hash: H256,
}

pub fn async_call_tx_input(async_data: &AsyncCallTxData, contract_addr: &Address) -> TxInput {
    let mut de = HexCallDataDeserializer::new(async_data.call_data.as_slice());
    let func_name = de.get_func_name().to_vec();
    let mut args: Vec<Vec<u8>> = Vec::new();
    let mut dct_token_identifier = Vec::<u8>::new();
    let mut dct_value = 0u32.into();

    if func_name == DCT_TRANSFER_STRING {
        dct_token_identifier = de.next_argument().unwrap().unwrap();
        dct_value = BigUint::from_bytes_be(&de.next_argument().unwrap().unwrap());
    }

    while let Some(deserialized_arg) = de.next_argument().unwrap() {
        args.push(deserialized_arg);
    }
    TxInput {
        from: contract_addr.clone(),
        to: async_data.to.clone(),
        call_value: async_data.call_value.clone(),
        dct_value,
        dct_token_identifier,
        func_name,
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn async_callback_tx_input(
    async_data: &AsyncCallTxData,
    contract_addr: &Address,
    async_result: &TxResult,
) -> TxInput {
    let mut args: Vec<Vec<u8>> = Vec::new();
    let serialized_bytes = top_encode_to_vec(&async_result.result_status).unwrap();
    args.push(serialized_bytes);
    if async_result.result_status == 0 {
        args.extend_from_slice(async_result.result_values.as_slice());
    } else {
        args.push(async_result.result_message.clone());
    }
    TxInput {
        from: async_data.to.clone(),
        to: contract_addr.clone(),
        call_value: 0u32.into(),
        dct_value: 0u32.into(),
        dct_token_identifier: Vec::new(),
        func_name: b"callBack".to_vec(),
        args,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
    }
}

pub fn merge_results(mut original: TxResult, mut new: TxResult) -> TxResult {
    if original.result_status == 0 {
        original.result_values.append(&mut new.result_values);
        original.result_logs.append(&mut new.result_logs);
        original.result_message = new.result_message;
        original
    } else {
        new
    }
}

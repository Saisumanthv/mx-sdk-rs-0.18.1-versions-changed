pub mod abi_json;
pub mod api;
mod arwen_denali_runner;
mod async_data;
mod blockchain_mock;
mod builtin_func_exec;
mod contract_map;
mod display_util;
mod execute_denali;
mod mock_error;
mod tx_context;

pub use async_data::*;
pub use blockchain_mock::*;
pub use builtin_func_exec::*;
pub use contract_map::*;
pub use display_util::*;
pub use mock_error::*;
pub use tx_context::*;

pub use arwen_denali_runner::denali_go;
pub use execute_denali::denali_rs;

#[macro_use]
extern crate alloc;
pub use alloc::boxed::Box;
pub use alloc::vec::Vec;

pub use std::collections::HashMap;

#[cfg(test)]
mod dharitri_codec_tests {
    use crate::api::{RustBigInt, RustBigUint};
    use core::fmt::Debug;
    use dharitri_wasm::dharitri_codec::test_util::{check_top_decode, check_top_encode};
    use dharitri_wasm::dharitri_codec::*;

    pub fn ser_deser_ok<V>(element: V, expected_bytes: &[u8])
    where
        V: TopEncode + TopDecode + PartialEq + Debug + 'static,
    {
        // serialize
        let serialized_bytes = check_top_encode(&element);
        assert_eq!(serialized_bytes.as_slice(), expected_bytes);

        // deserialize
        let deserialized: V = check_top_decode::<V>(&serialized_bytes[..]);
        assert_eq!(deserialized, element);
    }

    #[test]
    fn test_big_int_serialization() {
        ser_deser_ok(RustBigInt::from(5), &[5u8]);
        ser_deser_ok(RustBigInt::from(-5), &[251u8]);
    }

    #[test]
    fn test_big_uint_serialization() {
        ser_deser_ok(RustBigUint::from(5u32), &[5u8]);
    }
}

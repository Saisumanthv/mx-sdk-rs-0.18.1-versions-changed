use super::ArwenBigUint;
use crate::ArwenApiImpl;
use dharitri_wasm::api::CallValueApi;
use dharitri_wasm::types::{BoxedBytes, DctTokenType, TokenIdentifier};

const MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH: usize = 32;

extern "C" {
	fn checkNoPayment();

	fn bigIntNew(value: i64) -> i32;

	fn bigIntGetCallValue(dest: i32);
	fn bigIntGetDCTCallValue(dest: i32);
	fn getDCTTokenName(resultOffset: *const u8) -> i32;
	fn getDCTTokenNonce() -> u64;
	fn getDCTTokenType() -> i32;

	/// TODO: decide if it is worth using or not
	#[allow(dead_code)]
	fn getCallValueTokenName(callValueOffset: *const u8, resultOffset: *const u8) -> i32;
}

impl CallValueApi for ArwenApiImpl {
	type AmountType = ArwenBigUint;

	#[inline]
	fn check_not_payable(&self) {
		unsafe {
			checkNoPayment();
		}
	}

	fn moax_value(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn dct_value(&self) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetDCTCallValue(result);
			ArwenBigUint { handle: result }
		}
	}

	fn token(&self) -> TokenIdentifier {
		unsafe {
			let mut name_buffer = [0u8; MAX_POSSIBLE_TOKEN_IDENTIFIER_LENGTH];
			let name_len = getDCTTokenName(name_buffer.as_mut_ptr());
			if name_len == 0 {
				TokenIdentifier::moax()
			} else {
				BoxedBytes::from(&name_buffer[..name_len as usize]).into()
			}
		}
	}

	fn dct_token_nonce(&self) -> u64 {
		unsafe { getDCTTokenNonce() }
	}

	fn dct_token_type(&self) -> DctTokenType {
		unsafe { (getDCTTokenType() as u8).into() }
	}
}

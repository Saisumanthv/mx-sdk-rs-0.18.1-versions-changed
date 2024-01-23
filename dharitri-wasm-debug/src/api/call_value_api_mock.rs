use super::RustBigUint;
use crate::{TxContext, TxPanic};
use dharitri_wasm::api::CallValueApi;
use dharitri_wasm::err_msg;
use dharitri_wasm::types::{DctTokenType, TokenIdentifier};

impl CallValueApi for TxContext {
    type AmountType = RustBigUint;

    fn check_not_payable(&self) {
        if self.moax_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_MOAX.to_vec(),
            });
        }
        if self.dct_value() > 0 {
            std::panic::panic_any(TxPanic {
                status: 10,
                message: err_msg::NON_PAYABLE_FUNC_DCT.to_vec(),
            });
        }
    }

    #[inline]
    fn moax_value(&self) -> RustBigUint {
        self.tx_input_box.call_value.clone().into()
    }

    #[inline]
    fn dct_value(&self) -> RustBigUint {
        self.tx_input_box.dct_value.clone().into()
    }

    #[inline]
    fn token(&self) -> TokenIdentifier {
        TokenIdentifier::from(self.tx_input_box.dct_token_identifier.as_slice())
    }

    #[inline]
    fn dct_token_nonce(&self) -> u64 {
        // TODO: Add DCT nonce in mock
        0u64
    }

    #[inline]
    fn dct_token_type(&self) -> DctTokenType {
        // TODO: Add DCT token type in mock
        DctTokenType::Fungible
    }
}

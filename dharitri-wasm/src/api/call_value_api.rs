use super::{BigUintApi, ErrorApi};
use crate::err_msg;
use crate::types::{DctTokenType, TokenIdentifier};

pub trait CallValueApi: ErrorApi + Sized {
    /// The type of the payment arguments.
    /// Not named `BigUint` to avoid name collisions in types that implement multiple API traits.
    type AmountType: BigUintApi + 'static;

    fn check_not_payable(&self);

    /// Retrieves the MOAX call value from the VM.
    /// Will return 0 in case of an DCT transfer (cannot have both MOAX and DCT transfer simultaneously).
    fn moax_value(&self) -> Self::AmountType;

    /// Retrieves the DCT call value from the VM.
    /// Will return 0 in case of an MOAX transfer (cannot have both MOAX and DCT transfer simultaneously).
    fn dct_value(&self) -> Self::AmountType;

    /// Returns the call value token identifier of the current call.
    /// The identifier is wrapped in a TokenIdentifier object, to hide underlying logic.
    ///
    /// A note on implementation: even though the underlying api returns an empty name for MOAX,
    /// but the MOAX TokenIdentifier is serialized as `MOAX`.
    fn token(&self) -> TokenIdentifier;

    /// Returns the nonce of the received DCT token.
    /// Will return 0 in case of MOAX or fungible DCT transfer.
    fn dct_token_nonce(&self) -> u64;

    /// Returns the DCT token type.
    /// Will return "Fungible" for MOAX.
    fn dct_token_type(&self) -> DctTokenType;

    /// Will return the MOAX call value,
    /// but also fail with an error if DCT is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_moax(&self) -> Self::AmountType {
        if !self.token().is_moax() {
            self.signal_error(err_msg::NON_PAYABLE_FUNC_DCT);
        }
        self.moax_value()
    }

    /// Will return the DCT call value,
    /// but also fail with an error if MOAX or the wrong DCT token is sent.
    /// Especially used in the auto-generated call value processing.
    fn require_dct(&self, token: &[u8]) -> Self::AmountType {
        if self.token() != token {
            self.signal_error(err_msg::BAD_TOKEN_PROVIDED);
        }
        self.dct_value()
    }

    /// Returns both the call value (either MOAX or DCT) and the token identifier.
    /// Especially used in the `#[payable("*")] auto-generated snippets.
    /// The method might seem redundant, but there is such a hook in Arwen
    /// that might be used in this scenario in the future.
    fn payment_token_pair(&self) -> (Self::AmountType, TokenIdentifier) {
        let token = self.token();
        if token.is_moax() {
            (self.moax_value(), token)
        } else {
            (self.dct_value(), token)
        }
    }
}

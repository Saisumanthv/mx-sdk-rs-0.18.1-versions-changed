#![no_std]

dharitri_wasm::imports!();

/// Contract that only tests the call value features,
/// i.e. the framework/Arwen functionality for accepting MOAX and DCT payments.
#[dharitri_wasm::contract]
pub trait PayableFeatures {
    #[view]
    #[payable("*")]
    fn check_call_value(
        &self,
    ) -> MultiResult5<Self::BigUint, Self::BigUint, TokenIdentifier, Self::BigUint, TokenIdentifier>
    {
        let (pair_call_value, pair_token_name) = self.call_value().payment_token_pair();
        (
            self.call_value().moax_value(),
            self.call_value().dct_value(),
            self.call_value().token(),
            pair_call_value,
            pair_token_name,
        )
            .into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_1(
        &self,
        #[payment] payment: Self::BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_2(
        &self,
        #[payment] payment: Self::BigUint,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let (payment, _) = self.call_value().payment_token_pair();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("*")]
    fn payable_any_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        self.call_value().payment_token_pair().into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_1(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_2(
        &self,
        #[payment] payment: Self::BigUint,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("MOAX")]
    fn payable_moax_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let payment = self.call_value().moax_value();
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_1(
        &self,
        #[payment] payment: Self::BigUint,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_2(
        &self,
        #[payment] payment: Self::BigUint,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let token = self.call_value().token();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_3(
        &self,
        #[payment_token] token: TokenIdentifier,
    ) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let payment = self.call_value().dct_value();
        (payment, token).into()
    }

    #[endpoint]
    #[payable("PAYABLE-FEATURES-TOKEN")]
    fn payable_token_4(&self) -> MultiResult2<Self::BigUint, TokenIdentifier> {
        let payment = self.call_value().dct_value();
        let token = self.call_value().token();
        (payment, token).into()
    }
}

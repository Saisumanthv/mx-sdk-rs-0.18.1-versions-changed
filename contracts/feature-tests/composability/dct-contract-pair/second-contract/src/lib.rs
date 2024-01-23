#![no_std]

dharitri_wasm::imports!();

#[dharitri_wasm::contract]
pub trait SecondContract {
    #[init]
    fn init(&self, dct_token_name: TokenIdentifier) {
        self.set_contract_dct_token_name(&dct_token_name);
    }

    #[payable("*")]
    #[endpoint(acceptDctPayment)]
    fn accept_dct_payment(
        &self,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let expected_token_name = self.get_contract_dct_token_name();
        require!(actual_token_name == expected_token_name, "Wrong dct token");
        Ok(())
    }

    #[payable("*")]
    #[endpoint(rejectDctPayment)]
    fn reject_dct_payment(&self) -> SCResult<()> {
        sc_error!("Rejected")
    }

    // storage

    #[storage_set("dctTokenName")]
    fn set_contract_dct_token_name(&self, dct_token_name: &TokenIdentifier);

    #[view(getDctTokenName)]
    #[storage_get("dctTokenName")]
    fn get_contract_dct_token_name(&self) -> TokenIdentifier;
}

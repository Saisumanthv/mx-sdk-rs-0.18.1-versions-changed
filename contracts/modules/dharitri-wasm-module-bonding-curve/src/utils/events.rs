dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

#[dharitri_wasm::module]
pub trait EventsModule {
    #[event("buy-token")]
    fn buy_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

    #[event("sell-token")]
    fn sell_token_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}

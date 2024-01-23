#![no_std]

mod dns_proxy;

dharitri_wasm::imports!();

/// The module deals with registering usernames in a DNS contract.
#[dharitri_wasm::module]
pub trait DnsModule {
    #[proxy]
    fn dns_proxy(&self, to: Address) -> dns_proxy::Proxy<Self::SendApi>;

    #[payable("MOAX")]
    #[endpoint(dnsRegister)]
    fn dns_register(
        &self,
        dns_address: Address,
        name: BoxedBytes,
        #[payment] payment: Self::BigUint,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        only_owner!(self, "only owner can call dnsRegister");

        Ok(self
            .dns_proxy(dns_address)
            .register(name, payment)
            .async_call())
    }
}

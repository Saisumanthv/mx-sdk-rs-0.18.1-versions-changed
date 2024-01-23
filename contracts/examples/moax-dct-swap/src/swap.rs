#![no_std]

dharitri_wasm::imports!();
dharitri_wasm::derive_imports!();

const MOAX_NUM_DECIMALS: usize = 18;

/// Converts between MOAX and a wrapped MOAX DCT token.
///	1 MOAX = 1 wrapped MOAX and is interchangeable at all times.
/// Also manages the supply of wrapped MOAX tokens.
#[dharitri_wasm::contract]
pub trait MoaxDctSwap {
    #[init]
    fn init(&self) {}

    // endpoints - owner-only

    #[only_owner]
    #[payable("MOAX")]
    #[endpoint(issueWrappedMoax)]
    fn issue_wrapped_moax(
        &self,
        token_display_name: BoxedBytes,
        token_ticker: BoxedBytes,
        initial_supply: Self::BigUint,
        #[payment] issue_cost: Self::BigUint,
    ) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(
            self.wrapped_moax_token_id().is_empty(),
            "wrapped moax was already issued"
        );

        let caller = self.blockchain().get_caller();

        self.issue_started_event(&caller, token_ticker.as_slice(), &initial_supply);

        Ok(DCTSystemSmartContractProxy::new_proxy_obj(self.send())
            .issue_fungible(
                issue_cost,
                &token_display_name,
                &token_ticker,
                &initial_supply,
                FungibleTokenProperties {
                    num_decimals: MOAX_NUM_DECIMALS,
                    can_freeze: false,
                    can_wipe: false,
                    can_pause: false,
                    can_mint: true,
                    can_burn: false,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: false,
                },
            )
            .async_call()
            .with_callback(self.callbacks().dct_issue_callback(&caller)))
    }

    #[callback]
    fn dct_issue_callback(
        &self,
        caller: &Address,
        #[payment_token] token_identifier: TokenIdentifier,
        #[payment] returned_tokens: Self::BigUint,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        // callback is called with DCTTransfer of the newly issued token, with the amount requested,
        // so we can get the token identifier and amount from the call data
        match result {
            AsyncCallResult::Ok(()) => {
                self.issue_success_event(caller, &token_identifier, &returned_tokens);
                self.unused_wrapped_moax().set(&returned_tokens);
                self.wrapped_moax_token_id().set(&token_identifier);
            },
            AsyncCallResult::Err(message) => {
                self.issue_failure_event(caller, message.err_msg.as_slice());

                // return issue cost to the owner
                // TODO: test that it works
                if token_identifier.is_moax() && returned_tokens > 0 {
                    self.send().direct_moax(caller, &returned_tokens, &[]);
                }
            },
        }
    }

    #[only_owner]
    #[endpoint(mintWrappedMoax)]
    fn mint_wrapped_moax(&self, amount: Self::BigUint) -> SCResult<AsyncCall<Self::SendApi>> {
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        let wrapped_moax_token_id = self.wrapped_moax_token_id().get();
        let caller = self.blockchain().get_caller();
        self.mint_started_event(&caller, &amount);

        Ok(DCTSystemSmartContractProxy::new_proxy_obj(self.send())
            .mint(&wrapped_moax_token_id, &amount)
            .async_call()
            .with_callback(self.callbacks().dct_mint_callback(&caller, &amount)))
    }

    #[callback]
    fn dct_mint_callback(
        &self,
        caller: &Address,
        amount: &Self::BigUint,
        #[call_result] result: AsyncCallResult<()>,
    ) {
        match result {
            AsyncCallResult::Ok(()) => {
                self.mint_success_event(caller);
                self.unused_wrapped_moax()
                    .update(|unused_wrapped_moax| *unused_wrapped_moax += amount);
            },
            AsyncCallResult::Err(message) => {
                self.mint_failure_event(caller, message.err_msg.as_slice());
            },
        }
    }

    // endpoints

    #[payable("MOAX")]
    #[endpoint(wrapMoax)]
    fn wrap_moax(&self, #[payment] payment: Self::BigUint) -> SCResult<()> {
        require!(payment > 0, "Payment must be more than 0");
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        self.unused_wrapped_moax().update(|unused_wrapped_moax| {
            require!(
                *unused_wrapped_moax > payment,
                "Contract does not have enough wrapped MOAX. Please try again once more is minted."
            );

            *unused_wrapped_moax -= &payment;

            Ok(())
        })?;

        let caller = self.blockchain().get_caller();
        self.send().direct(
            &caller,
            &self.wrapped_moax_token_id().get(),
            0,
            &payment,
            b"wrapping",
        );

        self.wrap_moax_event(&caller, &payment);

        Ok(())
    }

    #[payable("*")]
    #[endpoint(unwrapMoax)]
    fn unwrap_moax(
        &self,
        #[payment] wrapped_moax_payment: Self::BigUint,
        #[payment_token] token_identifier: TokenIdentifier,
    ) -> SCResult<()> {
        require!(
            !self.wrapped_moax_token_id().is_empty(),
            "Wrapped MOAX was not issued yet"
        );

        let wrapped_moax_token_identifier = self.wrapped_moax_token_id().get();
        require!(
            token_identifier == wrapped_moax_token_identifier,
            "Wrong dct token"
        );

        require!(wrapped_moax_payment > 0, "Must pay more than 0 tokens!");
        // this should never happen, but we'll check anyway
        require!(
            wrapped_moax_payment
                <= self
                    .blockchain()
                    .get_sc_balance(&TokenIdentifier::moax(), 0),
            "Contract does not have enough funds"
        );

        self.unused_wrapped_moax()
            .update(|unused_wrapped_moax| *unused_wrapped_moax += &wrapped_moax_payment);

        // 1 wrapped MOAX = 1 MOAX, so we pay back the same amount
        let caller = self.blockchain().get_caller();
        self.send()
            .direct_moax(&caller, &wrapped_moax_payment, b"unwrapping");

        self.unwrap_moax_event(&caller, &wrapped_moax_payment);

        Ok(())
    }

    #[view(getLockedMoaxBalance)]
    fn get_locked_moax_balance(&self) -> Self::BigUint {
        self.blockchain()
            .get_sc_balance(&TokenIdentifier::moax(), 0)
    }

    // storage

    #[view(getWrappedMoaxTokenIdentifier)]
    #[storage_mapper("wrapped_moax_token_id")]
    fn wrapped_moax_token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;

    #[view(getUnusedWrappedMoax)]
    #[storage_mapper("unused_wrapped_moax")]
    fn unused_wrapped_moax(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    // events

    #[event("issue-started")]
    fn issue_started_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] token_ticker: &[u8],
        initial_supply: &Self::BigUint,
    );

    #[event("issue-success")]
    fn issue_success_event(
        &self,
        #[indexed] caller: &Address,
        #[indexed] token_identifier: &TokenIdentifier,
        initial_supply: &Self::BigUint,
    );

    #[event("issue-failure")]
    fn issue_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

    #[event("mint-started")]
    fn mint_started_event(&self, #[indexed] caller: &Address, amount: &Self::BigUint);

    #[event("mint-success")]
    fn mint_success_event(&self, #[indexed] caller: &Address);

    #[event("mint-failure")]
    fn mint_failure_event(&self, #[indexed] caller: &Address, message: &[u8]);

    #[event("wrap-moax")]
    fn wrap_moax_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);

    #[event("unwrap-moax")]
    fn unwrap_moax_event(&self, #[indexed] user: &Address, amount: &Self::BigUint);
}

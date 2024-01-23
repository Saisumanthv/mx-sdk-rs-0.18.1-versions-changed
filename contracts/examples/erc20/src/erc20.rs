#![no_std]

dharitri_wasm::imports!();

#[dharitri_wasm::contract]
pub trait SimpleErc20Token {
    // STORAGE

    /// Total number of tokens in existence.
    #[view(totalSupply)]
    #[storage_mapper("total_supply")]
    fn total_supply(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    /// Gets the balance of the specified address.
    ///
    /// Arguments:
    ///
    /// * `address` The address to query the the balance of
    ///
    #[view(balanceOf)]
    #[storage_mapper("balance")]
    fn token_balance(&self, address: &Address) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    /// The amount of tokens that an owner allowed to a spender.
    ///
    /// Arguments:
    ///
    /// * `owner` The address that owns the funds.
    /// * `spender` The address that will spend the funds.
    ///
    #[view(allowance)]
    #[storage_mapper("allowance")]
    fn allowance(
        &self,
        owner: &Address,
        spender: &Address,
    ) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    // FUNCTIONALITY

    /// Constructor, is called immediately after the contract is created
    /// Will set the fixed global token supply and give all the supply to the creator.
    #[init]
    fn init(&self, total_supply: &Self::BigUint) {
        let creator = self.blockchain().get_caller();

        // save total supply
        self.total_supply().set(total_supply);

        // deployer initially receives the total supply
        self.token_balance(&creator)
            .update(|balance| *balance += total_supply);
    }

    /// This method is private, deduplicates logic from transfer and transferFrom.
    fn perform_transfer(
        &self,
        sender: Address,
        recipient: Address,
        amount: Self::BigUint,
    ) -> SCResult<()> {
        // check if enough funds & decrease sender balance
        self.token_balance(&sender).update(|balance| {
            require!(amount <= *balance, "insufficient funds");

            *balance -= &amount;

            Ok(())
        })?;

        // increase recipient balance
        self.token_balance(&recipient)
            .update(|balance| *balance += &amount);

        // log operation
        self.transfer_event(&sender, &recipient, &amount);

        Ok(())
    }

    /// Transfer token to a specified address from sender.
    ///
    /// Arguments:
    ///
    /// * `to` The address to transfer to.
    ///
    #[endpoint]
    fn transfer(&self, to: Address, amount: Self::BigUint) -> SCResult<()> {
        // the sender is the caller
        let sender = self.blockchain().get_caller();
        self.perform_transfer(sender, to, amount)
    }

    /// Use allowance to transfer funds between two accounts.
    ///
    /// Arguments:
    ///
    /// * `sender` The address to transfer from.
    /// * `recipient` The address to transfer to.
    /// * `amount` the amount of tokens to be transferred.
    ///
    #[endpoint(transferFrom)]
    fn transfer_from(
        &self,
        sender: Address,
        recipient: Address,
        amount: Self::BigUint,
    ) -> SCResult<()> {
        // get caller
        let caller = self.blockchain().get_caller();

        self.allowance(&sender, &caller).update(|allowance| {
            require!(amount <= *allowance, "allowance exceeded");

            *allowance -= &amount;

            Ok(())
        })?;

        // transfer
        self.perform_transfer(sender, recipient, amount)
    }

    /// Approve the given address to spend the specified amount of tokens on behalf of the sender.
    /// It overwrites any previously existing allowance from sender to beneficiary.
    ///
    /// Arguments:
    ///
    /// * `spender` The address that will spend the funds.
    /// * `amount` The amount of tokens to be spent.
    ///
    #[endpoint]
    fn approve(&self, spender: Address, amount: Self::BigUint) -> SCResult<()> {
        // sender is the caller
        let caller = self.blockchain().get_caller();

        // store allowance
        self.allowance(&caller, &spender).set(&amount);

        // log operation
        self.approve_event(&caller, &spender, &amount);
        Ok(())
    }

    // EVENTS

    #[event("transfer")]
    fn transfer_event(
        &self,
        #[indexed] sender: &Address,
        #[indexed] recipient: &Address,
        amount: &Self::BigUint,
    );

    #[event("approve")]
    fn approve_event(
        &self,
        #[indexed] sender: &Address,
        #[indexed] recipient: &Address,
        amount: &Self::BigUint,
    );
}
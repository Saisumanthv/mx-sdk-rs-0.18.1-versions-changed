#![no_std]

dharitri_wasm::imports!();

use dharitri_wasm::HexCallDataSerializer;

const DCT_TRANSFER_STRING: &[u8] = b"DCTTransfer";
const SECOND_CONTRACT_ACCEPT_DCT_PAYMENT: &[u8] = b"acceptDctPayment";
const SECOND_CONTRACT_REJECT_DCT_PAYMENT: &[u8] = b"rejectDctPayment";

#[dharitri_wasm::contract]
pub trait FirstContract {
    #[init]
    fn init(&self, dct_token_name: TokenIdentifier, second_contract_address: Address) {
        self.set_contract_dct_token_name(&dct_token_name);
        self.set_second_contract_address(&second_contract_address);
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFull)]
    fn transfer_to_second_contract_full(
        &self,
        #[payment] dct_value: Self::BigUint,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let expected_token_name = self.get_contract_dct_token_name();

        require!(dct_value > 0, "no dct transfered!");
        require!(actual_token_name == expected_token_name, "Wrong dct token");

        self.call_dct_second_contract(
            &expected_token_name,
            &dct_value,
            &self.get_second_contract_address(),
            SECOND_CONTRACT_ACCEPT_DCT_PAYMENT,
            &[],
        );

        Ok(())
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractHalf)]
    fn transfer_to_second_contract_half(
        &self,
        #[payment] dct_value: Self::BigUint,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let expected_token_name = self.get_contract_dct_token_name();

        require!(dct_value > 0, "no dct transfered!");
        require!(actual_token_name == expected_token_name, "Wrong dct token");

        self.call_dct_second_contract(
            &expected_token_name,
            &(dct_value / Self::BigUint::from(2u32)),
            &self.get_second_contract_address(),
            SECOND_CONTRACT_ACCEPT_DCT_PAYMENT,
            &[],
        );

        Ok(())
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejected)]
    fn transfer_to_second_contract_rejected(
        &self,
        #[payment] dct_value: Self::BigUint,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let expected_token_name = self.get_contract_dct_token_name();

        require!(dct_value > 0, "no dct transfered!");
        require!(actual_token_name == expected_token_name, "Wrong dct token");

        self.call_dct_second_contract(
            &expected_token_name,
            &dct_value,
            &self.get_second_contract_address(),
            SECOND_CONTRACT_REJECT_DCT_PAYMENT,
            &[],
        );

        Ok(())
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractRejectedWithTransferAndExecute)]
    fn transfer_to_second_contract_rejected_with_transfer_and_execute(
        &self,
        #[payment] dct_value: Self::BigUint,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let second_contract_address = self.get_second_contract_address();
        let expected_token_name = self.get_contract_dct_token_name();

        require!(dct_value > 0, "no dct transfered!");
        require!(actual_token_name == expected_token_name, "Wrong dct token");

        let _ = self.send().direct_dct_execute(
            &second_contract_address,
            &expected_token_name,
            &dct_value,
            self.blockchain().get_gas_left(),
            SECOND_CONTRACT_REJECT_DCT_PAYMENT,
            &ArgBuffer::new(),
        );

        Ok(())
    }

    #[payable("*")]
    #[endpoint(transferToSecondContractFullWithTransferAndExecute)]
    fn transfer_to_second_contract_full_with_transfer_and_execute(
        &self,
        #[payment] dct_value: Self::BigUint,
        #[payment_token] actual_token_name: TokenIdentifier,
    ) -> SCResult<()> {
        let second_contract_address = self.get_second_contract_address();
        let expected_token_name = self.get_contract_dct_token_name();

        require!(dct_value > 0, "no dct transfered!");
        require!(actual_token_name == expected_token_name, "Wrong dct token");

        let _ = self.send().direct_dct_execute(
            &second_contract_address,
            &expected_token_name,
            &dct_value,
            self.blockchain().get_gas_left(),
            SECOND_CONTRACT_ACCEPT_DCT_PAYMENT,
            &ArgBuffer::new(),
        );

        Ok(())
    }

    fn call_dct_second_contract(
        &self,
        dct_token_name: &TokenIdentifier,
        amount: &Self::BigUint,
        to: &Address,
        func_name: &[u8],
        args: &[BoxedBytes],
    ) {
        let mut serializer = HexCallDataSerializer::new(DCT_TRANSFER_STRING);
        serializer.push_argument_bytes(dct_token_name.as_dct_identifier());
        serializer.push_argument_bytes(amount.to_bytes_be().as_slice());
        serializer.push_argument_bytes(func_name);
        for arg in args {
            serializer.push_argument_bytes(arg.as_slice());
        }

        self.send()
            .async_call_raw(to, &Self::BigUint::zero(), serializer.as_slice());
    }

    // storage

    #[storage_set("dctTokenName")]
    fn set_contract_dct_token_name(&self, dct_token_name: &TokenIdentifier);

    #[view(getDctTokenName)]
    #[storage_get("dctTokenName")]
    fn get_contract_dct_token_name(&self) -> TokenIdentifier;

    #[storage_set("secondContractAddress")]
    fn set_second_contract_address(&self, address: &Address);

    #[view(getSecondContractAddress)]
    #[storage_get("secondContractAddress")]
    fn get_second_contract_address(&self) -> Address;
}

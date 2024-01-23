use super::mock_error::BlockchainMockError;
use crate::contract_map::*;
use crate::display_util::*;
use crate::dct_transfer_event_log;
use crate::tx_context::*;
use alloc::boxed::Box;
use alloc::vec::Vec;
use dharitri_wasm::types::Address;
use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Write;

const DHARITRI_REWARD_KEY: &[u8] = b"DHARITRIreward";

pub type AccountStorage = HashMap<Vec<u8>, Vec<u8>>;
pub type AccountDct = HashMap<Vec<u8>, BigUint>;

pub struct AccountData {
    pub address: Address,
    pub nonce: u64,
    pub balance: BigUint,
    pub storage: AccountStorage,
    pub dct: AccountDct,
    pub username: Vec<u8>,
    pub contract_path: Option<Vec<u8>>,
    pub contract_owner: Option<Address>,
}

impl fmt::Display for AccountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut dct_buf = String::new();
        let mut dct_keys: Vec<Vec<u8>> =
            self.dct.clone().iter().map(|(k, _)| k.clone()).collect();
        dct_keys.sort();

        for key in &dct_keys {
            let value = self.dct.get(key).unwrap();
            write!(
                &mut dct_buf,
                "\n\t\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.to_bytes_be())
            )
            .unwrap();
        }

        let mut storage_buf = String::new();
        let mut storage_keys: Vec<Vec<u8>> = self.storage.iter().map(|(k, _)| k.clone()).collect();
        storage_keys.sort();

        for key in &storage_keys {
            let value = self.storage.get(key).unwrap();
            write!(
                &mut storage_buf,
                "\n\t\t\t{} -> 0x{}",
                key_hex(key.as_slice()),
                hex::encode(value.as_slice())
            )
            .unwrap();
        }

        write!(
            f,
            "AccountData {{
		nonce: {},
		balance: {},
		dct: [{} ],
		username: {},
		storage: [{} ]
	}}",
            self.nonce,
            self.balance,
            dct_buf,
            String::from_utf8(self.username.clone()).unwrap(),
            storage_buf
        )
    }
}

#[derive(Clone, Debug)]
pub struct BlockInfo {
    pub block_timestamp: u64,
    pub block_nonce: u64,
    pub block_round: u64,
    pub block_epoch: u64,
    pub block_random_seed: Box<[u8; 48]>,
}

impl BlockInfo {
    pub fn new() -> Self {
        BlockInfo {
            block_timestamp: 0,
            block_nonce: 0,
            block_round: 0,
            block_epoch: 0,
            block_random_seed: Box::from([0u8; 48]),
        }
    }
}

impl Default for BlockInfo {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BlockchainMock {
    pub accounts: HashMap<Address, AccountData>,
    pub new_addresses: HashMap<(Address, u64), Address>,
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
}

impl BlockchainMock {
    pub fn new() -> Self {
        BlockchainMock {
            accounts: HashMap::new(),
            new_addresses: HashMap::new(),
            previous_block_info: BlockInfo::new(),
            current_block_info: BlockInfo::new(),
        }
    }
}

impl Default for BlockchainMock {
    fn default() -> Self {
        Self::new()
    }
}

impl BlockchainMock {
    pub fn add_account(&mut self, acct: AccountData) {
        self.accounts.insert(acct.address.clone(), acct);
    }

    pub fn print_accounts(&self) {
        let mut accounts_buf = String::new();
        for (address, account) in &self.accounts {
            write!(
                &mut accounts_buf,
                "\n\t{} -> {}",
                address_hex(address),
                account
            )
            .unwrap();
        }
        println!("Accounts: {}", &accounts_buf);
    }

    pub fn put_new_address(
        &mut self,
        creator_address: Address,
        creator_nonce: u64,
        new_address: Address,
    ) {
        self.new_addresses
            .insert((creator_address, creator_nonce), new_address);
    }

    fn get_new_address(&self, creator_address: Address, creator_nonce: u64) -> Option<Address> {
        self.new_addresses
            .get(&(creator_address, creator_nonce))
            .cloned()
    }

    pub fn get_contract_path(&self, contract_address: &Address) -> Vec<u8> {
        if let Some(account) = self.accounts.get(contract_address) {
            if let Some(contract_path) = &account.contract_path {
                contract_path.clone()
            } else {
                panic!("Recipient account is not a smart contract");
            }
        } else {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(contract_address.as_ref()).unwrap()
            );
        }
    }

    pub fn subtract_tx_payment(
        &mut self,
        address: &Address,
        call_value: &BigUint,
    ) -> Result<(), BlockchainMockError> {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        if &sender_account.balance < call_value {
            return Err("failed transfer (insufficient funds)".into());
        }
        sender_account.balance -= call_value;
        Ok(())
    }

    pub fn subtract_tx_gas(&mut self, address: &Address, gas_limit: u64, gas_price: u64) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account not found"));
        let gas_cost = BigUint::from(gas_limit) * BigUint::from(gas_price);
        assert!(
            sender_account.balance >= gas_cost,
            "Not enough balance to pay gas upfront"
        );
        sender_account.balance -= &gas_cost;
    }

    pub fn increase_balance(&mut self, address: &Address, amount: &BigUint) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));
        account.balance += amount;
    }

    pub fn send_balance(
        &mut self,
        contract_address: &Address,
        send_balance_list: &[SendBalance],
        result_logs: &mut Vec<TxLog>,
    ) -> Result<(), BlockchainMockError> {
        for send_balance in send_balance_list {
            if send_balance.token.is_moax() {
                self.subtract_tx_payment(contract_address, &send_balance.amount)?;
                self.increase_balance(&send_balance.recipient, &send_balance.amount);
            } else {
                let dct_token_identifier = send_balance.token.as_dct_identifier();
                self.substract_dct_balance(
                    contract_address,
                    dct_token_identifier,
                    &send_balance.amount,
                );
                self.increase_dct_balance(
                    &send_balance.recipient,
                    dct_token_identifier,
                    &send_balance.amount,
                );

                let log = dct_transfer_event_log(
                    contract_address.clone(),
                    send_balance.recipient.clone(),
                    dct_token_identifier.to_vec(),
                    &send_balance.amount,
                );
                result_logs.insert(0, log); // TODO: it's a hack, should be inserted during execution, not here
            }
        }
        Ok(())
    }

    pub fn substract_dct_balance(
        &mut self,
        address: &Address,
        dct_token_identifier: &[u8],
        value: &BigUint,
    ) {
        let sender_account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Sender account {} not found", address_hex(address)));

        let dct_balance = sender_account
            .dct
            .get_mut(dct_token_identifier)
            .unwrap_or_else(|| {
                panic!(
                    "Account {} has no dct tokens with name {}",
                    address_hex(address),
                    String::from_utf8(dct_token_identifier.to_vec()).unwrap()
                )
            });

        assert!(
            *dct_balance >= *value,
            "Not enough dct balance, have {}, need at least {}",
            dct_balance,
            value
        );

        *dct_balance -= value;
    }

    pub fn increase_dct_balance(
        &mut self,
        address: &Address,
        dct_token_identifier: &[u8],
        value: &BigUint,
    ) {
        let account = self
            .accounts
            .get_mut(address)
            .unwrap_or_else(|| panic!("Receiver account not found"));

        if account.dct.contains_key(dct_token_identifier) {
            let dct_balance = account.dct.get_mut(dct_token_identifier).unwrap();
            *dct_balance += value;
        } else {
            account
                .dct
                .insert(dct_token_identifier.to_vec(), value.clone());
        }
    }

    pub fn increase_nonce(&mut self, address: &Address) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.nonce += 1;
    }

    pub fn create_account_after_deploy(
        &mut self,
        tx_input: &TxInput,
        new_storage: HashMap<Vec<u8>, Vec<u8>>,
        contract_path: Vec<u8>,
    ) -> Address {
        let sender = self
            .accounts
            .get(&tx_input.from)
            .unwrap_or_else(|| panic!("Unknown deployer"));
        let sender_nonce_before_tx = sender.nonce - 1;
        let new_address = self
            .get_new_address(tx_input.from.clone(), sender_nonce_before_tx)
            .unwrap_or_else(|| {
                panic!("Missing new address. Only explicit new deploy addresses supported")
            });
        let mut dct = HashMap::<Vec<u8>, BigUint>::new();
        if !tx_input.dct_token_identifier.is_empty() {
            dct.insert(
                tx_input.dct_token_identifier.clone(),
                tx_input.dct_value.clone(),
            );
        }

        let old_value = self.accounts.insert(
            new_address.clone(),
            AccountData {
                address: new_address.clone(),
                nonce: 0,
                balance: tx_input.call_value.clone(),
                storage: new_storage,
                dct,
                username: Vec::new(),
                contract_path: Some(contract_path),
                contract_owner: Some(tx_input.from.clone()),
            },
        );
        if old_value.is_some() {
            panic!("Account already exists at deploy address.");
        }

        new_address
    }

    pub fn increase_validator_reward(&mut self, address: &Address, amount: &BigUint) {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        account.balance += amount;
        let mut storage_v_rew =
            if let Some(old_storage_value) = account.storage.get(DHARITRI_REWARD_KEY) {
                BigUint::from_bytes_be(old_storage_value)
            } else {
                BigUint::zero()
            };
        storage_v_rew += amount;
        account
            .storage
            .insert(DHARITRI_REWARD_KEY.to_vec(), storage_v_rew.to_bytes_be());
    }

    pub fn try_set_username(&mut self, address: &Address, username: &[u8]) -> bool {
        let account = self.accounts.get_mut(address).unwrap_or_else(|| {
            panic!(
                "Account not found: {}",
                &std::str::from_utf8(address.as_ref()).unwrap()
            )
        });
        if account.username.is_empty() {
            account.username = username.to_vec();
            true
        } else {
            false
        }
    }
}

pub fn execute_tx(
    tx_context: TxContext,
    contract_identifier: &[u8],
    contract_map: &ContractMap<TxContext>,
) -> TxOutput {
    let func_name = tx_context.tx_input_box.func_name.clone();
    let contract_inst = contract_map.new_contract_instance(contract_identifier, tx_context);
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let call_successful = contract_inst.call(func_name.as_slice());
        if !call_successful {
            std::panic::panic_any(TxPanic {
                status: 1,
                message: b"invalid function (not found)".to_vec(),
            });
        }
        let context = contract_inst.into_api();
        context.into_output()
    }));
    match result {
        Ok(tx_output) => tx_output,
        Err(panic_any) => panic_result(panic_any),
    }
}

fn panic_result(panic_any: Box<dyn std::any::Any + std::marker::Send>) -> TxOutput {
    if panic_any.downcast_ref::<TxOutput>().is_some() {
        // async calls panic with the tx output directly
        // it is not a failure, simply a way to kill the execution
        return *panic_any.downcast::<TxOutput>().unwrap();
    }

    if let Some(panic_obj) = panic_any.downcast_ref::<TxPanic>() {
        return TxOutput::from_panic_obj(panic_obj);
    }

    if let Some(panic_string) = panic_any.downcast_ref::<String>() {
        return TxOutput::from_panic_string(panic_string.as_str());
    }

    TxOutput::from_panic_string("unknown panic")
}

/// Some data to get copied for the tx.
/// Would be nice maybe at some point to have a reference to the full blockchain mock in the tx context,
/// but for now, copying some data is enough.
#[derive(Clone, Debug)]
pub struct BlockchainTxInfo {
    pub previous_block_info: BlockInfo,
    pub current_block_info: BlockInfo,
    pub contract_balance: BigUint,
    pub contract_dct: HashMap<Vec<u8>, BigUint>,
    pub contract_owner: Option<Address>,
}

impl BlockchainMock {
    pub fn create_tx_info(&self, contract_address: &Address) -> BlockchainTxInfo {
        if let Some(contract) = self.accounts.get(contract_address) {
            BlockchainTxInfo {
                previous_block_info: self.previous_block_info.clone(),
                current_block_info: self.current_block_info.clone(),
                contract_balance: contract.balance.clone(),
                contract_dct: contract.dct.clone(),
                contract_owner: contract.contract_owner.clone(),
            }
        } else {
            BlockchainTxInfo {
                previous_block_info: self.previous_block_info.clone(),
                current_block_info: self.current_block_info.clone(),
                contract_balance: 0u32.into(),
                contract_dct: HashMap::new(),
                contract_owner: None,
            }
        }
    }
}

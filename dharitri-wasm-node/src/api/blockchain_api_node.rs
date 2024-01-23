use super::ArwenBigUint;
use crate::ArwenApiImpl;
use dharitri_wasm::api::BlockchainApi;
use dharitri_wasm::types::{
	Address, Box, BoxedBytes, DctTokenData, DctTokenType, TokenIdentifier, H256,
};

extern "C" {
	fn getSCAddress(resultOffset: *mut u8);
	fn getOwnerAddress(resultOffset: *mut u8);
	fn getShardOfAddress(address_ptr: *const u8) -> i32;
	fn isSmartContract(address_ptr: *const u8) -> i32;

	/// Currently not used.
	#[allow(dead_code)]
	fn blockHash(nonce: i64, resultOffset: *mut u8) -> i32;

	/// Currently not used.
	#[allow(dead_code)]
	fn getFunction(functionOffset: *const u8) -> i32;

	fn getCaller(resultOffset: *mut u8);

	fn getGasLeft() -> i64;
	fn getBlockTimestamp() -> i64;
	fn getBlockNonce() -> i64;
	fn getBlockRound() -> i64;
	fn getBlockEpoch() -> i64;
	fn getBlockRandomSeed(resultOffset: *mut u8);
	/// Currently not used.
	#[allow(dead_code)]
	fn getStateRootHash(resultOffset: *mut u8);
	fn getPrevBlockTimestamp() -> i64;
	fn getPrevBlockNonce() -> i64;
	fn getPrevBlockRound() -> i64;
	fn getPrevBlockEpoch() -> i64;
	fn getPrevBlockRandomSeed(resultOffset: *const u8);
	fn getOriginalTxHash(resultOffset: *const u8);

	// big int API
	fn bigIntNew(value: i64) -> i32;
	fn bigIntGetExternalBalance(address_ptr: *const u8, dest: i32);
	fn bigIntGetDCTExternalBalance(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
		dest: i32,
	);

	// DCT NFT
	fn getCurrentDCTNFTNonce(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
	) -> i64;
	fn getDCTTokenData(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
		valueOffset: i32,
		propertiesOffset: *const u8,
		hashOffset: *const u8,
		nameOffset: *const u8,
		attributesOffset: *const u8,
		creatorOffset: *const u8,
		royaltiesOffset: i32,
		urisOffset: *const u8,
	) -> i32;

	// helper functions for getDCTTokenData
	fn getDCTNFTNameLength(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
	) -> i32;
	fn getDCTNFTAttributeLength(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
	) -> i32;
	fn getDCTNFTURILength(
		address_ptr: *const u8,
		tokenIDOffset: *const u8,
		tokenIDLen: i32,
		nonce: i64,
	) -> i32;
}

impl BlockchainApi for ArwenApiImpl {
	type BalanceType = ArwenBigUint;

	#[inline]
	fn get_sc_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getSCAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_owner_address(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getOwnerAddress(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_shard_of_address(&self, address: &Address) -> u32 {
		unsafe { getShardOfAddress(address.as_ref().as_ptr()) as u32 }
	}

	#[inline]
	fn is_smart_contract(&self, address: &Address) -> bool {
		unsafe { isSmartContract(address.as_ref().as_ptr()) > 0 }
	}

	#[inline]
	fn get_caller(&self) -> Address {
		unsafe {
			let mut res = Address::zero();
			getCaller(res.as_mut_ptr());
			res
		}
	}

	fn get_balance(&self, address: &Address) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetExternalBalance(address.as_ref().as_ptr(), result);
			ArwenBigUint { handle: result }
		}
	}

	#[inline]
	fn get_tx_hash(&self) -> H256 {
		unsafe {
			let mut res = H256::zero();
			getOriginalTxHash(res.as_mut_ptr());
			res
		}
	}

	#[inline]
	fn get_gas_left(&self) -> u64 {
		unsafe { getGasLeft() as u64 }
	}

	#[inline]
	fn get_block_timestamp(&self) -> u64 {
		unsafe { getBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_block_nonce(&self) -> u64 {
		unsafe { getBlockNonce() as u64 }
	}

	#[inline]
	fn get_block_round(&self) -> u64 {
		unsafe { getBlockRound() as u64 }
	}

	#[inline]
	fn get_block_epoch(&self) -> u64 {
		unsafe { getBlockEpoch() as u64 }
	}

	#[inline]
	fn get_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
	}

	#[inline]
	fn get_prev_block_timestamp(&self) -> u64 {
		unsafe { getPrevBlockTimestamp() as u64 }
	}

	#[inline]
	fn get_prev_block_nonce(&self) -> u64 {
		unsafe { getPrevBlockNonce() as u64 }
	}

	#[inline]
	fn get_prev_block_round(&self) -> u64 {
		unsafe { getPrevBlockRound() as u64 }
	}

	#[inline]
	fn get_prev_block_epoch(&self) -> u64 {
		unsafe { getPrevBlockEpoch() as u64 }
	}

	#[inline]
	fn get_prev_block_random_seed(&self) -> Box<[u8; 48]> {
		unsafe {
			let mut res = [0u8; 48];
			getPrevBlockRandomSeed(res.as_mut_ptr());
			Box::new(res)
		}
	}

	#[inline]
	fn get_current_dct_nft_nonce(&self, address: &Address, token: &TokenIdentifier) -> u64 {
		unsafe {
			getCurrentDCTNFTNonce(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
			) as u64
		}
	}

	#[inline]
	fn get_dct_balance(
		&self,
		address: &Address,
		token: &TokenIdentifier,
		nonce: u64,
	) -> ArwenBigUint {
		unsafe {
			let result = bigIntNew(0);
			bigIntGetDCTExternalBalance(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
				result,
			);

			ArwenBigUint { handle: result }
		}
	}

	#[inline]
	fn get_dct_token_data(
		&self,
		address: &Address,
		token: &TokenIdentifier,
		nonce: u64,
	) -> DctTokenData<ArwenBigUint> {
		unsafe {
			let value = bigIntNew(0);
			let mut properties = [0u8; 2]; // always 2 bytes
			let mut hash = BoxedBytes::allocate(128);

			let name_len = getDCTNFTNameLength(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
			) as usize;
			let mut name_buffer = BoxedBytes::allocate(name_len);

			let attr_len = getDCTNFTAttributeLength(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
			) as usize;
			let mut attr_buffer = BoxedBytes::allocate(attr_len);

			// Current implementation of the underlying API only provides one URI
			// In the future, this might be extended to multiple URIs per token,
			// Hence the DctTokenData receives a Vec<BoxedBytes>
			let uris_len = getDCTNFTURILength(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
			) as usize;
			let mut uris_buffer = BoxedBytes::allocate(uris_len);

			let mut creator = Address::zero();
			let royalties = bigIntNew(0);

			getDCTTokenData(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
				nonce as i64,
				value,
				properties.as_mut_ptr(),
				hash.as_mut_ptr(),
				name_buffer.as_mut_ptr(),
				attr_buffer.as_mut_ptr(),
				creator.as_mut_ptr(),
				royalties,
				uris_buffer.as_mut_ptr(),
			);

			// Fungible always have a nonce of 0, so we check nonce to figure out the type
			let nonce = getCurrentDCTNFTNonce(
				address.as_ref().as_ptr(),
				token.as_ptr(),
				token.len() as i32,
			);
			let token_type = if nonce == 0 {
				DctTokenType::Fungible
			} else {
				DctTokenType::NonFungible
			};

			// Token is frozen is properties is not 0
			let frozen = properties[0] == 0 && properties[1] == 0;

			DctTokenData {
				token_type,
				amount: ArwenBigUint { handle: value },
				frozen,
				hash,
				name: name_buffer,
				attributes: attr_buffer,
				creator,
				royalties: ArwenBigUint { handle: royalties },
				uris: [uris_buffer].to_vec(),
			}
		}
	}
}

use crate::abi::TypeAbi;
use crate::Vec;
use alloc::string::String;
use dharitri_codec::dharitri_codec_derive::{NestedDecode, NestedEncode, TopDecode, TopEncode};

const DCT_ROLE_NONE: &[u8] = &[];
const DCT_ROLE_LOCAL_MINT: &[u8] = b"DCTRoleLocalMint";
const DCT_ROLE_LOCAL_BURN: &[u8] = b"DCTRoleLocalBurn";
const DCT_ROLE_NFT_CREATE: &[u8] = b"DCTRoleNFTCreate";
const DCT_ROLE_NFT_ADD_QUANTITY: &[u8] = b"DCTRoleNFTAddQuantity";
const DCT_ROLE_NFT_BURN: &[u8] = b"DCTRoleNFTBurn";

#[derive(TopDecode, TopEncode, NestedDecode, NestedEncode, Clone, PartialEq, Debug)]
pub enum DctLocalRole {
    None,
    Mint,
    Burn,
    NftCreate,
    NftAddQuantity,
    NftBurn,
}

impl DctLocalRole {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::None => 0,
            Self::Mint => 1,
            Self::Burn => 2,
            Self::NftCreate => 3,
            Self::NftAddQuantity => 4,
            Self::NftBurn => 5,
        }
    }

    pub fn as_role_name(&self) -> &'static [u8] {
        match self {
            Self::None => DCT_ROLE_NONE,
            Self::Mint => DCT_ROLE_LOCAL_MINT,
            Self::Burn => DCT_ROLE_LOCAL_BURN,
            Self::NftCreate => DCT_ROLE_NFT_CREATE,
            Self::NftAddQuantity => DCT_ROLE_NFT_ADD_QUANTITY,
            Self::NftBurn => DCT_ROLE_NFT_BURN,
        }
    }
}

impl From<u8> for DctLocalRole {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Mint,
            2 => Self::Burn,
            3 => Self::NftCreate,
            4 => Self::NftAddQuantity,
            5 => Self::NftBurn,
            _ => Self::None,
        }
    }
}

impl<'a> From<&'a [u8]> for DctLocalRole {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCT_ROLE_LOCAL_MINT {
            Self::Mint
        } else if byte_slice == DCT_ROLE_LOCAL_BURN {
            Self::Burn
        } else if byte_slice == DCT_ROLE_NFT_CREATE {
            Self::NftCreate
        } else if byte_slice == DCT_ROLE_NFT_ADD_QUANTITY {
            Self::NftAddQuantity
        } else if byte_slice == DCT_ROLE_NFT_BURN {
            Self::NftBurn
        } else {
            Self::None
        }
    }
}

impl TypeAbi for DctLocalRole {
    fn type_name() -> String {
        "DctLocalRole".into()
    }

    fn provide_type_descriptions<TDC: crate::abi::TypeDescriptionContainer>(accumulator: &mut TDC) {
        let type_name = Self::type_name();
        if !accumulator.contains_type(&type_name) {
            accumulator.reserve_type_name(type_name.clone());
            let variant_descriptions = [
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 0usize,
                    name: "None",
                    fields: Vec::new(),
                },
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 1usize,
                    name: "Mint",
                    fields: Vec::new(),
                },
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 2usize,
                    name: "Burn",
                    fields: Vec::new(),
                },
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 3usize,
                    name: "NftCreate",
                    fields: Vec::new(),
                },
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 4usize,
                    name: "NftAddQuantity",
                    fields: Vec::new(),
                },
                crate::abi::EnumVariantDescription {
                    docs: &[],
                    discriminant: 5usize,
                    name: "NftBurn",
                    fields: Vec::new(),
                },
            ]
            .to_vec();
            accumulator.insert(
                type_name.clone(),
                crate::abi::TypeDescription {
                    docs: &[],
                    name: type_name,
                    contents: crate::abi::TypeContents::Enum(variant_descriptions),
                },
            );
        }
    }
}

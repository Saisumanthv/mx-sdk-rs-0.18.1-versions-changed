use crate::abi::TypeAbi;
use alloc::string::String;
use dharitri_codec::*;

const DCT_TYPE_FUNGIBLE: &[u8] = b"FungibleDCT";
const DCT_TYPE_NON_FUNGIBLE: &[u8] = b"NonFungibleDCT";
const DCT_TYPE_SEMI_FUNGIBLE: &[u8] = b"SemiFungibleDCT";
const DCT_TYPE_INVALID: &[u8] = &[];

// Note: In the current implementation, SemiFungible is never returned
#[derive(Clone, PartialEq, Debug)]
pub enum DctTokenType {
    Fungible,
    NonFungible,
    SemiFungible,
    Invalid,
}

impl DctTokenType {
    pub fn as_u8(&self) -> u8 {
        match self {
            Self::Fungible => 0,
            Self::NonFungible => 1,
            Self::SemiFungible => 2,
            Self::Invalid => 3,
        }
    }

    pub fn as_type_name(&self) -> &'static [u8] {
        match self {
            Self::Fungible => DCT_TYPE_FUNGIBLE,
            Self::NonFungible => DCT_TYPE_NON_FUNGIBLE,
            Self::SemiFungible => DCT_TYPE_SEMI_FUNGIBLE,
            Self::Invalid => DCT_TYPE_INVALID,
        }
    }
}

impl From<u8> for DctTokenType {
    #[inline]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fungible,
            1 => Self::NonFungible,
            2 => Self::SemiFungible,
            _ => Self::Invalid,
        }
    }
}

impl<'a> From<&'a [u8]> for DctTokenType {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == DCT_TYPE_FUNGIBLE {
            Self::Fungible
        } else if byte_slice == DCT_TYPE_NON_FUNGIBLE {
            Self::NonFungible
        } else if byte_slice == DCT_TYPE_SEMI_FUNGIBLE {
            Self::SemiFungible
        } else {
            Self::Invalid
        }
    }
}

impl NestedEncode for DctTokenType {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_u8().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_u8().dep_encode_or_exit(dest, c, exit);
    }
}

impl TopEncode for DctTokenType {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_u8().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_u8().top_encode_or_exit(output, c, exit);
    }
}

impl NestedDecode for DctTokenType {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(Self::from(u8::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Self::from(u8::dep_decode_or_exit(input, c, exit))
    }
}

impl TopDecode for DctTokenType {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(Self::from(u8::top_decode(input)?))
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        Self::from(u8::top_decode_or_exit(input, c, exit))
    }
}

impl TypeAbi for DctTokenType {
    fn type_name() -> String {
        "DctTokenType".into()
    }
}

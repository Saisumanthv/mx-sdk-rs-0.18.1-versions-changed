use super::BoxedBytes;
use crate::abi::TypeAbi;
use alloc::string::String;
use dharitri_codec::*;

/// Specialized type for handling token identifiers.
/// It wraps a BoxedBytes with the full ASCII name of the token.
/// MOAX is stored as an empty name.
///
/// Not yet implemented, but we might add additional restrictions when deserializing as argument.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenIdentifier(BoxedBytes);

impl TokenIdentifier {
    /// This special representation is interpreted as the MOAX token.
    pub const MOAX_REPRESENTATION: &'static [u8] = b"MOAX";

    pub const TICKER_MIN_LENGTH: usize = 3;
    pub const TICKER_MAX_LENGTH: usize = 10;
    pub const ADDITIONAL_RANDOM_CHARS_LENGTH: usize = 6;
    // +1 because of the '-' (dash) between ticker and the random chars
    pub const IDENTIFIER_MIN_LENGTH: usize =
        Self::TICKER_MIN_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;
    pub const IDENTIFIER_MAX_LENGTH: usize =
        Self::TICKER_MAX_LENGTH + Self::ADDITIONAL_RANDOM_CHARS_LENGTH + 1;

    pub const DASH_CHARACTER: u8 = b'-';

    /// New instance of the special MOAX token representation.
    pub fn moax() -> Self {
        TokenIdentifier(BoxedBytes::empty())
    }

    #[inline]
    pub fn is_moax(&self) -> bool {
        self.is_empty()
    }

    #[inline]
    pub fn is_dct(&self) -> bool {
        !self.is_moax()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn into_boxed_bytes(self) -> BoxedBytes {
        self.0
    }

    #[deprecated(
        note = "Please use the as_dct_identifier method instead, its name is more suggestive."
    )]
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_dct_identifier(&self) -> &[u8] {
        self.0.as_slice()
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_name(&self) -> &[u8] {
        if self.is_moax() {
            TokenIdentifier::MOAX_REPRESENTATION
        } else {
            self.0.as_slice()
        }
    }

    pub fn is_valid_dct_identifier(&self) -> bool {
        if self.is_moax() {
            return false;
        }

        let id_len = self.0.len();
        #[allow(clippy::manual_range_contains)]
        if id_len < Self::IDENTIFIER_MIN_LENGTH || id_len > Self::IDENTIFIER_MAX_LENGTH {
            return false;
        }

        let id_as_slice = self.0.as_slice();

        let lowercase_letter_range = &b'a'..=&b'z';
        let uppercase_letter_range = &b'A'..=&b'Z';
        let number_range = &b'0'..=&b'9';

        // ticker must be all uppercase alphanumeric
        let ticker_len = id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH - 1;
        let ticker = &id_as_slice[..ticker_len];
        for ticker_char in ticker {
            let is_uppercase_letter = uppercase_letter_range.contains(&ticker_char);
            let is_number = number_range.contains(&ticker_char);

            if !is_uppercase_letter && !is_number {
                return false;
            }
        }

        let dash_position = ticker_len;
        if id_as_slice[dash_position] != Self::DASH_CHARACTER {
            return false;
        }

        // random chars are alphanumeric lowercase
        let random_chars = &id_as_slice[(id_len - Self::ADDITIONAL_RANDOM_CHARS_LENGTH)..];
        for rand_char in random_chars {
            let is_lowercase_letter = lowercase_letter_range.contains(&rand_char);
            let is_number = number_range.contains(&rand_char);

            if !is_lowercase_letter && !is_number {
                return false;
            }
        }

        true
    }
}

impl AsRef<[u8]> for TokenIdentifier {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<BoxedBytes> for TokenIdentifier {
    #[inline]
    fn from(boxed_bytes: BoxedBytes) -> Self {
        if boxed_bytes.as_slice() == TokenIdentifier::MOAX_REPRESENTATION {
            TokenIdentifier::moax()
        } else {
            TokenIdentifier(boxed_bytes)
        }
    }
}

impl<'a> From<&'a [u8]> for TokenIdentifier {
    #[inline]
    fn from(byte_slice: &'a [u8]) -> Self {
        if byte_slice == TokenIdentifier::MOAX_REPRESENTATION {
            TokenIdentifier::moax()
        } else {
            TokenIdentifier(BoxedBytes::from(byte_slice))
        }
    }
}

impl PartialEq<&[u8]> for TokenIdentifier {
    #[inline]
    fn eq(&self, other: &&[u8]) -> bool {
        if self.is_moax() {
            *other == TokenIdentifier::MOAX_REPRESENTATION
        } else {
            self.0.as_slice().eq(*other)
        }
    }
}

impl NestedEncode for TokenIdentifier {
    #[inline]
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        self.as_name().dep_encode(dest)
    }

    #[inline]
    fn dep_encode_or_exit<O: NestedEncodeOutput, ExitCtx: Clone>(
        &self,
        dest: &mut O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_name().dep_encode_or_exit(dest, c, exit);
    }
}

impl TopEncode for TokenIdentifier {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        self.as_name().top_encode(output)
    }

    #[inline]
    fn top_encode_or_exit<O: TopEncodeOutput, ExitCtx: Clone>(
        &self,
        output: O,
        c: ExitCtx,
        exit: fn(ExitCtx, EncodeError) -> !,
    ) {
        self.as_name().top_encode_or_exit(output, c, exit);
    }
}

impl NestedDecode for TokenIdentifier {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        Ok(TokenIdentifier::from(BoxedBytes::dep_decode(input)?))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        TokenIdentifier::from(BoxedBytes::dep_decode_or_exit(input, c, exit))
    }
}

impl TopDecode for TokenIdentifier {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        Ok(TokenIdentifier::from(BoxedBytes::top_decode(input)?))
    }

    fn top_decode_or_exit<I: TopDecodeInput, ExitCtx: Clone>(
        input: I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        TokenIdentifier::from(BoxedBytes::top_decode_or_exit(input, c, exit))
    }
}

impl TypeAbi for TokenIdentifier {
    fn type_name() -> String {
        "TokenIdentifier".into()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use dharitri_codec::test_util::*;

    #[test]
    fn test_moax() {
        assert!(TokenIdentifier::moax().is_moax());
    }

    #[test]
    fn test_codec() {
        check_top_encode_decode(
            TokenIdentifier::moax(),
            TokenIdentifier::MOAX_REPRESENTATION,
        );
        check_dep_encode_decode(
            TokenIdentifier::moax(),
            dep_encode_to_vec_or_panic(&TokenIdentifier::MOAX_REPRESENTATION).as_slice(),
        );

        // also allowed
        assert_eq!(
            TokenIdentifier::moax(),
            check_top_decode::<TokenIdentifier>(&[])
        );
        assert_eq!(
            TokenIdentifier::moax(),
            check_dep_decode::<TokenIdentifier>(&[0, 0, 0, 0])
        );
    }

    #[test]
    fn test_is_valid_dct_identifier() {
        // valid identifier
        assert!(TokenIdentifier::from(&b"ALC-6258d2"[..]).is_valid_dct_identifier());

        // valid identifier with numbers in ticker
        assert!(TokenIdentifier::from(&b"ALC123-6258d2"[..]).is_valid_dct_identifier());

        // valid ticker only numbers
        assert!(TokenIdentifier::from(&b"12345-6258d2"[..]).is_valid_dct_identifier());

        // missing dash
        assert!(!TokenIdentifier::from(&b"ALC6258d2"[..]).is_valid_dct_identifier());

        // wrong dash position
        assert!(!TokenIdentifier::from(&b"AL-C6258d2"[..]).is_valid_dct_identifier());

        // lowercase ticker
        assert!(!TokenIdentifier::from(&b"alc-6258d2"[..]).is_valid_dct_identifier());

        // uppercase random chars
        assert!(!TokenIdentifier::from(&b"ALC-6258D2"[..]).is_valid_dct_identifier());

        // too many random chars
        assert!(!TokenIdentifier::from(&b"ALC-6258d2ff"[..]).is_valid_dct_identifier());

        // ticker too short
        assert!(!TokenIdentifier::from(&b"AL-6258d2"[..]).is_valid_dct_identifier());

        // ticker too long
        assert!(!TokenIdentifier::from(&b"ALCCCCCCCCC-6258d2"[..]).is_valid_dct_identifier());
    }
}

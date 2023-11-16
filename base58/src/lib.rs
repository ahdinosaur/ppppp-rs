use base_x;

const BASE58_BITCOIN_ALPHABET: &[u8; 58] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode(input: &[u8]) -> String {
    base_x::encode(BASE58_BITCOIN_ALPHABET as &[u8], input)
}

pub fn decode(input: &str) -> Result<Vec<u8>, base_x::DecodeError> {
    base_x::decode(BASE58_BITCOIN_ALPHABET as &[u8], input)
}

pub use base_x::DecodeError;

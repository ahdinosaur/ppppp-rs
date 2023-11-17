use ppppp_bytes::{impl_as_bytes_outputs, impl_from_bytes_inputs, AsBytes, FromBytes};
use ppppp_crypto::Hash;
use serde::{Deserialize, Serialize, Serializer};
use std::{
    convert::{Infallible, TryFrom},
    fmt::Display,
    str::FromStr,
};

/// A message metadata hash
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MsgMetadataHash([u8; 16]);

impl FromBytes<16> for MsgMetadataHash {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Ok(Self(*bytes))
    }
}

impl AsBytes<16> for MsgMetadataHash {
    fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl_from_bytes_inputs!(MsgMetadataHash, 16_usize);
impl_as_bytes_outputs!(MsgMetadataHash, 16_usize);

impl MsgMetadataHash {
    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        let bytes = bytes[0..16].try_into().unwrap();
        Self(bytes)
    }
}

/// A message data hash
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MsgDataHash([u8; 16]);

impl FromBytes<16> for MsgDataHash {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 16]) -> Result<Self, Self::Error> {
        Ok(Self(*bytes))
    }
}

impl AsBytes<16> for MsgDataHash {
    fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }
}

impl_from_bytes_inputs!(MsgDataHash, 16_usize);
impl_as_bytes_outputs!(MsgDataHash, 16_usize);

impl MsgDataHash {
    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        let bytes = bytes[0..16].try_into().unwrap();
        Self(bytes)
    }
}

#[cfg(test)]
mod tests {
    use ppppp_bytes::DeserializeBytesError;

    use super::*;

    #[test]
    fn test_msg_hash_roundtrip() -> Result<(), DeserializeBytesError> {
        let msg_hash_str = "Cz1jtXr2oBrhk8czWiz6kH";
        let msg_hash = MsgMetadataHash::from_base58(msg_hash_str)?;
        assert_eq!(msg_hash_str, msg_hash.to_string());
        Ok(())
    }
}

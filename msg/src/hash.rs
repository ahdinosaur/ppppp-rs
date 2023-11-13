use ppppp_base58 as base58;
use ppppp_crypto::Hash;
use serde::{Deserialize, Serialize, Serializer};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[derive(Debug, ThisError)]
pub enum HashFromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

/// A message metadata hash
#[derive(
    Clone, Debug, FromZeroes, FromBytes, AsBytes, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(C)]
#[serde(try_from = "String")]
pub struct MsgMetadataHash([u8; 16]);

impl MsgMetadataHash {
    pub const BYTE_SIZE: usize = 16_usize;

    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        let bytes = bytes[0..16].try_into().unwrap();
        Self(bytes)
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, HashFromBase58Error> {
        let data = base58::decode(base58_str).map_err(HashFromBase58Error::Decode)?;
        if data.len() != 16 {
            return Err(HashFromBase58Error::Size { size: data.len() });
        }
        let key = Self::read_from(&data).unwrap();
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.as_bytes();
        base58::encode(data)
    }
}

impl Serialize for MsgMetadataHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for MsgMetadataHash {
    type Error = HashFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        MsgMetadataHash::from_base58(&value)
    }
}

impl FromStr for MsgMetadataHash {
    type Err = HashFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MsgMetadataHash::from_base58(s)
    }
}

impl From<&MsgMetadataHash> for String {
    fn from(value: &MsgMetadataHash) -> String {
        value.to_string()
    }
}

impl Display for MsgMetadataHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

/// A message data hash
#[derive(
    Clone, Debug, FromZeroes, FromBytes, AsBytes, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(C)]
#[serde(try_from = "String")]
pub struct MsgDataHash([u8; 16]);

impl MsgDataHash {
    pub const BYTE_SIZE: usize = 16_usize;

    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        let bytes = bytes[0..16].try_into().unwrap();
        Self(bytes)
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, HashFromBase58Error> {
        let data = base58::decode(base58_str).map_err(HashFromBase58Error::Decode)?;
        if data.len() != 16 {
            return Err(HashFromBase58Error::Size { size: data.len() });
        }
        let key = Self::read_from(&data).unwrap();
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.as_bytes();
        base58::encode(data)
    }
}

impl Serialize for MsgDataHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for MsgDataHash {
    type Error = HashFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        MsgDataHash::from_base58(&value)
    }
}

impl FromStr for MsgDataHash {
    type Err = HashFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MsgDataHash::from_base58(s)
    }
}

impl From<&MsgDataHash> for String {
    fn from(value: &MsgDataHash) -> String {
        value.to_string()
    }
}

impl Display for MsgDataHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_hash_roundtrip() -> Result<(), HashFromBase58Error> {
        let msg_hash_str = "Cz1jtXr2oBrhk8czWiz6kH";
        let msg_hash = MsgMetadataHash::from_base58(msg_hash_str)?;
        assert_eq!(msg_hash_str, msg_hash.to_string());
        Ok(())
    }
}

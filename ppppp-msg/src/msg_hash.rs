use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;
use ppppp_base58 as base58;

#[derive(Debug, ThisError)]
pub enum IdError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct MsgHash(Vec<u8>);

impl MsgHash {
    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        Self(Vec::from(&bytes[0..16]))
    }

    pub fn from_str(id_str: &str) -> Result<Self, IdError> {
        let data = base58::decode(id_str)?;
        assert_eq!(data.len(), 16);
        Ok(Self(data))
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        base58::encode(data)
    }
}

impl TryFrom<String> for MsgHash {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        MsgHash::from_str(&value)
    }
}

impl FromStr for MsgHash {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MsgHash::from_str(&s)
    }
}

impl From<&MsgHash> for String {
    fn from(value: &MsgHash) -> String {
        value.to_string()
    }
}

impl Display for MsgHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl AsRef<[u8]> for MsgHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_hash_roundtrip() -> Result<(), IdError> {
        let msg_hash_str = "Cz1jtXr2oBrhk8czWiz6kH";
        let msg_hash = MsgHash::from_str(msg_hash_str)?;
        assert_eq!(msg_hash_str, msg_hash.to_string());
        Ok(())
    }
}

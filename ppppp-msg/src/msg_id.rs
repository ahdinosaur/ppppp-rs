use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;

use crate::base58;

#[derive(Debug, ThisError)]
pub enum IdError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct MsgId(Vec<u8>);

impl MsgId {
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

impl TryFrom<String> for MsgId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        MsgId::from_str(&value)
    }
}

impl FromStr for MsgId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        MsgId::from_str(&s)
    }
}

impl From<&MsgId> for String {
    fn from(value: &MsgId) -> String {
        value.to_string()
    }
}

impl Display for MsgId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl AsRef<[u8]> for MsgId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_id_roundtrip() -> Result<(), IdError> {
        let msg_id_str = "Cz1jtXr2oBrhk8czWiz6kH";
        let msg_id = MsgId::from_str(msg_id_str)?;
        assert_eq!(msg_id_str, msg_id.to_string());
        Ok(())
    }
}

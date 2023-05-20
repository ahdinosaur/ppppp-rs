use ed25519_dalek::PublicKey;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error as ThisError;

use crate::base58;

#[derive(Debug, ThisError)]
pub enum IdError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct KeyId(Vec<u8>);

impl KeyId {
    pub fn from_pubkey(pubkey: PublicKey) -> Self {
        let bytes = pubkey.as_bytes();
        let vec = bytes.to_vec();
        Self(vec)
    }

    pub fn from_str(id_str: &str) -> Result<Self, IdError> {
        let data = base58::decode(id_str)?;
        assert_eq!(data.len(), 32);
        Ok(Self(data))
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        base58::encode(data)
    }
}

impl TryFrom<String> for KeyId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        KeyId::from_str(&value)
    }
}

impl FromStr for KeyId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        KeyId::from_str(&s)
    }
}

impl From<&KeyId> for String {
    fn from(value: &KeyId) -> String {
        value.to_string()
    }
}

impl ToString for KeyId {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

impl AsRef<[u8]> for KeyId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_id_roundtrip() -> Result<(), IdError> {
        let msg_id_str = "4mjQ5aJu378cEu6TksRG3uXAiKFiwGjYQtWAjfVjDAJW";
        let msg_id = KeyId::from_str(msg_id_str)?;
        assert_eq!(msg_id_str, msg_id.to_string());
        Ok(())
    }
}

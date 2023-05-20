use blake3::Hash;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error as ThisError;

use crate::base58;

#[derive(Debug, ThisError)]
pub enum ContentHashError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "String")]
pub struct ContentHash(Vec<u8>);

impl ContentHash {
    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        Self(Vec::from(&bytes[0..16]))
    }

    pub fn from_str(id_str: &str) -> Result<Self, ContentHashError> {
        let data = Self::decode_data(id_str)?;
        assert_eq!(data.len(), 16);
        Ok(Self(data))
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        format!("{}", Self::encode_data(data))
    }

    fn encode_data(data: &[u8]) -> String {
        base58::encode(data)
    }

    fn decode_data(data_str: &str) -> Result<Vec<u8>, ContentHashError> {
        let data = base58::decode(data_str)?;
        Ok(data)
    }
}

impl TryFrom<String> for ContentHash {
    type Error = ContentHashError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        ContentHash::from_str(&value)
    }
}

impl FromStr for ContentHash {
    type Err = ContentHashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ContentHash::from_str(&s)
    }
}

impl From<&ContentHash> for String {
    fn from(value: &ContentHash) -> String {
        value.to_string()
    }
}

impl ToString for ContentHash {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

impl AsRef<[u8]> for ContentHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

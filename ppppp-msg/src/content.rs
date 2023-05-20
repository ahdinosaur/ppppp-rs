use base_x::{decode as b58decode, encode as b58encode, DecodeError as B58DecodeError};
use blake3::{Hash, Hasher};
use json_canon::to_writer as canon_json_to_writer;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error as ThisError;

const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

#[derive(Debug, ThisError)]
pub enum ContentHashError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] B58DecodeError),
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
        b58encode(BASE58_ALPHABET as &[u8], data)
    }

    fn decode_data(data_str: &str) -> Result<Vec<u8>, ContentHashError> {
        let data = b58decode(BASE58_ALPHABET as &[u8], data_str)?;
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

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "Value")]
pub struct Content(Value);

impl Content {
    pub fn to_hash(&self) -> ContentHash {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self.0).unwrap();
        let hash = hasher.finalize();
        ContentHash::from_hash(hash)
    }
}

impl From<Value> for Content {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json::json;

    #[test]
    fn test_hello_world() {
        let value = json!({
            "text": "hello world!"
        });
        let content: Content = value.into();
        let hash: ContentHash = content.to_hash();
        assert_eq!(hash.to_string(), "Cz1jtXr2oBrhk8czWiz6kH");
    }
}

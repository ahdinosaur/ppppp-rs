use blake3::Hash;
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
pub struct MsgId(Vec<u8>);

impl MsgId {
    pub fn from_hash(hash: Hash) -> Self {
        let bytes = hash.as_bytes();
        Self(Vec::from(&bytes[0..16]))
    }

    pub fn from_str(s: &str) -> Result<Self, IdError> {
        if Self::is_uri_str(s) {
            Self::from_uri_str(s)
        } else {
            Self::from_id_str(s)
        }
    }

    pub fn from_id_str(id_str: &str) -> Result<Self, IdError> {
        let data = Self::decode_data(id_str)?;
        assert_eq!(data.len(), 16);
        Ok(Self(data))
    }

    pub fn from_uri_str(uri_str: &str) -> Result<Self, IdError> {
        let id_str = &uri_str[17..];
        println!("{}", id_str);
        Self::from_id_str(id_str)
    }

    pub fn to_id_string(&self) -> String {
        let data = self.0.as_slice();
        format!("{}", Self::encode_data(data))
    }

    pub fn to_uri_string(&self) -> String {
        let data = self.0.as_slice();
        format!("ppppp:message/v1/{}", Self::encode_data(data))
    }

    fn encode_data(data: &[u8]) -> String {
        base58::encode(data)
    }

    fn decode_data(data_str: &str) -> Result<Vec<u8>, IdError> {
        let data = base58::decode(data_str)?;
        Ok(data)
    }

    fn is_uri_str(maybe_uri_str: &str) -> bool {
        maybe_uri_str.starts_with("ppppp:message/v1/")
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
        value.to_id_string()
    }
}

impl ToString for MsgId {
    fn to_string(&self) -> String {
        self.to_id_string()
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
        let msg_id = MsgId::from_id_str(msg_id_str)?;
        assert_eq!(msg_id_str, msg_id.to_id_string());
        Ok(())
    }

    #[test]
    fn test_msg_uri_roundtrip() -> Result<(), IdError> {
        let msg_uri_str = "ppppp:message/v1/Cz1jtXr2oBrhk8czWiz6kH";
        let msg_id = MsgId::from_uri_str(msg_uri_str)?;
        assert_eq!(msg_uri_str, msg_id.to_uri_string());
        Ok(())
    }
}

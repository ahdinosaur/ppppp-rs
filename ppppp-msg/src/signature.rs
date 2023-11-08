use ed25519_dalek::Signature as Ed25519Signature;

use ppppp_base58 as base58;
use serde::{Deserialize, Serialize, Serializer};
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum SignatureError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Signature([u8; 64]);

impl Signature {
    pub fn from_signature(signature: Ed25519Signature) -> Self {
        let bytes = signature.to_bytes();
        Self(bytes)
    }

    pub fn from_str(id_str: &str) -> Result<Self, SignatureError> {
        let data = Self::decode_data(id_str)?;
        assert_eq!(data.len(), 64);
        Ok(Self(data.try_into().unwrap()))
    }

    pub fn to_signature(&self) -> Ed25519Signature {
        let bytes = self.0.as_ref().try_into().unwrap();
        Ed25519Signature::from_bytes(bytes)
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        format!("{}", Self::encode_data(data))
    }

    fn encode_data(data: &[u8]) -> String {
        base58::encode(data)
    }

    fn decode_data(data_str: &str) -> Result<Vec<u8>, SignatureError> {
        let data = base58::decode(data_str)?;
        Ok(data)
    }
}

impl Serialize for Signature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for Signature {
    type Error = SignatureError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Signature::from_str(&value)
    }
}

impl FromStr for Signature {
    type Err = SignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Signature::from_str(&s)
    }
}

impl From<&Signature> for String {
    fn from(value: &Signature) -> String {
        value.to_string()
    }
}

impl ToString for Signature {
    fn to_string(&self) -> String {
        self.to_string()
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

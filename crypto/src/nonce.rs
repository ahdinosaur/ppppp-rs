use std::{fmt::Display, str::FromStr};

use ppppp_base58 as base58;
use serde::{Deserialize, Serialize, Serializer};
use zerocopy::{AsBytes, FromBytes, FromZeroes};

#[derive(Debug, thiserror::Error)]
pub enum NonceFromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

/// A 32 byte nonce
#[derive(
    Clone, Debug, FromZeroes, FromBytes, AsBytes, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(C)]
#[serde(try_from = "String")]
pub struct Nonce([u8; 32]);

impl Nonce {
    pub const BYTE_SIZE: usize = 32_usize;

    pub fn from_base58(base58_str: &str) -> Result<Self, NonceFromBase58Error> {
        let data = base58::decode(base58_str).map_err(NonceFromBase58Error::Decode)?;
        if data.len() != 32 {
            return Err(NonceFromBase58Error::Size { size: data.len() });
        }
        let key = Self::read_from(&data).unwrap();
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.as_bytes();
        base58::encode(data)
    }
}

impl Serialize for Nonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for Nonce {
    type Error = NonceFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Nonce::from_base58(&value)
    }
}

impl FromStr for Nonce {
    type Err = NonceFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Nonce::from_base58(s)
    }
}

impl From<&Nonce> for String {
    fn from(value: &Nonce) -> String {
        value.to_string()
    }
}

impl Display for Nonce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

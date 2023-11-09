use blake3::{Hash as CryptoHash, Hasher as CryptoHasher};
use ppppp_base58 as base58;
use serde::{Deserialize, Serialize, Serializer};
use std::{convert::TryFrom, io::Write, str::FromStr};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum HashFromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

/// A cryptographic hash
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
pub struct Hash(CryptoHash);

impl Hash {
    pub const BYTE_SIZE: usize = 32_usize;

    pub fn from_bytes(bytes: &[u8; Self::BYTE_SIZE]) -> Self {
        Self(CryptoHash::from_bytes(*bytes))
    }

    pub fn as_bytes(&self) -> &[u8; Self::BYTE_SIZE] {
        self.0.as_bytes()
    }

    pub fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        *self.as_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, HashFromBase58Error> {
        let data = base58::decode(base58_str).map_err(HashFromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(HashFromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.as_bytes();
        base58::encode(data)
    }
}

impl Serialize for Hash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for Hash {
    type Error = HashFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Hash::from_base58(&value)
    }
}

impl FromStr for Hash {
    type Err = HashFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Hash::from_base58(s)
    }
}

impl From<&Hash> for String {
    fn from(value: &Hash) -> String {
        value.to_string()
    }
}

impl ToString for Hash {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}

pub struct Hasher(CryptoHasher);

impl Hasher {
    pub fn new() -> Self {
        Self(CryptoHasher::new())
    }

    pub fn finalize(&self) -> Hash {
        Hash(self.0.finalize())
    }
}

impl Default for Hasher {
    fn default() -> Self {
        Self::new()
    }
}

impl Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_hash_roundtrip() -> Result<(), HashFromBase58Error> {
        let msg_hash_str = "Cz1jtXr2oBrhk8czWiz6kHCz1jtXr2oBrhk8czWiz6kH";
        let msg_hash = Hash::from_base58(msg_hash_str)?;
        assert_eq!(msg_hash_str, msg_hash.to_string());
        Ok(())
    }
}

use std::{
    fmt::{self, Display},
    marker::PhantomData,
    str::FromStr,
};

use ppppp_base58 as base58;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Debug, thiserror::Error)]
pub enum FromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

pub trait FromBytes<const BYTE_SIZE: usize> {
    fn from_bytes(bytes: &[u8; BYTE_SIZE]) -> Self;

    fn from_base58(base58_str: &str) -> Result<Self, FromBase58Error> {
        let data = base58::decode(base58_str).map_err(FromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(FromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }
}

impl<const BYTE_SIZE: usize, B: FromBytes<BYTE_SIZE>> TryFrom<String> for B {
    type Error = FromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        B::from_base58(&value)
    }
}

impl<const BYTE_SIZE: usize, B: FromBytes<BYTE_SIZE>> FromStr for B {
    type Err = FromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        B::from_base58(s)
    }
}

struct FromBytesVisitor<const BYTE_SIZE: usize, B: FromBytes<BYTE_SIZE>> {
    b: PhantomData<B>,
}

impl<'de, const BYTE_SIZE: usize, B: FromBytes<BYTE_SIZE>> Visitor<'de>
    for FromBytesVisitor<BYTE_SIZE, B>
{
    type Value = B;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("base58 string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(base58::decode(value))
    }
}

impl<'de, const BYTE_SIZE: usize, B: FromBytes<BYTE_SIZE>> Deserialize<'de> for B {
    fn deserialize<D>(deserializer: D) -> Result<B, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromBytesVisitor { b: PhantomData })
    }
}

pub trait AsBytes<const BYTE_SIZE: usize> {
    fn as_bytes(bytes: &[u8; BYTE_SIZE]) -> Self;

    fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
    }
}

impl<const BYTE_SIZE: usize, B: AsBytes<BYTE_SIZE>> From<&B> for String {
    fn from(value: &B) -> String {
        value.to_string()
    }
}

impl<const BYTE_SIZE: usize, B: AsBytes<BYTE_SIZE>> Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl<const BYTE_SIZE: usize, B: AsBytes<BYTE_SIZE>> Serialize for B {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // assert!(true);
    }
}

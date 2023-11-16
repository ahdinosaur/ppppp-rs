use generic_array::{ArrayLength, GenericArray};
use ppppp_base58 as base58;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{self, Display},
    marker::PhantomData,
    str::FromStr,
};

struct Bytes<N: ArrayLength> {
    data: GenericArray<u8, N>,
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeBytesError {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

pub trait FromBytes {
    type LEN: ArrayLength;

    fn from_bytes<B: Into<Bytes<Self::LEN>>>(bytes: B) -> Self;

    fn from_base58(base58_str: &str) -> Result<Self, DeserializeBytesError> {
        let data = base58::decode(base58_str).map_err(DeserializeBytesError::Decode)?;
        if data.len() != 64 {
            return Err(DeserializeBytesError::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }
}

impl<B: FromBytes> TryFrom<String> for B {
    type Error = DeserializeBytesError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        B::from_base58(&value)
    }
}

impl<B: FromBytes> FromStr for B {
    type Err = DeserializeBytesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        B::from_base58(s)
    }
}

struct FromBytesVisitor<B: FromBytes> {
    b: PhantomData<B>,
}

impl<'de, B: FromBytes> Visitor<'de> for FromBytesVisitor<B> {
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

impl<'de, B: FromBytes> Deserialize<'de> for B {
    fn deserialize<D>(deserializer: D) -> Result<B, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(FromBytesVisitor { b: PhantomData })
    }
}

pub trait AsBytes {
    type LEN: ArrayLength;

    fn as_bytes<B: Into<Bytes<Self::LEN>>>(&self) -> B;

    fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
    }
}

impl<B: AsBytes> From<&B> for String {
    fn from(value: &B) -> String {
        value.to_string()
    }
}

impl<B: AsBytes> Display for B {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl<B: AsBytes> Serialize for B {
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

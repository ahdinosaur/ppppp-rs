use ed25519_dalek::{
    Signature as CryptoSignature, SignatureError as CryptoSignatureError,
    SigningKey as CryptoSigningKey, VerifyingKey as CryptoVerifyingKey,
};
use ppppp_base58 as base58;
use serde::{Deserialize, Serialize, Serializer};
use std::{convert::TryFrom, str::FromStr};
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum FromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
    #[error("Invalid crypto: {0}")]
    Crypto(#[source] CryptoSignatureError),
}

/// A secret key to sign messages
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct SigningKey(CryptoSigningKey);

impl SigningKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(CryptoSigningKey::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, FromBase58Error> {
        let data = base58::decode(base58_str).map_err(FromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(FromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
    }
}

impl Serialize for SigningKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for SigningKey {
    type Error = FromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SigningKey::from_base58(&value)
    }
}

impl FromStr for SigningKey {
    type Err = FromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SigningKey::from_base58(s)
    }
}

impl From<&SigningKey> for String {
    fn from(value: &SigningKey) -> String {
        value.to_string()
    }
}

impl ToString for SigningKey {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}

/// A public key to verify signatures
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct VerifyingKey(CryptoVerifyingKey);

impl VerifyingKey {
    pub fn from_bytes(bytes: &[u8; 32]) -> Result<Self, CryptoSignatureError> {
        Ok(Self(CryptoVerifyingKey::from_bytes(bytes)?))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, FromBase58Error> {
        let data = base58::decode(base58_str).map_err(FromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(FromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes).map_err(FromBase58Error::Crypto)?;
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
    }
}

impl Serialize for VerifyingKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl TryFrom<String> for VerifyingKey {
    type Error = FromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        VerifyingKey::from_base58(&value)
    }
}

impl FromStr for VerifyingKey {
    type Err = FromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VerifyingKey::from_base58(s)
    }
}

impl From<&VerifyingKey> for String {
    fn from(value: &VerifyingKey) -> String {
        value.to_string()
    }
}

impl ToString for VerifyingKey {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}

/// A public key to verify signatures
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Signature(CryptoSignature);

impl Signature {
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        Self(CryptoSignature::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, FromBase58Error> {
        let data = base58::decode(base58_str).map_err(FromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(FromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
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
    type Error = FromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Signature::from_base58(&value)
    }
}

impl FromStr for Signature {
    type Err = FromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Signature::from_base58(s)
    }
}

impl From<&Signature> for String {
    fn from(value: &Signature) -> String {
        value.to_string()
    }
}

impl ToString for Signature {
    fn to_string(&self) -> String {
        self.to_base58()
    }
}

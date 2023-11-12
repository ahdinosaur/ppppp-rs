use ed25519_dalek::{
    Signature as CryptoSignature, SignatureError as CryptoSignatureError,
    SigningKey as CryptoSigningKey, VerifyingKey as CryptoVerifyingKey,
};
use ppppp_base58 as base58;
use serde::{Deserialize, Serialize, Serializer};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;

pub use ed25519_dalek::SignatureError;

#[derive(Debug, ThisError)]
pub enum SignFromBase58Error {
    #[error("Failed to decode base58: {0}")]
    Decode(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
    #[error("Invalid verifying key: {0}")]
    VerifyingKey(#[source] CryptoSignatureError),
}

/// A secret key to sign messages
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct SigningKey(CryptoSigningKey);

impl SigningKey {
    pub const BYTE_SIZE: usize = 32_usize;

    pub fn from_bytes(bytes: &[u8; Self::BYTE_SIZE]) -> Self {
        Self(CryptoSigningKey::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, SignFromBase58Error> {
        let data = base58::decode(base58_str).map_err(SignFromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(SignFromBase58Error::Size { size: data.len() });
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
    type Error = SignFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SigningKey::from_base58(&value)
    }
}

impl FromStr for SigningKey {
    type Err = SignFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        SigningKey::from_base58(s)
    }
}

impl From<&SigningKey> for String {
    fn from(value: &SigningKey) -> String {
        value.to_string()
    }
}

impl Display for SigningKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

/// A public key to verify signatures
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct VerifyingKey(CryptoVerifyingKey);

impl VerifyingKey {
    pub const BYTE_SIZE: usize = 32_usize;

    pub fn from_bytes(bytes: &[u8; Self::BYTE_SIZE]) -> Result<Self, CryptoSignatureError> {
        Ok(Self(CryptoVerifyingKey::from_bytes(bytes)?))
    }

    pub fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_bytes()
    }

    pub fn as_bytes(&self) -> &[u8; Self::BYTE_SIZE] {
        self.0.as_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, SignFromBase58Error> {
        let data = base58::decode(base58_str).map_err(SignFromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(SignFromBase58Error::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes).map_err(SignFromBase58Error::VerifyingKey)?;
        Ok(key)
    }

    pub fn to_base58(&self) -> String {
        let data = self.0.to_bytes();
        base58::encode(&data)
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.0.verify_strict(message, &signature.0)
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
    type Error = SignFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        VerifyingKey::from_base58(&value)
    }
}

impl FromStr for VerifyingKey {
    type Err = SignFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        VerifyingKey::from_base58(s)
    }
}

impl From<&VerifyingKey> for String {
    fn from(value: &VerifyingKey) -> String {
        value.to_string()
    }
}

impl Display for VerifyingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

impl PartialEq for VerifyingKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

/// An Ed25519 signature
#[derive(Clone, Debug, Deserialize)]
#[serde(try_from = "String")]
pub struct Signature(CryptoSignature);

impl Signature {
    pub const BYTE_SIZE: usize = 64_usize;

    pub fn from_bytes(bytes: &[u8; Self::BYTE_SIZE]) -> Self {
        Self(CryptoSignature::from_bytes(bytes))
    }

    pub fn to_bytes(&self) -> [u8; Self::BYTE_SIZE] {
        self.0.to_bytes()
    }

    pub fn from_base58(base58_str: &str) -> Result<Self, SignFromBase58Error> {
        let data = base58::decode(base58_str).map_err(SignFromBase58Error::Decode)?;
        if data.len() != 64 {
            return Err(SignFromBase58Error::Size { size: data.len() });
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
    type Error = SignFromBase58Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Signature::from_base58(&value)
    }
}

impl FromStr for Signature {
    type Err = SignFromBase58Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Signature::from_base58(s)
    }
}

impl From<&Signature> for String {
    fn from(value: &Signature) -> String {
        value.to_string()
    }
}

impl Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_base58())
    }
}

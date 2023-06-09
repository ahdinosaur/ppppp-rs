use ed25519_dalek::{ed25519::Error as Ed25519Error, PublicKey};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;

use crate::base58;

#[derive(Debug, ThisError)]
pub enum IdError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
    #[error("Failed to convert to public key: {0}")]
    ToPubkey(#[source] Ed25519Error),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct AuthorId(Vec<u8>);

impl AuthorId {
    pub fn from_pubkey(pubkey: PublicKey) -> Self {
        let bytes = pubkey.as_bytes();
        let vec = bytes.to_vec();
        Self(vec)
    }

    pub fn from_str(id_str: &str) -> Result<Self, IdError> {
        let data = base58::decode(id_str)?;
        assert_eq!(data.len(), 32);
        Ok(Self(data))
    }

    pub fn to_pubkey(&self) -> Result<PublicKey, Ed25519Error> {
        let bytes = &self.0;
        PublicKey::from_bytes(bytes)
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        base58::encode(data)
    }
}

impl TryFrom<String> for AuthorId {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AuthorId::from_str(&value)
    }
}

impl FromStr for AuthorId {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AuthorId::from_str(&s)
    }
}

impl From<&AuthorId> for String {
    fn from(value: &AuthorId) -> String {
        value.to_string()
    }
}

impl Display for AuthorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl AsRef<[u8]> for AuthorId {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msg_id_roundtrip() -> Result<(), IdError> {
        let author_id_str = "4mjQ5aJu378cEu6TksRG3uXAiKFiwGjYQtWAjfVjDAJW";
        let author_id = AuthorId::from_str(author_id_str)?;
        assert_eq!(author_id_str, author_id.to_string());
        Ok(())
    }
}

use ed25519_dalek::{ed25519::Error as Ed25519Error, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, fmt::Display, str::FromStr};
use thiserror::Error as ThisError;

use ppppp_base58 as base58;

#[derive(Debug, ThisError)]
pub enum AccountKeyError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[from] base58::DecodeError),
    #[error("Failed to convert to public key: {0}")]
    ToPubkey(#[source] Ed25519Error),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct AccountKey([u8; 32]);

impl AccountKey {
    pub fn from_pubkey(pubkey: VerifyingKey) -> Self {
        let bytes = pubkey.to_bytes();
        Self(bytes)
    }

    pub fn from_str(id_str: &str) -> Result<Self, AccountKeyError> {
        let data = base58::decode(id_str)?;
        assert_eq!(data.len(), 32);
        Ok(Self(data.try_into().unwrap()))
    }

    pub fn to_pubkey(&self) -> Result<VerifyingKey, Ed25519Error> {
        let bytes = &self.0;
        VerifyingKey::from_bytes(bytes)
    }

    pub fn to_string(&self) -> String {
        let data = self.0.as_slice();
        base58::encode(data)
    }
}

impl TryFrom<String> for AccountKey {
    type Error = AccountKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        AccountKey::from_str(&value)
    }
}

impl FromStr for AccountKey {
    type Err = AccountKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccountKey::from_str(s)
    }
}

impl From<&AccountKey> for String {
    fn from(value: &AccountKey) -> String {
        value.to_string()
    }
}

impl Display for AccountKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl AsRef<[u8]> for AccountKey {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_key_roundtrip() -> Result<(), AccountKeyError> {
        let account_key_str = "4mjQ5aJu378cEu6TksRG3uXAiKFiwGjYQtWAjfVjDAJW";
        let account_key = AccountKey::from_str(account_key_str)?;
        assert_eq!(account_key_str, account_key.to_string());
        Ok(())
    }
}

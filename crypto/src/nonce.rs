use ppppp_bytes::{impl_as_bytes_outputs, impl_from_bytes_inputs, AsBytes, FromBytes};
use serde::{Deserialize, Serialize, Serializer};
use std::{fmt::Display, str::FromStr};

/// A 32 byte nonce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nonce([u8; 32]);

impl FromBytes<32> for Nonce {
    fn from_bytes(bytes: &[u8; 32]) -> Self {
        Self(*bytes)
    }
}

impl_from_bytes_inputs!(Nonce, 32_usize);

impl AsBytes<32> for Nonce {
    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl_as_bytes_outputs!(Nonce, 32_usize);

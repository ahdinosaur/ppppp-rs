use ppppp_bytes::{impl_as_bytes_outputs, impl_from_bytes_inputs, AsBytes, FromBytes};
use std::convert::Infallible;

/// A 32 byte nonce
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Nonce([u8; 32]);

impl FromBytes<32> for Nonce {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        Ok(Self(*bytes))
    }
}

impl AsBytes<32> for Nonce {
    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl_from_bytes_inputs!(Nonce, 32_usize);
impl_as_bytes_outputs!(Nonce, 32_usize);

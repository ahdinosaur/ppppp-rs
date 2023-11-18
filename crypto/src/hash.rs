use blake3::{Hash as CryptoHash, Hasher as CryptoHasher};
use ppppp_bytes::{impl_as_bytes_outputs, impl_from_bytes_inputs, AsBytes, FromBytes};
use std::{
    convert::Infallible,
    io::Write,
};

/// A cryptographic hash
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Hash(CryptoHash);

impl FromBytes<32> for Hash {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        Ok(Hash(CryptoHash::from_bytes(*bytes)))
    }
}

impl AsBytes<32> for Hash {
    fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl_from_bytes_inputs!(Hash, 32_usize);
impl_as_bytes_outputs!(Hash, 32_usize);

pub struct Hasher(CryptoHasher);

impl Hasher {
    pub fn new() -> Self {
        Self(CryptoHasher::new())
    }

    pub fn finalize(&self) -> Hash {
        Hash(self.0.finalize())
    }

    pub fn count(&self) -> u64 {
        self.0.count()
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
    use std::error::Error;

    use ppppp_bytes::DeserializeBytesError;

    use super::*;

    #[test]
    fn hash_hello_world() -> Result<(), Box<dyn Error>> {
        let input = "hello world";
        let mut hasher = Hasher::new();
        write!(hasher, "{}", input)?;
        let hash = hasher.finalize();
        assert_eq!(
            hash.to_string(),
            "FVPfbg9bK7mj7jnaSRXhuVcVakkXcjMPgSwxmauUofYf"
        );
        Ok(())
    }

    #[test]
    fn base58_roundtrip() -> Result<(), DeserializeBytesError> {
        let msg_hash_str = "FVPfbg9bK7mj7jnaSRXhuVcVakkXcjMPgSwxmauUofYf";
        let msg_hash = Hash::from_base58(msg_hash_str)?;
        assert_eq!(msg_hash_str, msg_hash.to_string());
        Ok(())
    }
}

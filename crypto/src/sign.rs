use ed25519_dalek::{
    Signature as CryptoSignature, Signer, SigningKey as CryptoSigningKey,
    VerifyingKey as CryptoVerifyingKey,
};
use getter_methods::GetterMethods;
use ppppp_bytes::{
    impl_as_bytes_outputs, impl_from_bytes_inputs, impl_to_bytes_outputs, AsBytes,
    DeserializeBytesError, FromBytes, ToBytes,
};
use serde::{Deserialize, Serialize, Serializer};
use std::{
    convert::{Infallible, TryFrom},
    fmt::Display,
    str::FromStr,
};

pub use ed25519_dalek::SignatureError;

pub type SignDeserializeBytesError = DeserializeBytesError<SignatureError>;

/// A secret key to sign messages
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SigningKey(CryptoSigningKey);

impl FromBytes<32> for SigningKey {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        Ok(SigningKey(CryptoSigningKey::from_bytes(bytes)))
    }
}

impl AsBytes<32> for SigningKey {
    fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl_from_bytes_inputs!(SigningKey, 32_usize);
impl_as_bytes_outputs!(SigningKey, 32_usize);

impl SigningKey {
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature(self.0.sign(message))
    }

    pub fn try_sign(&self, message: &[u8]) -> Result<Signature, SignatureError> {
        Ok(Signature(self.0.try_sign(message)?))
    }
}

/// A public key to verify signatures
#[derive(Clone, Debug, Eq)]
pub struct VerifyingKey(CryptoVerifyingKey);

impl FromBytes<32> for VerifyingKey {
    type Error = SignatureError;

    fn from_bytes(bytes: &[u8; 32]) -> Result<Self, Self::Error> {
        // TODO check if key is weak
        Ok(VerifyingKey(CryptoVerifyingKey::from_bytes(bytes)?))
    }
}

impl AsBytes<32> for VerifyingKey {
    fn as_bytes(&self) -> &[u8; 32] {
        self.0.as_bytes()
    }
}

impl_from_bytes_inputs!(VerifyingKey, 32_usize);
impl_as_bytes_outputs!(VerifyingKey, 32_usize);

impl VerifyingKey {
    // https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek#weak-key-forgery-and-verify_strict
    pub fn is_weak(&self) -> bool {
        self.0.is_weak()
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), SignatureError> {
        self.0.verify_strict(message, &signature.0)
    }
}

impl PartialEq for VerifyingKey {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

/// A private and public key pair to sign and verify signatures
#[derive(Clone, Debug, GetterMethods)]
pub struct SignKeypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

/// An Ed25519 signature
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Signature(CryptoSignature);

impl FromBytes<64> for Signature {
    type Error = Infallible;

    fn from_bytes(bytes: &[u8; 64]) -> Result<Self, Self::Error> {
        Ok(Signature(CryptoSignature::from_bytes(bytes)))
    }
}

impl ToBytes<64> for Signature {
    fn to_bytes(&self) -> [u8; 64] {
        self.0.to_bytes()
    }
}

impl_from_bytes_inputs!(Signature, 64_usize);
impl_to_bytes_outputs!(Signature, 64_usize);

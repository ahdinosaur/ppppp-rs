use getter_methods::GetterMethods;
use json_canon::to_writer as canon_json_to_writer;
use ppppp_crypto::{
    Hasher, Nonce, SignKeypair, Signature, SignatureError, SigningKey, VerifyingKey,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Error as JsonError, Map, Value};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    ops::Deref,
};
use typed_builder::TypedBuilder;

use crate::{AccountId, MsgDataHash, MsgDomain, MsgMetadataHash, Tangle};

#[derive(Debug, thiserror::Error)]
pub enum MsgError {
    #[error("failed to serialize to canonical json: {0}")]
    JsonCanon(#[source] JsonError),
    #[error("failed to verify signature: {0}")]
    Signature(#[source] SignatureError),
}

pub type MsgId = MsgMetadataHash;

#[derive(Clone, Debug, TypedBuilder)]
pub struct MsgCreateOpts {
    #[builder(setter(into))]
    pub data: MsgData,
    #[builder(setter(into))]
    pub domain: MsgDomain,
    #[builder(setter(into))]
    pub sign_keypair: SignKeypair,
    #[builder(setter(into))]
    pub account_id: AccountId,
    #[builder(setter(into))]
    pub account_tips: Option<Vec<MsgId>>,
    #[builder(setter(into))]
    pub tangles: HashMap<MsgId, Tangle>,
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
#[serde(deny_unknown_fields)]
pub struct Msg {
    #[serde(rename = "data")]
    data: MsgData,
    metadata: MsgMetadata,
    #[serde(rename = "pubkey")]
    verifying_key: VerifyingKey,
    #[serde(rename = "sig")]
    signature: MsgSignature,
}

impl Msg {
    pub fn create(opts: MsgCreateOpts) -> Result<Self, MsgError> {
        let MsgCreateOpts {
            data,
            domain,
            sign_keypair,
            account_id,
            mut account_tips,
            tangles: opt_tangles,
        } = opts;
        let (data_hash, data_size) = data.to_hash();
        if let Some(ref mut account_tips) = account_tips {
            account_tips.sort();
        }

        let mut tangles = HashMap::new();
        for (root_msg_id, tangle) in opt_tangles.iter() {
            let depth = tangle.get_max_depth() + 1;
            let lipmaa_set = tangle.get_lipmaa_set(depth);
            let mut prev_msg_ids: HashSet<_> =
                lipmaa_set.union(&tangle.get_tips()).cloned().collect();
            tangles.insert(
                root_msg_id.clone(),
                MsgTangle {
                    prev_msg_ids,
                    depth,
                },
            );
        }

        let metadata = MsgMetadata {
            data_hash: Some(data_hash),
            data_size,
            account_id,
            account_tips,
            tangles,
            domain,
            version: 3,
        };

        let signing_key = sign_keypair.signing_key();
        let verifying_key = sign_keypair.verifying_key();

        let signature = metadata.to_signature(signing_key)?;

        Ok(Msg {
            data,
            metadata,
            verifying_key: verifying_key.clone(),
            signature,
        })
    }

    pub fn create_moot(
        account_id: AccountId,
        domain: MsgDomain,
        sign_keypair: SignKeypair,
    ) -> Result<Msg, MsgError> {
        let metadata = MsgMetadata {
            data_hash: None,
            data_size: 0,
            account_id,
            account_tips: None,
            tangles: HashMap::new(),
            domain,
            version: 3,
        };

        let signing_key = sign_keypair.signing_key();
        let verifying_key = sign_keypair.verifying_key();

        let signature = metadata.to_signature(signing_key)?;

        Ok(Msg {
            data: MsgData(Value::Null),
            metadata,
            verifying_key: verifying_key.clone(),
            signature,
        })
    }

    pub fn create_account<CreateNonce>(
        sign_keypair: SignKeypair,
        domain: MsgDomain,
        create_nonce: CreateNonce,
    ) where
        CreateNonce: Fn() -> Nonce,
    {
    }

    pub fn id(&self) -> Result<MsgId, MsgError> {
        let hash = self.metadata.to_hash()?;
        Ok(hash)
    }

    pub fn is_moot(&self, account_id: Option<AccountId>, find_domain: Option<MsgDomain>) -> bool {
        let metadata = self.metadata();
        !(metadata.data_hash().is_some()
            || metadata.data_size() != 0
            || (account_id.is_some() && metadata.account_id() == &account_id.unwrap())
            || metadata.account_tips().is_some()
            || !metadata.tangles().is_empty()
            || (find_domain.is_some() && metadata.domain() == &find_domain.unwrap()))
    }

    pub fn verify_signature(&self) -> Result<(), MsgError> {
        self.metadata()
            .verify_signature(self.verifying_key(), self.signature())?;

        Ok(())
    }

    pub fn get_moot_id(account_id: AccountId, domain: MsgDomain) -> Result<MsgId, MsgError> {
        let moot = MsgMetadata::get_moot(account_id, domain);
        let hash = moot.to_hash()?;
        Ok(hash)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "Value")]
pub struct MsgData(Value);

impl MsgData {
    pub fn to_hash(&self) -> (MsgDataHash, u64) {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self.0).unwrap();
        let hash = hasher.finalize();
        let size = hasher.count();

        (MsgDataHash::from_hash(hash), size)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    pub fn is_string(&self) -> bool {
        self.0.is_string()
    }

    pub fn as_str(&self) -> Option<&str> {
        self.0.as_str()
    }

    pub fn is_object(&self) -> bool {
        self.0.is_object()
    }

    pub fn as_object(&self) -> Option<&Map<String, Value>> {
        self.0.as_object()
    }
}

#[derive(Copy, Clone, Debug, thiserror::Error)]
#[error("invalid data, must be JSON object, string, or null")]
pub struct MsgDataFromJsonValue;

impl TryFrom<Value> for MsgData {
    type Error = MsgDataFromJsonValue;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if value.is_object() || value.is_string() || value.is_null() {
            Ok(Self(value))
        } else {
            Err(MsgDataFromJsonValue)
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
#[serde(deny_unknown_fields)]
pub struct MsgMetadata {
    #[serde(rename = "account")]
    account_id: AccountId,
    #[serde(rename = "accountTips")]
    account_tips: Option<Vec<MsgId>>,
    #[serde(rename = "dataHash")]
    data_hash: Option<MsgDataHash>,
    #[serde(rename = "dataSize")]
    data_size: u64,
    domain: MsgDomain,
    tangles: MsgTangles,
    #[serde(rename = "v")]
    version: u8,
}

impl MsgMetadata {
    pub fn to_hash(&self) -> Result<MsgMetadataHash, MsgError> {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self).map_err(MsgError::JsonCanon)?;
        let hash = hasher.finalize();
        Ok(MsgMetadataHash::from_hash(hash))
    }

    pub fn to_signature(&self, signing_key: &SigningKey) -> Result<MsgSignature, MsgError> {
        let signable = self.to_signable()?;
        let signature = signing_key
            .try_sign(&signable)
            .map_err(MsgError::Signature)?;
        Ok(MsgSignature(signature))
    }

    pub fn verify_signature(
        &self,
        verifying_key: &VerifyingKey,
        signature: &MsgSignature,
    ) -> Result<(), MsgError> {
        let signable = self.to_signable()?;
        verifying_key
            .verify(&signable, signature)
            .map_err(MsgError::Signature)?;
        Ok(())
    }

    pub fn to_signable(&self) -> Result<Vec<u8>, MsgError> {
        static TAG: &[u8] = ":msg-v3:".as_bytes();
        let json = json_canon::to_vec(self).map_err(MsgError::JsonCanon)?;
        Ok([TAG, json.as_slice()].concat())
    }

    pub fn get_moot(account_id: AccountId, domain: MsgDomain) -> Self {
        Self {
            account_id,
            domain,
            account_tips: None,
            data_hash: None,
            data_size: 0,
            tangles: HashMap::new(),
            version: 3,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
#[serde(deny_unknown_fields)]
pub struct MsgTangle {
    #[serde(rename = "prev")]
    prev_msg_ids: HashSet<MsgId>,
    depth: u64,
}

pub type MsgTangles = HashMap<MsgId, MsgTangle>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgSignature(Signature);

impl Deref for MsgSignature {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_hello_world() {
        let value = json!({
            "text": "hello world!"
        });
        let data: MsgData = value.try_into().unwrap();
        let (hash, size): (MsgDataHash, _) = data.to_hash();
        assert_eq!(hash.to_string(), "Cz1jtXr2oBrhk8czWiz6kH");
        assert_eq!(size, 23);
    }
}

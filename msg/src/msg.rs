use getter_methods::GetterMethods;
use json_canon::to_writer as canon_json_to_writer;
use ppppp_crypto::{Hasher, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Map, Value};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    io::{self, Write},
    ops::Deref,
};

use crate::{MsgDataHash, MsgDomain, MsgMetadataHash};

#[derive(Debug, thiserror::Error)]
pub enum MsgError {
    #[error("failed to serialize to canonical json: {0}")]
    JsonCanon(#[source] JsonError),
    #[error("io error: {0}")]
    Io(#[source] io::Error),
}

pub type MsgId = MsgMetadataHash;

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

    pub fn to_signable(&self) -> Result<Vec<u8>, MsgError> {
        let mut signable = Vec::new();

        static TAG: &[u8] = ":msg-v3:".as_bytes();
        signable.write_all(TAG).map_err(MsgError::Io)?;

        json_canon::to_writer(&mut signable, self).map_err(MsgError::JsonCanon)?;

        Ok(signable)
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

    pub fn get_moot_id(account_id: AccountId, domain: MsgDomain) -> Result<MsgId, MsgError> {
        let moot = Self::get_moot(account_id, domain);
        let hash = moot.to_hash()?;
        Ok(hash)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum AccountId {
    Tangle(MsgId),
    #[serde(rename = "self")]
    SelfIdentity,
    #[serde(rename = "any")]
    Any,
}

impl Display for AccountId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountId::Tangle(msg_id) => write!(f, "AccountId::Tangle({}", msg_id),
            AccountId::SelfIdentity => write!(f, "AccountId::SelfIdentity"),
            AccountId::Any => write!(f, "AccountId::Any"),
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

use getter_methods::GetterMethods;
use json_canon::to_writer as canon_json_to_writer;
use ppppp_crypto::{Hash, Hasher, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::{data::Data, domain::Domain, AccountId, MsgDataHash, MsgMetadataHash};

pub type MsgId = MsgMetadataHash;

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
#[serde(deny_unknown_fields)]
pub struct Msg {
    #[serde(rename = "data")]
    data: Data,
    metadata: MsgMetadata,
    #[serde(rename = "pubkey")]
    verifying_key: VerifyingKey,
    #[serde(rename = "sig")]
    signature: MsgSignature,
}

impl Msg {
    pub fn id(&self) -> MsgId {
        MsgId::from_hash(self.metadata.to_hash())
    }

    pub fn is_moot(&self, account_id: Option<AccountId>, find_domain: Option<Domain>) -> bool {
        let metadata = self.metadata();
        if metadata.data_hash().is_some() {
            false
        } else if metadata.data_size() != 0 {
            false
        } else if account_id.is_some() && metadata.account_id() == &account_id.unwrap() {
            false
        } else if metadata.account_tips().is_some() {
            false
        } else if !metadata.tangles().is_empty() {
            false
        } else if find_domain.is_some() && metadata.domain() == &find_domain.unwrap() {
            false
        } else {
            true
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
    domain: Domain,
    tangles: MsgTangles,
    #[serde(rename = "v")]
    version: u8,
}

impl MsgMetadata {
    pub fn to_hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self).unwrap();
        let hash = hasher.finalize();
        hash
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
#[serde(deny_unknown_fields)]
pub struct MsgTangle {
    #[serde(rename = "prev")]
    prev_msg_hashs: HashSet<MsgId>,
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

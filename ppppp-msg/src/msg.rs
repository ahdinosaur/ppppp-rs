use blake3::{Hash, Hasher};
use getter_methods::GetterMethods;
use json_canon::to_writer as canon_json_to_writer;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    account_id::AccountId, data::Data, data_hash::DataHash, domain::Domain, msg_hash::MsgHash,
    signature::Signature,
};

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
pub struct Msg {
    #[serde(rename = "data")]
    data: Data,
    metadata: MsgMetadata,
    #[serde(rename = "sig")]
    signature: Signature,
}

impl Msg {
    pub fn id(&self) -> MsgHash {
        MsgHash::from_hash(self.metadata.to_hash())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
pub struct MsgMetadata {
    account: AccountId,
    #[serde(rename = "accountTips")]
    account_tips: Option<Vec<MsgHash>>,
    #[serde(rename = "dataHash")]
    data_hash: Option<DataHash>,
    #[serde(rename = "dataSize")]
    data_size: u64,
    domain: Domain,
    tangles: MsgTangles,
    #[serde(rename = "type")]
    data_type: String,
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
pub struct MsgTangle {
    #[serde(rename = "prev")]
    prev_msg_hashs: BTreeSet<MsgHash>,
    depth: u64,
}

pub type MsgTangles = BTreeMap<MsgHash, MsgTangle>;

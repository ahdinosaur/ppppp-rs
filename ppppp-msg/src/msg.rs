use getter_methods::GetterMethods;
use json_canon::to_writer as canon_json_to_writer;
use ppppp_crypto::{Hash, Hasher, Signature};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    ops::Deref,
};

use crate::{account_id::AccountId, data::Data, domain::Domain, MsgDataHash, MsgMetadataHash};

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
pub struct Msg {
    #[serde(rename = "data")]
    data: Data,
    metadata: MsgMetadata,
    #[serde(rename = "sig")]
    signature: MsgSignature,
}

impl Msg {
    pub fn id(&self) -> MsgMetadataHash {
        MsgMetadataHash::from_hash(self.metadata.to_hash())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, GetterMethods)]
pub struct MsgMetadata {
    account: AccountId,
    #[serde(rename = "accountTips")]
    account_tips: Option<Vec<MsgMetadataHash>>,
    #[serde(rename = "dataHash")]
    data_hash: Option<MsgDataHash>,
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
    prev_msg_hashs: HashSet<MsgMetadataHash>,
    depth: u64,
}

pub type MsgTangles = HashMap<MsgMetadataHash, MsgTangle>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgSignature(Signature);

impl Deref for MsgSignature {
    type Target = Signature;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

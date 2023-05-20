use blake3::{Hash, Hasher};
use json_canon::to_writer as canon_json_to_writer;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    content::Content, content_hash::ContentHash, key_id::KeyId, msg_id::MsgId, signature::Signature,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Msg {
    content: Content,
    metadata: MsgMetadata,
    #[serde(rename = "sig")]
    signature: Signature,
}

impl Msg {
    pub fn id(&self) -> MsgId {
        MsgId::from_hash(self.metadata.to_hash())
    }

    pub fn content(&self) -> &Content {
        &self.content
    }

    pub fn metadata(&self) -> &MsgMetadata {
        &self.metadata
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgMetadata {
    #[serde(rename = "hash")]
    content_hash: Option<ContentHash>,
    #[serde(rename = "size")]
    content_size: u64,
    tangles: MsgTangles,
    #[serde(rename = "type")]
    content_type: String,
    #[serde(rename = "v")]
    version: u8,
    #[serde(rename = "who")]
    key_id: KeyId,
}

impl MsgMetadata {
    pub fn content_hash(&self) -> &Option<ContentHash> {
        &self.content_hash
    }

    pub fn content_size(&self) -> &u64 {
        &self.content_size
    }

    pub fn tangles(&self) -> &MsgTangles {
        &self.tangles
    }

    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn version(&self) -> &u8 {
        &self.version
    }

    pub fn key_id(&self) -> &KeyId {
        &self.key_id
    }

    pub fn to_hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self).unwrap();
        let hash = hasher.finalize();
        hash
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgTangle {
    #[serde(rename = "prev")]
    prev_msg_ids: BTreeSet<MsgId>,
    depth: u64,
}

impl MsgTangle {
    pub fn prev_msg_ids(&self) -> &BTreeSet<MsgId> {
        &self.prev_msg_ids
    }

    pub fn depth(&self) -> &u64 {
        &self.depth
    }
}

pub type MsgTangles = BTreeMap<MsgId, MsgTangle>;

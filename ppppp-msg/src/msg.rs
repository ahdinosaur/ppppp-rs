use blake3::{Hash, Hasher};
use json_canon::to_writer as canon_json_to_writer;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

use crate::{
    content::{Content, ContentHash},
    id::MsgId,
    signature::Signature,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Msg {
    content: Content,
    metadata: MsgMetadata,
}

impl Msg {
    pub fn to_id(&self) -> MsgId {
        MsgId::from_hash(self.metadata.to_hash())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MsgMetadata {
    hash: ContentHash,
    size: u32,
    tangles: BTreeMap<MsgId, BTreeSet<MsgId>>,
    type_: String,
    v: u8,
    sig: Signature,
}

impl MsgMetadata {
    pub fn to_hash(&self) -> Hash {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self).unwrap();
        let hash = hasher.finalize();
        hash
    }
}

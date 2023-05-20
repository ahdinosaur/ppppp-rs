use std::collections::{BTreeMap, BTreeSet};

use crate::{
    content::{Content, ContentHash},
    id::MsgId,
};

pub struct Msg {
    data: Content,
    metadata: MsgMetadata,
}

pub struct MsgMetadata {
    hash: ContentHash,
    size: u32,
    tangles: BTreeMap<MsgId, BTreeSet<MsgId>>,
    type_: String,
    v: u8,
}

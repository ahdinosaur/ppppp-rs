use serde::{Deserialize, Serialize};

use crate::msg_hash::MsgHash;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum AccountId {
    Tangle(MsgHash),
    #[serde(rename = "self")]
    SelfIdentity,
    #[serde(rename = "any")]
    Any,
}

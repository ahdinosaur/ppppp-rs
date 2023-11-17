mod account;
mod domain;
mod hash;
mod msg;
mod tangle;
mod validate;

pub use ppppp_bytes::DeserializeBytesError;

pub use crate::account::AccountId;
pub use crate::domain::MsgDomain;
pub use crate::hash::{MsgDataHash, MsgMetadataHash};
pub use crate::msg::{Msg, MsgData, MsgId, MsgMetadata, MsgSignature, MsgTangle, MsgTangles};
pub use crate::tangle::{Tangle, TangleType};
pub use crate::validate::{validate, ValidateError};

pub struct MootDetails {
    pub account_id: AccountId,
    pub domain: MsgDomain,
    pub id: MsgId,
}

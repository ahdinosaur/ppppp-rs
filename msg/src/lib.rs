mod hash;
mod msg;
mod tangle;
mod validate;

pub use crate::hash::{HashFromBase58Error, MsgDataHash, MsgMetadataHash};
pub use crate::msg::{
    AccountId, Msg, MsgData, MsgDomain, MsgId, MsgMetadata, MsgSignature, MsgTangle, MsgTangles,
};
pub use crate::tangle::Tangle;
pub use crate::validate::{validate, ValidateError};

pub struct MootDetails {
    pub account_id: AccountId,
    pub domain: MsgDomain,
    pub id: MsgId,
}

// https://github.com/staltz/ppppp-db/blob/master/protospec.md#account-tangle-msgs

use std::fmt::Display;

use ppppp_crypto::{Nonce, Signature};
use serde::{Deserialize, Deserializer, Serialize};

use crate::MsgId;

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AccountMsgData {
    #[serde(rename = "add")]
    Add { add: AccountAdd },
    #[serde(rename = "del")]
    Del { del: AccountDel },
}

/// base58 encoded signature of the string `:account-add:<ID>` where `<ID>` is the account's ID
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountConsent(Signature);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountAdd {
    key: AccountKey,
    // nonce required only on the account tangle's root
    nonce: Option<Nonce>,
    // required only on non-root msgs
    consent: Option<AccountConsent>,
    // list of powers granted to this key, defaults to []
    #[serde(rename = "accountPowers", default)]
    account_powers: Vec<AccountPower>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AccountDel {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AccountKey {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum AccountPower {
    #[serde(rename = "add")]
    Add,
    #[serde(rename = "del")]
    Del,
    #[serde(rename = "internal-encryption")]
    InternalEncryption,
    #[serde(rename = "external-encryption")]
    ExternalEncryption,
}

// https://github.com/staltz/ppppp-db/blob/master/protospec.md#account-tangle-msgs

use monostate::MustBe;
use ppppp_crypto::{Nonce, Signature, SigningKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Display;

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

/// "add" means this shs peer can validly add more keys to the account tangle
/// "del" means this shs peer can validly revoke keys from the account tangle
/// "internal-encryption" means this shs peer should get access to symmetric key
/// "external-encryption" means this shs peer should get access to asymmetric key
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
pub struct AccountDel {
    key: AccountKey,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "purpose")]
pub enum AccountKey {
    // secret-handshake and digital signatures
    #[serde(rename = "shs-and-external-signature")]
    ShsAndExternalSignature {
        algorithm: MustBe!("ed25519"),
        bytes: SigningKey,
    },
    // asymmetric encryption
    #[serde(rename = "external-encryption")]
    ExternalEncryption {
        algorithm: MustBe!("x25519-xsalsa20-poly1305"),
        // TODO bytes: BoxingKey
    },
    // digital signature of internal messages
    #[serde(rename = "internal-signature")]
    InternalSignature {
        algorithm: MustBe!("ed25519"),
        bytes: SigningKey,
    },
}

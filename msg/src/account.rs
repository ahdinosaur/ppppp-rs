// https://github.com/staltz/ppppp-db/blob/master/protospec.md#account-tangle-msgs

use monostate::MustBe;
use ppppp_crypto::{Nonce, Signature, VerifyingKey};
use serde::{Deserialize, Serialize};
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
#[serde(tag = "action", rename = "kebab-case")]
pub enum AccountMsgData {
    Add {
        key: AccountKey,
        // nonce required only on the account tangle's root
        nonce: Option<Nonce>,
        // required only on non-root msgs
        consent: Option<AccountConsent>,
        // list of powers granted to this key, defaults to []
        #[serde(rename = "accountPowers", default)]
        account_powers: Vec<AccountPower>,
    },
    Del {
        key: AccountKey,
    },
}

/// base58 encoded signature of the string `:account-add:<ID>` where `<ID>` is the account's ID
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountConsent(Signature);

/// "add" means this shs peer can validly add more keys to the account tangle
/// "del" means this shs peer can validly revoke keys from the account tangle
/// "internal-encryption" means this shs peer should get access to symmetric key
/// "external-encryption" means this shs peer should get access to asymmetric key
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename = "kebab-case")]
pub enum AccountPower {
    Add,
    Del,
    InternalEncryption,
    ExternalEncryption,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "purpose", rename = "kebab-case")]
pub enum AccountKey {
    // secret-handshake and digital signatures
    ShsAndExternalSignature {
        algorithm: MustBe!("ed25519"),
        bytes: VerifyingKey,
    },
    // asymmetric encryption
    ExternalEncryption {
        algorithm: MustBe!("x25519-xsalsa20-poly1305"),
        // TODO bytes: BoxingKey
    },
    // digital signature of internal messages
    InternalSignature {
        algorithm: MustBe!("ed25519"),
        bytes: VerifyingKey,
    },
}

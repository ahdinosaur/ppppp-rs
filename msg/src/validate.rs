use std::io;

use ppppp_crypto::{SignatureError, VerifyingKey};
use serde_json::Error as JsonError;

use crate::{
    msg::MsgError, tangle::TangleMissingRootMessageError, AccountId, MootDetails, Msg, MsgData,
    MsgDomain, MsgId, Tangle, TangleType,
};

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
    #[error("invalid version: {version}")]
    Version { version: u8 },
    #[error("io error: {0}")]
    Io(#[source] io::Error),
    #[error("failed to serialize to canonical json: {0}")]
    JsonCanon(#[source] JsonError),
    #[error("invalid signature: {0}")]
    Signature(#[source] SignatureError),
    #[error("tangle missing root message: {root_msg_id}")]
    TangleMissingRootMessage { root_msg_id: MsgId },
    #[error("tangle missing root message id: {root_msg_id}")]
    MsgTanglesMissingTangleRootMsgId { root_msg_id: MsgId },
    #[error("domain {msg_domain} should have been feed domain {feed_domain}")]
    MsgDomainMustBeFeedDomain {
        msg_domain: MsgDomain,
        feed_domain: MsgDomain,
    },
    #[error("account {msg_account_id} should have been feed domain {feed_account_id}")]
    MsgAccountMustBeFeedAccount {
        msg_account_id: AccountId,
        feed_account_id: AccountId,
    },
    #[error("account {account_id} cannot be \"self\" in a feed tangle")]
    AccountCannotBeSelfInAFeedTangle { account_id: AccountId },
    #[error("account {account_id} must be \"self\" in a feed tangle")]
    AccountMustBeSelfInAFeedTangle { account_id: AccountId },
    #[error("verifying key {verifying_key} should have been one of {verifying_keys:?} from the account {account_id}")]
    VerifyingKeyMustBeFromAccount {
        verifying_key: VerifyingKey,
        verifying_keys: Vec<VerifyingKey>,
        account_id: AccountId,
    },
    #[error("accountTips {account_tips:?} must be none in an account tangle")]
    AccountTipsMustBeNullInAnAccountTangle { account_tips: Vec<MsgId> },
    #[error("depth of prev {prev_msg_id} is not lower")]
    TanglePrevDepthNotLower { prev_msg_id: MsgId },
    #[error("all prev are locally unknown")]
    AllPrevUnknown,
    #[error("depth must be the largest prev depth plus one")]
    DepthMustBeMaxPlusOne,
    #[error("if tangle empty, msg id must match tangle root msg id")]
    IfEmptyTangleThenMsgIdMustMatchTangleRootMsgId,
    #[error("tangle root must not have self tangles")]
    TangleRootMustNotHaveSelfTangles,
    #[error("data size does not match metadata.size")]
    DataSizeDoesNotMatchMetadata,
    #[error("data hash does not match metadata.hash")]
    DataHashDoesNotMatchMetadata,
    #[error("data must be null, string, or object")]
    DataMustBeNullOrStringOrObject { msg_data: MsgData },
}

pub fn validate(
    msg: &Msg,
    msg_id: &MsgId,
    tangle: &Tangle,
    verifying_keys: &[VerifyingKey],
    tangle_root_msg_id: &MsgId,
) -> Result<(), ValidateError> {
    validate_version(msg)?;
    validate_data(msg)?;

    let tangle_type =
        tangle
            .get_type()
            .map_err(|TangleMissingRootMessageError { root_msg_id }| {
                ValidateError::TangleMissingRootMessage { root_msg_id }
            })?;
    if tangle_type == TangleType::Feed && msg.is_moot(None, None) {
        // nothing else to check
        return Ok(());
    }

    validate_data_size_hash(msg)?;
    validate_verifying_key_and_account(msg, tangle, verifying_keys)?;
    if msg_id == tangle_root_msg_id {
        validate_tangle_root(msg, msg_id, tangle_root_msg_id)?;
    } else {
        validate_tangle(msg, tangle, tangle_root_msg_id)?;
    }

    validate_signature(msg)?;

    Ok(())
}

pub fn validate_version(msg: &Msg) -> Result<(), ValidateError> {
    let version = msg.metadata().version();
    if version != 3 {
        Err(ValidateError::Version { version })
    } else {
        Ok(())
    }
}

fn validate_data(msg: &Msg) -> Result<(), ValidateError> {
    let data = msg.data();
    if data.is_null() || data.is_string() || data.is_object() {
        Ok(())
    } else {
        Err(ValidateError::DataMustBeNullOrStringOrObject {
            msg_data: data.clone(),
        })
    }
}

fn validate_data_size_hash(msg: &Msg) -> Result<(), ValidateError> {
    let data = msg.data();
    let metadata = msg.metadata();

    if data.is_null() {
        return Ok(());
    }

    let (data_hash, data_size) = data.to_hash();
    if &Some(data_hash) != metadata.data_hash() {
        Err(ValidateError::DataHashDoesNotMatchMetadata)
    } else if data_size != metadata.data_size() {
        Err(ValidateError::DataSizeDoesNotMatchMetadata)
    } else {
        Ok(())
    }
}

fn validate_verifying_key_and_account(
    msg: &Msg,
    tangle: &Tangle,
    verifying_keys: &[VerifyingKey],
) -> Result<(), ValidateError> {
    let tangle_type =
        tangle
            .get_type()
            .map_err(|TangleMissingRootMessageError { root_msg_id }| {
                ValidateError::TangleMissingRootMessage { root_msg_id }
            })?;
    let account_id = msg.metadata().account_id();
    let verifying_key = msg.verifying_key();

    if tangle_type == TangleType::Feed || tangle_type == TangleType::Weave {
        if account_id == &AccountId::SelfIdentity {
            return Err(ValidateError::AccountCannotBeSelfInAFeedTangle {
                account_id: account_id.clone(),
            });
        }
        if account_id != &AccountId::Any && verifying_keys.iter().any(|k| k == verifying_key) {
            return Err(ValidateError::VerifyingKeyMustBeFromAccount {
                verifying_key: verifying_key.clone(),
                verifying_keys: verifying_keys.iter().cloned().collect(),
                account_id: account_id.clone(),
            });
        }
    } else if tangle_type == TangleType::Account {
        if account_id != &AccountId::SelfIdentity {
            return Err(ValidateError::AccountMustBeSelfInAFeedTangle {
                account_id: account_id.clone(),
            });
        }
        if let Some(account_tips) = msg.metadata().account_tips() {
            return Err(ValidateError::AccountTipsMustBeNullInAnAccountTangle {
                account_tips: account_tips.clone(),
            });
        }
    }
    Ok(())
}

pub fn validate_signature(msg: &Msg) -> Result<(), ValidateError> {
    let signable = msg.metadata().to_signable().map_err(|err| match err {
        MsgError::JsonCanon(json_err) => ValidateError::JsonCanon(json_err),
        MsgError::Io(io_err) => ValidateError::Io(io_err),
    })?;
    let verifying_key = msg.verifying_key();
    let signature = msg.signature();

    verifying_key
        .verify(&signable, signature)
        .map_err(ValidateError::Signature)?;

    Ok(())
}

pub fn validate_tangle(
    msg: &Msg,
    tangle: &Tangle,
    tangle_root_msg_id: &MsgId,
) -> Result<(), ValidateError> {
    let metadata = msg.metadata();

    let msg_tangles = metadata.tangles();
    let msg_tangle = msg_tangles.get(tangle_root_msg_id).ok_or(
        ValidateError::MsgTanglesMissingTangleRootMsgId {
            root_msg_id: tangle_root_msg_id.clone(),
        },
    )?;

    let depth = msg_tangle.depth();
    let prev_msg_ids = msg_tangle.prev_msg_ids();

    if tangle.is_feed() {
        let MootDetails {
            account_id: feed_account_id,
            domain: feed_domain,
            ..
        } = tangle.get_moot_details().unwrap();
        let msg_domain = msg.metadata().domain();
        if &feed_domain != msg_domain {
            return Err(ValidateError::MsgDomainMustBeFeedDomain {
                msg_domain: msg_domain.clone(),
                feed_domain,
            });
        }
        let msg_account_id = msg.metadata().account_id();
        if &feed_account_id != msg_account_id {
            return Err(ValidateError::MsgAccountMustBeFeedAccount {
                msg_account_id: msg_account_id.clone(),
                feed_account_id,
            });
        }
    }

    let mut min_diff = u64::MAX;
    let mut count_prev_unknown = 0_u64;

    for prev_msg_id in prev_msg_ids {
        if !tangle.has(prev_msg_id) {
            count_prev_unknown += 1;
            continue;
        }

        let prev_depth = tangle.get_depth(prev_msg_id).unwrap();

        let diff = depth - prev_depth;
        if diff <= 0 {
            return Err(ValidateError::TanglePrevDepthNotLower {
                prev_msg_id: prev_msg_id.clone(),
            });
        }
        if diff < min_diff {
            min_diff = diff
        }
    }

    if count_prev_unknown == prev_msg_ids.len() as u64 {
        return Err(ValidateError::AllPrevUnknown);
    }

    if count_prev_unknown == 0 && min_diff != 1 {
        return Err(ValidateError::DepthMustBeMaxPlusOne);
    }

    Ok(())
}

fn validate_tangle_root(
    msg: &Msg,
    msg_id: &MsgId,
    tangle_root_msg_id: &MsgId,
) -> Result<(), ValidateError> {
    if msg_id == tangle_root_msg_id {
        Err(ValidateError::IfEmptyTangleThenMsgIdMustMatchTangleRootMsgId)
    } else if msg.metadata().tangles().contains_key(tangle_root_msg_id) {
        Err(ValidateError::TangleRootMustNotHaveSelfTangles)
    } else {
        Ok(())
    }
}

use ppppp_crypto::{SignatureError, VerifyingKey};
use serde_json::Error as JsonError;

use crate::{Msg, MsgData, MsgId, Tangle};

#[derive(Debug, thiserror::Error)]
pub enum ValidateError {
    #[error("invalid version: {version}")]
    Version { version: u8 },
    #[error("failed to serialize to canonical json: {0}")]
    JsonCanon(#[source] JsonError),
    #[error("invalid signature: {0}")]
    Signature(#[source] SignatureError),
    #[error("tangle missing root message id: {root_msg_id}")]
    MsgTanglesMissingTangleRootMsgId { root_msg_id: MsgId },
    #[error("msg data type doesn't match feed type: {data_type}")]
    MsgTypeDoesNotMatchFeedType { data_type: String },
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
    verifying_keys: Vec<VerifyingKey>,
    tangle_root_msg_id: &MsgId,
) -> Result<(), ValidateError> {
    validate_version(msg)?;
    validate_data(msg)?;

    if tangle.type() == TangleType::

    if tangle.size() == 0 {
        validate_tangle_root(msg, msg_id, tangle_root_msg_id)?;
    } else {
        validate_tangle(msg, tangle, tangle_root_msg_id)?;
    }

    validate_data(msg)?;
    validate_signature(msg)?;

    Ok(())
}

pub fn validate_version(msg: &Msg) -> Result<(), ValidateError> {
    let version = *msg.metadata().version();
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
    let metadata = msg.metadata();

    if data.is_null() {
        return Ok(());
    }

    let (data_hash, data_size) = data.to_hash();
    if &Some(data_hash) != metadata.data_hash() {
        Err(ValidateError::DataHashDoesNotMatchMetadata)
    } else if &data_size != metadata.data_size() {
        Err(ValidateError::DataSizeDoesNotMatchMetadata)
    } else {
        Ok(())
    }
}

pub fn validate_signature(msg: &Msg) -> Result<(), ValidateError> {
    let metadata = msg.metadata();
    let signature = msg.signature();

    let author_id = metadata.author_id();
    let pubkey = author_id.to_pubkey().map_err(ValidateError::Pubkey)?;

    let signable = json_canon::to_vec(metadata).map_err(ValidateError::JsonCanon)?;

    pubkey
        .verify_strict(&signable, &signature.to_signature())
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
        let (feed_author_id, feed_data_type) = tangle.get_feed().unwrap();
        let data_type = metadata.data_type();
        if data_type != feed_data_type {
            return Err(ValidateError::MsgTypeDoesNotMatchFeedType {
                data_type: data_type.to_owned(),
            });
        }
        let author_id = metadata.author_id();
        if author_id != &feed_author_id {
            return Err(ValidateError::MsgAuthorIdDoesNotMatchFeedAuthorId {
                author_id: author_id.clone(),
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

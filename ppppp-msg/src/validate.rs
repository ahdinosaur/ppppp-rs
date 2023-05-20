use ed25519_dalek::ed25519::Error as Ed25519Error;
use serde_json::Error as JsonError;

use crate::{key_id::KeyId, msg::Msg, msg_id::MsgId, tangle::Tangle};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid version: {version}")]
    Version { version: u8 },
    #[error("invalid pubkey: {0}")]
    Pubkey(#[source] Ed25519Error),
    #[error("failed to serialize to canonical json: {0}")]
    JsonCanon(#[source] JsonError),
    #[error("invalid signature: {0}")]
    Signature(#[source] Ed25519Error),
    #[error("tangle missing root message id: {root_msg_id}")]
    MsgTanglesMissingTangleRootMsgId { root_msg_id: MsgId },
    #[error("msg content type doesn't match feed type: {content_type}")]
    MsgTypeDoesNotMatchFeedType { content_type: String },
    #[error("msg key id doesn't match feed key id: {key_id}")]
    MsgKeyIdDoesNotMatchFeedKeyId { key_id: KeyId },
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
    #[error("content size does not match metadata.size")]
    ContentSizeDoesNotMatchMetadata,
    #[error("content hash does not match metadata.hash")]
    ContentHashDoesNotMatchMetadata,
}

pub fn validate(
    msg: &Msg,
    msg_id: &MsgId,
    tangle: &Tangle,
    tangle_root_msg_id: &MsgId,
) -> Result<(), Error> {
    validate_version(msg)?;

    if tangle.size() == 0 {
        validate_tangle_root(msg, msg_id, tangle_root_msg_id)?;
    } else {
        validate_tangle(msg, tangle, tangle_root_msg_id)?;
    }

    validate_content(msg)?;
    validate_signature(msg)?;

    Ok(())
}

pub fn validate_version(msg: &Msg) -> Result<(), Error> {
    let version = *msg.metadata().version();
    if version != 1 {
        Err(Error::Version { version })
    } else {
        Ok(())
    }
}

pub fn validate_signature(msg: &Msg) -> Result<(), Error> {
    let metadata = msg.metadata();
    let signature = msg.signature();

    let key_id = metadata.key_id();
    let pubkey = key_id.to_pubkey().map_err(Error::Pubkey)?;

    let signable = json_canon::to_vec(metadata).map_err(Error::JsonCanon)?;

    pubkey
        .verify_strict(&signable, &signature.to_signature())
        .map_err(Error::Signature)?;

    Ok(())
}

pub fn validate_tangle(
    msg: &Msg,
    tangle: &Tangle,
    tangle_root_msg_id: &MsgId,
) -> Result<(), Error> {
    let metadata = msg.metadata();

    let msg_tangles = metadata.tangles();
    let msg_tangle =
        msg_tangles
            .get(tangle_root_msg_id)
            .ok_or(Error::MsgTanglesMissingTangleRootMsgId {
                root_msg_id: tangle_root_msg_id.clone(),
            })?;

    let depth = msg_tangle.depth();
    let prev_msg_ids = msg_tangle.prev_msg_ids();

    if tangle.is_feed() {
        let (feed_key_id, feed_content_type) = tangle.get_feed().unwrap();
        let content_type = metadata.content_type();
        if content_type != feed_content_type {
            return Err(Error::MsgTypeDoesNotMatchFeedType {
                content_type: content_type.to_owned(),
            });
        }
        let key_id = metadata.key_id();
        if key_id != &feed_key_id {
            return Err(Error::MsgKeyIdDoesNotMatchFeedKeyId {
                key_id: key_id.clone(),
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
            return Err(Error::TanglePrevDepthNotLower {
                prev_msg_id: prev_msg_id.clone(),
            });
        }
        if diff < min_diff {
            min_diff = diff
        }
    }

    if count_prev_unknown == prev_msg_ids.len() as u64 {
        return Err(Error::AllPrevUnknown);
    }

    if count_prev_unknown == 0 && min_diff != 1 {
        return Err(Error::DepthMustBeMaxPlusOne);
    }

    Ok(())
}

fn validate_tangle_root(
    msg: &Msg,
    msg_id: &MsgId,
    tangle_root_msg_id: &MsgId,
) -> Result<(), Error> {
    if msg_id == tangle_root_msg_id {
        Err(Error::IfEmptyTangleThenMsgIdMustMatchTangleRootMsgId)
    } else if msg.metadata().tangles().contains_key(tangle_root_msg_id) {
        Err(Error::TangleRootMustNotHaveSelfTangles)
    } else {
        Ok(())
    }
}

fn validate_content(msg: &Msg) -> Result<(), Error> {
    let content = msg.content();
    let metadata = msg.metadata();

    if content.is_null() {
        return Ok(());
    }

    let (content_hash, content_size) = content.to_hash();
    if &Some(content_hash) != metadata.content_hash() {
        Err(Error::ContentHashDoesNotMatchMetadata)
    } else if &content_size != metadata.content_size() {
        Err(Error::ContentSizeDoesNotMatchMetadata)
    } else {
        Ok(())
    }
}

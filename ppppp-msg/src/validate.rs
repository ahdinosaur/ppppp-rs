use ed25519_dalek::ed25519::Error as Ed25519Error;
use serde_json::Error as JsonError;

use crate::{author_id::AuthorId, msg::Msg, msg_hash::MsgHash, tangle::Tangle};

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
    #[error("tangle missing root message id: {root_msg_hash}")]
    MsgTanglesMissingTangleRootMsgHash { root_msg_hash: MsgHash },
    #[error("msg data type doesn't match feed type: {data_type}")]
    MsgTypeDoesNotMatchFeedType { data_type: String },
    #[error("msg key id doesn't match feed key id: {author_id}")]
    MsgAuthorIdDoesNotMatchFeedAuthorId { author_id: AuthorId },
    #[error("depth of prev {prev_msg_hash} is not lower")]
    TanglePrevDepthNotLower { prev_msg_hash: MsgHash },
    #[error("all prev are locally unknown")]
    AllPrevUnknown,
    #[error("depth must be the largest prev depth plus one")]
    DepthMustBeMaxPlusOne,
    #[error("if tangle empty, msg id must match tangle root msg id")]
    IfEmptyTangleThenMsgHashMustMatchTangleRootMsgHash,
    #[error("tangle root must not have self tangles")]
    TangleRootMustNotHaveSelfTangles,
    #[error("data size does not match metadata.size")]
    DataSizeDoesNotMatchMetadata,
    #[error("data hash does not match metadata.hash")]
    DataHashDoesNotMatchMetadata,
}

pub fn validate(
    msg: &Msg,
    msg_hash: &MsgHash,
    tangle: &Tangle,
    tangle_root_msg_hash: &MsgHash,
) -> Result<(), Error> {
    validate_version(msg)?;

    if tangle.size() == 0 {
        validate_tangle_root(msg, msg_hash, tangle_root_msg_hash)?;
    } else {
        validate_tangle(msg, tangle, tangle_root_msg_hash)?;
    }

    validate_data(msg)?;
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

    let author_id = metadata.author_id();
    let pubkey = author_id.to_pubkey().map_err(Error::Pubkey)?;

    let signable = json_canon::to_vec(metadata).map_err(Error::JsonCanon)?;

    pubkey
        .verify_strict(&signable, &signature.to_signature())
        .map_err(Error::Signature)?;

    Ok(())
}

pub fn validate_tangle(
    msg: &Msg,
    tangle: &Tangle,
    tangle_root_msg_hash: &MsgHash,
) -> Result<(), Error> {
    let metadata = msg.metadata();

    let msg_tangles = metadata.tangles();
    let msg_tangle =
        msg_tangles
            .get(tangle_root_msg_hash)
            .ok_or(Error::MsgTanglesMissingTangleRootMsgHash {
                root_msg_hash: tangle_root_msg_hash.clone(),
            })?;

    let depth = msg_tangle.depth();
    let prev_msg_hashs = msg_tangle.prev_msg_hashs();

    if tangle.is_feed() {
        let (feed_author_id, feed_data_type) = tangle.get_feed().unwrap();
        let data_type = metadata.data_type();
        if data_type != feed_data_type {
            return Err(Error::MsgTypeDoesNotMatchFeedType {
                data_type: data_type.to_owned(),
            });
        }
        let author_id = metadata.author_id();
        if author_id != &feed_author_id {
            return Err(Error::MsgAuthorIdDoesNotMatchFeedAuthorId {
                author_id: author_id.clone(),
            });
        }
    }

    let mut min_diff = u64::MAX;
    let mut count_prev_unknown = 0_u64;

    for prev_msg_hash in prev_msg_hashs {
        if !tangle.has(prev_msg_hash) {
            count_prev_unknown += 1;
            continue;
        }

        let prev_depth = tangle.get_depth(prev_msg_hash).unwrap();

        let diff = depth - prev_depth;
        if diff <= 0 {
            return Err(Error::TanglePrevDepthNotLower {
                prev_msg_hash: prev_msg_hash.clone(),
            });
        }
        if diff < min_diff {
            min_diff = diff
        }
    }

    if count_prev_unknown == prev_msg_hashs.len() as u64 {
        return Err(Error::AllPrevUnknown);
    }

    if count_prev_unknown == 0 && min_diff != 1 {
        return Err(Error::DepthMustBeMaxPlusOne);
    }

    Ok(())
}

fn validate_tangle_root(
    msg: &Msg,
    msg_hash: &MsgHash,
    tangle_root_msg_hash: &MsgHash,
) -> Result<(), Error> {
    if msg_hash == tangle_root_msg_hash {
        Err(Error::IfEmptyTangleThenMsgHashMustMatchTangleRootMsgHash)
    } else if msg.metadata().tangles().contains_key(tangle_root_msg_hash) {
        Err(Error::TangleRootMustNotHaveSelfTangles)
    } else {
        Ok(())
    }
}

fn validate_data(msg: &Msg) -> Result<(), Error> {
    let data = msg.data();
    let metadata = msg.metadata();

    if data.is_null() {
        return Ok(());
    }

    let (data_hash, data_size) = data.to_hash();
    if &Some(data_hash) != metadata.data_hash() {
        Err(Error::DataHashDoesNotMatchMetadata)
    } else if &data_size != metadata.data_size() {
        Err(Error::DataSizeDoesNotMatchMetadata)
    } else {
        Ok(())
    }
}

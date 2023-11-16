use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(try_from = "String")]
pub struct MsgDomain(pub String);

#[derive(Clone, Debug, thiserror::Error)]
pub enum MsgDomainDeserializeError {
    #[error("domain {domain} is 100+ characters long")]
    TooLong { domain: String, length: usize },
    #[error("domain {domain} is shorter than 3 characters")]
    TooShort { domain: String, length: usize },
    #[error("domain {domain} contains characters other than a-z, A-Z, 0-9, or _")]
    BadCharacters { domain: String },
}

impl TryFrom<String> for MsgDomain {
    type Error = MsgDomainDeserializeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 100 {
            Err(MsgDomainDeserializeError::TooLong {
                domain: value.clone(),
                length: value.len(),
            })
        } else if value.len() < 3 {
            Err(MsgDomainDeserializeError::TooShort {
                domain: value.clone(),
                length: value.len(),
            })
        } else if !value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            Err(MsgDomainDeserializeError::BadCharacters { domain: value })
        } else {
            Ok(MsgDomain(value))
        }
    }
}

impl Display for MsgDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Domain({})", self.0)
    }
}

use blake3::Hasher;
use json_canon::to_writer as canon_json_to_writer;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::content_hash::ContentHash;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(from = "Value")]
pub struct Content(Value);

impl Content {
    pub fn to_hash(&self) -> (ContentHash, u64) {
        let mut hasher = Hasher::new();
        canon_json_to_writer(&mut hasher, &self.0).unwrap();
        let hash = hasher.finalize();
        let size = hasher.count();

        (ContentHash::from_hash(hash), size)
    }

    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }
}

impl From<Value> for Content {
    fn from(value: Value) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::content::ContentHash;

    use super::*;

    #[test]
    fn test_hello_world() {
        let value = json!({
            "text": "hello world!"
        });
        let content: Content = value.into();
        let (hash, size): (ContentHash, _) = content.to_hash();
        assert_eq!(hash.to_string(), "Cz1jtXr2oBrhk8czWiz6kH");
        assert_eq!(size, 23);
    }
}

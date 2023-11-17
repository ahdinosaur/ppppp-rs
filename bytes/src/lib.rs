use ppppp_base58 as base58;

pub use paste::paste;

#[derive(Debug, thiserror::Error)]
pub enum DeserializeBytesError {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
}

pub trait FromBytes<const LENGTH: usize>: Sized {
    fn from_bytes(bytes: &[u8; LENGTH]) -> Self;

    fn from_base58(base58_str: &str) -> Result<Self, DeserializeBytesError> {
        let data = base58::decode(base58_str).map_err(DeserializeBytesError::DecodeBase58)?;
        if data.len() != LENGTH {
            return Err(DeserializeBytesError::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        let key = Self::from_bytes(&bytes);
        Ok(key)
    }
}

#[macro_export]
macro_rules! impl_from_bytes_inputs {
    ($Type:ty, $LENGTH:expr) => {
        $crate::paste! {
            impl TryFrom<String> for $Type {
                type Error = $crate::DeserializeBytesError;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    $Type::from_base58(&value)
                }
            }

            impl FromStr for $Type {
                type Err = $crate::DeserializeBytesError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    $Type::from_base58(s)
                }
            }

            struct [<FromBytesHumanVisitor $Type>] {}

            impl<'de> serde::de::Visitor<'de>
                for [<FromBytesHumanVisitor $Type>]
            {
                type Value = $Type;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("base58 string")
                }

                fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    $Type::from_base58(&value).map_err(|err| E::custom(err.to_string()))
                }
            }

            struct [<FromBytesRawVisitor $Type>] {}

            impl<'de> serde::de::Visitor<'de>
                for [<FromBytesRawVisitor $Type>]
            {
                type Value = $Type;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    formatter.write_str("bytes")
                }

                fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    let bytes = value.try_into().map_err(|err: std::array::TryFromSliceError| E::custom(err.to_string()))?;
                    Ok($Type::from_bytes(bytes))
                }
            }

            impl<'de> Deserialize<'de> for $Type {
                fn deserialize<D>(deserializer: D) -> Result<$Type, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    if deserializer.is_human_readable() {
                        deserializer.deserialize_str([<FromBytesHumanVisitor $Type>] {})
                    } else {
                        deserializer.deserialize_bytes([<FromBytesRawVisitor $Type>] {})
                    }
                }
            }
        }
    };
}

pub trait AsBytes<const LENGTH: usize>: Sized {
    fn as_bytes(&self) -> &[u8; LENGTH];

    fn to_base58(&self) -> String {
        let data = self.as_bytes();
        base58::encode(data)
    }
}

#[macro_export]
macro_rules! impl_as_bytes_outputs {
    ($Type:ty, $LENGTH:expr) => {
        impl From<&$Type> for String {
            fn from(value: &$Type) -> String {
                value.to_string()
            }
        }

        impl Display for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_base58())
            }
        }

        impl Serialize for $Type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                if serializer.is_human_readable() {
                    serializer.serialize_str(&self.to_string())
                } else {
                    serializer.serialize_bytes(self.as_bytes())
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        // assert!(true);
    }
}

use std::convert::Infallible;

use ppppp_base58 as base58;

pub use once_cell::sync::Lazy;
pub use paste::paste;
pub use serde;

#[derive(Debug, thiserror::Error)]
pub enum DeserializeBytesError<Error: std::error::Error = Infallible> {
    #[error("Failed to decode base58: {0}")]
    DecodeBase58(#[source] base58::DecodeError),
    #[error("Incorrect size: {size}")]
    Size { size: usize },
    #[error("{0}")]
    Bytes(#[source] Error),
}

pub trait FromBytes<const LENGTH: usize>: Sized {
    type Error: std::error::Error;

    fn from_bytes(bytes: &[u8; LENGTH]) -> Result<Self, Self::Error>;

    fn from_base58(base58_str: &str) -> Result<Self, DeserializeBytesError<Self::Error>> {
        let data = base58::decode(base58_str).map_err(DeserializeBytesError::DecodeBase58)?;
        if data.len() != LENGTH {
            return Err(DeserializeBytesError::Size { size: data.len() });
        }
        let bytes = data.try_into().unwrap();
        Self::from_bytes(&bytes).map_err(DeserializeBytesError::Bytes)
    }
}

#[macro_export]
macro_rules! impl_from_bytes_inputs {
    ($Type:ty, $LENGTH:expr) => {
        $crate::paste! {
            impl TryFrom<String> for $Type {
                type Error = $crate::DeserializeBytesError<<$Type as $crate::FromBytes<$LENGTH>>::Error>;

                fn try_from(value: String) -> Result<Self, Self::Error> {
                    $Type::from_base58(&value)
                }
            }

            impl std::str::FromStr for $Type {
                type Err = $crate::DeserializeBytesError<<$Type as $crate::FromBytes<$LENGTH>>::Error>;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    $Type::from_base58(s)
                }
            }

            impl<'de> $crate::serde::Deserialize<'de> for $Type {
                fn deserialize<D>(deserializer: D) -> Result<$Type, D::Error>
                where
                    D: $crate::serde::de::Deserializer<'de>,
                {

                    struct [<FromBytesHumanVisitor $Type>] {}

                    impl<'de> $crate::serde::de::Visitor<'de>
                        for [<FromBytesHumanVisitor $Type>]
                    {
                        type Value = $Type;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            static msg: $crate::Lazy<String> = $crate::Lazy::new(|| format!("base58 string for {} bytes", $LENGTH));

                            formatter.write_str(&msg)
                        }

                        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
                        where
                            E: $crate::serde::de::Error,
                        {
                            $Type::from_base58(&value).map_err(|err| E::custom(err.to_string()))
                        }
                    }

                    struct [<FromBytesRawVisitor $Type>] {}

                    impl<'de> $crate::serde::de::Visitor<'de>
                        for [<FromBytesRawVisitor $Type>]
                    {
                        type Value = $Type;

                        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                            static msg: $crate::Lazy<String> = $crate::Lazy::new(|| format!("{} bytes", $LENGTH));

                            formatter.write_str(&msg)
                        }

                        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                        where
                            E: $crate::serde::de::Error,
                        {
                            let bytes = value.try_into().map_err(|err: std::array::TryFromSliceError| E::custom(err.to_string()))?;
                            $Type::from_bytes(bytes).map_err(E::custom)
                        }

                        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                        where
                            A: $crate::serde::de::SeqAccess<'de>,
                        {
                            static error_msg: $crate::Lazy<String> = $crate::Lazy::new(|| format!("expected {} bytes", $LENGTH));

                            let mut bytes = [0_u8; $LENGTH];
                            #[allow(clippy::needless_range_loop)]
                            for i in 0..$LENGTH {
                                bytes[i] = seq
                                    .next_element()?
                                    .ok_or_else(|| $crate::serde::de::Error::invalid_length(i, &error_msg.as_str()))?;
                            }

                            let remaining = (0..)
                                .map(|_| seq.next_element::<u8>())
                                .take_while(|el| matches!(el, Ok(Some(_))))
                                .count();

                            if remaining > 0 {
                                return Err($crate::serde::de::Error::invalid_length(
                                    $LENGTH + remaining,
                                    &error_msg.as_str(),
                                ));
                            }

                            $Type::from_bytes(&bytes).map_err($crate::serde::de::Error::custom)
                        }
                    }

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

        impl std::fmt::Display for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_base58())
            }
        }

        impl $crate::serde::Serialize for $Type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
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

pub trait ToBytes<const LENGTH: usize>: Sized {
    fn to_bytes(&self) -> [u8; LENGTH];

    fn to_base58(&self) -> String {
        let data = self.to_bytes();
        base58::encode(&data)
    }
}

#[macro_export]
macro_rules! impl_to_bytes_outputs {
    ($Type:ty, $LENGTH:expr) => {
        impl From<&$Type> for String {
            fn from(value: &$Type) -> String {
                value.to_string()
            }
        }

        impl std::fmt::Display for $Type {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_base58())
            }
        }

        impl $crate::serde::Serialize for $Type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer,
            {
                if serializer.is_human_readable() {
                    serializer.serialize_str(&self.to_string())
                } else {
                    serializer.serialize_bytes(&self.to_bytes())
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

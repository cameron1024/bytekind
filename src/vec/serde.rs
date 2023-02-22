use core::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::{Bytes, Plain};

impl Serialize for Bytes<Plain> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Bytes<Plain> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::<u8>::deserialize(deserializer).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }
}

#[cfg(feature = "hex")]
mod hex_impl {
    use serde::de::Visitor;

    use crate::HexString;

    use super::*;

    impl Serialize for Bytes<HexString> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&hex::encode(&self.inner))
        }
    }

    impl<'de> Deserialize<'de> for Bytes<HexString> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V;

            impl Visitor<'_> for V {
                type Value = Vec<u8>;

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    formatter.write_str("a hexadecimal string")
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    hex::decode(v.trim_start_matches("0x")).map_err(E::custom)
                }
            }

            deserializer.deserialize_str(V).map(|inner| Self {
                inner,
                _marker: PhantomData,
            })
        }
    }
}

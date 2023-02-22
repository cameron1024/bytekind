use core::marker::PhantomData;

use serde::{de::Visitor, Deserialize, Serialize};
use unarray::UnarrayArrayExt;

use crate::{ByteArray, Plain};

impl<const N: usize> Serialize for ByteArray<Plain, N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.serialize(serializer)
    }
}

impl<'de, const N: usize> Deserialize<'de> for ByteArray<Plain, N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V<const N: usize>;

        impl<'de, const N: usize> Visitor<'de> for V<N> {
            type Value = [u8; N];

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(formatter, "a byte array of length {N}")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                v.try_into().map_err(E::custom)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let err_fn = |s: &'static str| -> Result<Self::Value, A::Error> {
                    Err(<A::Error as serde::de::Error>::custom(s))
                };

                // this is not the most efficient algorithm but it works
                let mut result = [None; N];

                for slot in result.iter_mut() {
                    match seq.next_element()? {
                        Some(elem) => *slot = Some(elem),
                        None => return err_fn("not enough elements"),
                    }
                }

                match seq.next_element::<u8>() {
                    Ok(None) => {}
                    Ok(Some(_)) => return err_fn("too many elements"),
                    Err(_) => return err_fn("too many elements"),
                }

                // unwrap is fine here because all elements should be `Some`
                Ok(result.map_option(|i| i).unwrap())
            }
        }

        deserializer.deserialize_bytes(V::<N>).map(|inner| Self {
            inner,
            _marker: PhantomData,
        })
    }
}

#[cfg(feature = "hex")]
mod hex_impl {
    use crate::HexString;

    use super::*;

    impl<const N: usize> Serialize for ByteArray<HexString, N> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_str(&hex::encode(self.inner))
        }
    }

    impl<'de, const N: usize> Deserialize<'de> for ByteArray<HexString, N> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            struct V<const N: usize>;

            impl<const N: usize> Visitor<'_> for V<N> {
                type Value = [u8; N];

                fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                    write!(formatter, "a hex string representing a byte array of length {N} (i.e. a hex string with length {})", N * 2)
                }

                fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    let mut buf = [0; N];
                    hex::decode_to_slice(v.trim_start_matches("0x"), &mut buf).map_err(E::custom)?;
                    Ok(buf)
                }
            }

            deserializer.deserialize_str(V::<N>).map(|inner| Self {
                inner,
                _marker: PhantomData,
            })
        }
    }
}

#[cfg(all(test, feature = "hex"))]
mod tests {
    use serde_json::{from_value, json, to_value};

    use crate::HexString;

    use super::*;

    #[derive(Debug, Deserialize, Serialize)]
    struct Foo {
        plain: ByteArray<Plain, 4>,
        hex: ByteArray<HexString, 4>,
    }

    #[test]
    fn serialize_deserialize_sanity() {
        let value = json!({
            "plain": [1, 2, 3, 4],
            "hex": "01020304",
        });

        let Foo { plain, hex } = from_value(value.clone()).unwrap();
        let value_again = to_value(&Foo { plain, hex }).unwrap();

        assert_eq!(value, value_again);
    }

    #[test]
    fn fails_if_wrong_length() {
        let plain_too_long = json!({
            "plain": [1, 2, 3, 4, 5],
            "hex": "01020304",
        });
        from_value::<Foo>(plain_too_long).unwrap_err();

        let plain_too_short = json!({
            "plain": [1, 2, 3, 4, 5],
            "hex": "01020304",
        });
        from_value::<Foo>(plain_too_short).unwrap_err();

        let hex_too_long = json!({
            "plain": [1, 2, 3, 4],
            "hex": "0102030405",
        });
        from_value::<Foo>(hex_too_long).unwrap_err();

        let hex_too_short = json!({
            "plain": [1, 2, 3, 4],
            "hex": "010203",
        });
        from_value::<Foo>(hex_too_short).unwrap_err();
    }
}

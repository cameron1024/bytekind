use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value};

use crate::{ByteArray, Bytes, Format, HexString, Plain};

#[test]
fn simple_example() {
    #[derive(Serialize, Deserialize)]
    struct Foo {
        hex: Bytes<HexString>,
        plain: Bytes<Plain>,
    }

    let value = json!({
        "hex": "01020304",
        "plain": [1, 2, 3, 4],
    });

    let Foo { hex, plain } = from_value(value.clone()).unwrap();

    assert_eq!(hex, vec![1, 2, 3, 4]);
    assert_eq!(plain, vec![1, 2, 3, 4]);

    let value_again = to_value(&Foo { hex, plain }).unwrap();
    assert_eq!(value, value_again);
}

#[test]
fn implements_traits() {
    fn foo<T>(_t: T)
    where
        T: std::fmt::Debug
            + Clone
            + PartialEq
            + Eq
            + PartialOrd
            + Ord
            + std::hash::Hash
            + AsRef<[u8]>
            + AsMut<[u8]>,
    {
    }

    foo(Bytes::<Plain>::from(vec![1, 2, 3]));
    foo(Bytes::<HexString>::from(vec![1, 2, 3]));

    foo(ByteArray::<Plain, 3>::from([1, 2, 3]));
    foo(ByteArray::<HexString, 3>::from([1, 2, 3]));

    fn _has_generic<F: Format>() {
        foo(Bytes::<F>::from(vec![1, 2, 3]));
        foo(ByteArray::<F, 3>::from([1, 2, 3]));
    }
}

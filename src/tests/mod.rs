use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value};

use crate::{Bytes, HexString, Plain};

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

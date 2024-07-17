use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_repr_base64::{base64, base64_if_readable, base64_string};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct BytesTest {
    #[serde(with = "base64")]
    byte_array: [u8; 2],
    #[serde(with = "base64")]
    bytes: Vec<u8>,
    #[serde(with = "base64_if_readable")]
    byte_array2: [u8; 2],
    #[serde(with = "base64_if_readable")]
    bytes2: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NumbersTest {
    #[serde(with = "base64")]
    byte_array: [usize; 2],
    #[serde(with = "base64")]
    bytes: Vec<i64>,
    #[serde(with = "base64_if_readable")]
    byte_array2: [usize; 2],
    #[serde(with = "base64_if_readable")]
    bytes2: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct StringTest {
    #[serde(with = "base64_string")]
    str: String,
}

fn assert_round_trips<A: PartialEq + Serialize + DeserializeOwned + Debug>(a: A) {
    let b = serde_json::to_string(&a).unwrap();
    let b: A = serde_json::from_str(&b).unwrap();
    assert_eq!(a, b);
    let c = postcard::to_allocvec(&a).unwrap();
    let c: A = postcard::from_bytes(&c).unwrap();
    assert_eq!(a, c);
}

#[test]
pub fn test() {
    assert_round_trips(StringTest {
        str: "Hello, World".into(),
    });
    assert_round_trips(BytesTest {
        byte_array: [123, 74],
        bytes: vec![1, 23, 14, 51, 125],
        byte_array2: [123, 12],
        bytes2: vec![123, 12, 84, 2],
    });
    assert_round_trips(NumbersTest {
        byte_array: [123, 74],
        bytes: vec![1, 23, 14, 51, 125],
        byte_array2: [123, 12],
        bytes2: vec![123, 12, 84, 2],
    });
}

use serde::{Deserialize, Serialize};

// fn test_decode<'a, T: serde::de::Deserialize<'a> + PartialEq + std::fmt::Debug>(expected: T, variants: &'a [&'a [u8]]) {
//     for &x in variants.iter() {
//         let v: T = embedded_msgpack::decode::from_slice(x).unwrap();
//         assert_eq!(expected, v);
//     }
// }

#[derive(Serialize, Deserialize)]
struct FuzzTest1 {
    i: i32,
}
#[test]
fn decode_fuzz1() { let _: Result<FuzzTest1, _> = embedded_msgpack::decode::from_slice(&[133, 217, 0, 201, 136, 210]); }
#[test]
fn decode_fuzz2() {
    let _: Result<FuzzTest1, _> = embedded_msgpack::decode::from_slice(&[0x85, 0xd9, 0x0, 0xd9, 0x10, 0xa5, 0xf1, 0x30, 0x85, 0x2c, 0x2c]);
}

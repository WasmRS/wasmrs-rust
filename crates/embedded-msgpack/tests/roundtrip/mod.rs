#[allow(unused_imports)]
use embedded_msgpack::encode::Binary;

fn print_slice(data: &[u8]) {
    print!("[");
    for (i, v) in data.iter().enumerate() {
        print!("{}0x{:02x}", if i > 0 { ", " } else { "" }, v);
    }
    println!("]");
}

fn test_roundtrip<T: serde::Serialize + serde::de::DeserializeOwned + PartialEq + core::fmt::Debug>(data: T) {
    let mut buf = [0u8; 1000];
    let len = embedded_msgpack::encode::serde::to_array(&data, &mut buf).unwrap();
    print_slice(&buf[..len]);
    let v = embedded_msgpack::decode::from_slice(&buf).unwrap();
    assert_eq!(data, v);
}
fn test_roundtrip_borrowed<'a, T: 'a + serde::Serialize + serde::de::Deserialize<'a> + PartialEq + core::fmt::Debug>(
    data: T,
    buf: &'a mut [u8],
) {
    let len = embedded_msgpack::encode::serde::to_array(&data, buf).unwrap();
    print_slice(&buf[..len]);
    let v = embedded_msgpack::decode::from_slice(&buf[..len]).unwrap();
    assert_eq!(data, v);
}

#[test]
fn roundtrip_nil() {
    let nil: Option<u8> = None;
    test_roundtrip(nil);
}
#[test]
fn roundtrip_bool() {
    test_roundtrip(true);
    test_roundtrip(false);
}
#[cfg(feature = "timestamp")]
#[test]
fn roundtrip_timestamp() {
    use embedded_msgpack::timestamp::Timestamp;
    test_roundtrip(Timestamp::new(1514862245, 0).unwrap());
    test_roundtrip(Timestamp::new(1514862245, 678901234).unwrap());
    test_roundtrip(Timestamp::new(2147483647, 999999999).unwrap());
    test_roundtrip(Timestamp::new(2147483648, 0).unwrap());
    test_roundtrip(Timestamp::new(2147483648, 1).unwrap());
    test_roundtrip(Timestamp::new(4294967295, 0).unwrap());
    test_roundtrip(Timestamp::new(4294967295, 999999999).unwrap());
    test_roundtrip(Timestamp::new(4294967296, 0).unwrap());
    test_roundtrip(Timestamp::new(17179869183, 999999999).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(17179869184, 0).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(-1, 0).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(-1, 999999999).unwrap());
    test_roundtrip(Timestamp::new(0, 0).unwrap());
    test_roundtrip(Timestamp::new(0, 1).unwrap());
    test_roundtrip(Timestamp::new(1, 0).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(-2208988801, 999999999).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(-2208988800, 0).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(-62167219200, 0).unwrap());
    #[cfg(feature = "timestamp96")]
    test_roundtrip(Timestamp::new(253402300799, 999999999).unwrap());
}

#[test]
fn roundtrip_int() {
    test_roundtrip(-1i32);
    test_roundtrip(-32i32);
    test_roundtrip(-33i32);
    test_roundtrip(-128i32);
    test_roundtrip(-256i32);
    test_roundtrip(-32768i32);
    test_roundtrip(-65536i32);
    test_roundtrip(-2147483648i32);
}
#[test]
fn roundtrip_uint() {
    test_roundtrip(4u32);
    test_roundtrip(4u8);
    test_roundtrip(255u8);
    test_roundtrip(255u16);
    test_roundtrip(255u32);
    test_roundtrip(256u16);
    test_roundtrip(256u32);
    test_roundtrip(65535u16);
    test_roundtrip(65535u32);
    test_roundtrip(65536u32);
    test_roundtrip(2147483647u32);
    test_roundtrip(2147483648u32);
    test_roundtrip(4294967295u32);

    test_roundtrip(4i32);
    test_roundtrip(255i32);
    test_roundtrip(256i32);
    test_roundtrip(65535i32);
    test_roundtrip(65536i32);
    test_roundtrip(2147483647i32);
}
#[cfg(feature = "u64")]
#[test]
fn roundtrip_u64() {
    test_roundtrip(4294967296u64);
    test_roundtrip(281474976710656u64);
    test_roundtrip(9223372036854775807u64);
    test_roundtrip(9223372036854775808u64);
    test_roundtrip(18446744073709551615u64);
}
#[cfg(feature = "i64")]
#[test]
fn roundtrip_i64() {
    test_roundtrip(2147483648i64);
    test_roundtrip(4294967295i64);
    test_roundtrip(-4294967296i64);
    test_roundtrip(-281474976710656i64);
    test_roundtrip(9223372036854775807i64);
    test_roundtrip(-9223372036854775807i64);
    test_roundtrip(-9223372036854775808i64);
}
#[test]
fn roundtrip_float() {
    test_roundtrip(0.5f32);
    test_roundtrip(-0.5f32);
}
#[test]
fn roundtrip_map() {
    let mut buf = [0u8; 1000];
    let map: [(&str, u32); 2] = [("abc", 34), ("def", 128)];
    test_roundtrip_borrowed(map, &mut buf);
}
#[test]
fn roundtrip_array() {
    let mut buf = [0u8; 1000];
    test_roundtrip_borrowed(["abc", "def"], &mut buf);
    test_roundtrip_borrowed([1u32, 2, 3], &mut buf);
}
#[test]
fn roundtrip_str() {
    let mut buf = [0u8; 1000];
    test_roundtrip_borrowed("", &mut buf);
    test_roundtrip_borrowed("a", &mut buf);
    test_roundtrip_borrowed("1234567890123456789012345678901", &mut buf);
    test_roundtrip_borrowed("12345678901234567890123456789012", &mut buf);
}
#[test]
fn roundtrip_bin() {
    let mut buf = [0u8; 100000];
    test_roundtrip_borrowed(Binary::new(&[]), &mut buf);
    test_roundtrip_borrowed(Binary::new(&[2]), &mut buf);
    test_roundtrip_borrowed(Binary::new(&[0, 0xff]), &mut buf);
    test_roundtrip_borrowed(Binary::new(&[1u8, 2, 3, 4, 5, 6, 7]), &mut buf);
    test_roundtrip_borrowed(Binary::new(&[10u8; 300]), &mut buf);
    #[cfg(feature = "bin32")]
    test_roundtrip_borrowed(Binary::new(&[20u8; 70000]), &mut buf);
}

#[test]
fn roundtrip_struct() {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Test {
        a: Option<i32>,
        b: u32,
    }
    test_roundtrip(Test { a: None, b: 1 });
    test_roundtrip(Test { a: Some(1), b: 2 });
}
#[test]
fn roundtrip_complex_struct() {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Test {
        a: Option<i32>,
        b: u32,
        c: (i8, [u8; 3]),
    }
    test_roundtrip(Test {
        a: None,
        b: 1,
        c: (2, [3, 4, 5]),
    });
    test_roundtrip(Test {
        a: Some(1),
        b: 2,
        c: (2, [3, 4, 5]),
    });
}

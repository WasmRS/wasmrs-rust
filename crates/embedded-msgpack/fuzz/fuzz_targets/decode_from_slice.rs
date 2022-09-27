#![no_main]
use libfuzzer_sys::fuzz_target;
use serde::{Serialize, Deserialize};
use embedded_msgpack;

#[derive(Serialize, Deserialize)]
struct TestNested<'a> {
    x: &'a str,
}

#[derive(Serialize, Deserialize)]
struct TestData<'a> {
    s: &'a str,
    b: &'a [u8],
    o: Option<bool>,
    i1:i8,
    u1:u8,
    i2:i16,
    u2:u16,
    i4:i32,
    u4:u32,
    il:i64,
    ul:u64,
    n:TestNested<'a>,
    a:[TestNested<'a>; 2],
}

fuzz_target!(|data: &[u8]| {
     let _: Result<TestData,_> = embedded_msgpack::decode::from_slice(data);
});

fn main() {
    let stringified_bytes: String = std::env::args().skip(1).take(1).collect();
    println!("Decoding: {}", stringified_bytes);
    let bytes: Vec<_> = stringified_bytes
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|v| v.trim())
        .map(|v| v.parse::<u8>().unwrap())
        .collect();
    let frame = wasmrs_rsocket::Frame::decode(bytes).unwrap();
    println!("{:#?}", frame);
}

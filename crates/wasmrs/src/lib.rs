mod frames;
mod util;

mod generated;

pub fn decode_stream_id(bytes: Vec<u8>) -> u32 {
    let payload = frames::payload::Payload {
        stream_id: 1234,
        metadata: b"".to_vec(),
        data: b"".to_vec(),
        follows: false,
        complete: false,
        next: false,
    };
    payload.stream_id
}

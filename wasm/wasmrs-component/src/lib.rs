#[no_mangle]
pub fn gen_payload() -> u32 {
    let payload = wasmrs::decode_stream_id(vec![]);
    payload
}

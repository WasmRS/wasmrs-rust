mod generated;

use generated::Hello;
use guest::*;
use wasmrs_guest as guest;

#[no_mangle]
extern "C" fn __wasmrs_init(
    guest_buffer_size: u32,
    host_buffer_size: u32,
    max_host_frame_len: u32,
) {
    guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);
    guest::register_request_response("greeting", "sayHello", hello_wrapper);
}

fn init() {}

fn hello_wrapper(input_stream: IncomingStream) -> Result<OutgoingStream, GenericError> {
    crate::Hello::start(input_stream)
}

impl Hello {
    async fn task(mut self) -> Result<(), GenericError> {
        // Real user task
        while let Some(Ok(msg)) = self.inputs.msg.next().await {
            println!("got message in wasm {}", msg);
            self.outputs
                .msg
                .send("This is my return message".to_owned())
                .unwrap();
        }
        Ok(())
    }
}

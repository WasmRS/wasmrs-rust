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
        let all: Vec<_> = self.inputs.msg.collect().await;
        self.outputs
            .msg
            .send(format!("Got {} messages", all.len()))
            .unwrap();

        Ok(())
    }
}

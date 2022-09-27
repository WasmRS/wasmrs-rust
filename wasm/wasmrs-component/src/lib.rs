mod generated;
mod guest;
mod macros;

use futures_util::StreamExt;
use generated::Hello;
use guest::*;

fn init() {
    guest::register_request_response("greeting", "sayHello", hello_wrapper);
}

fn hello_wrapper(input_stream: IncomingStream) -> Result<OutgoingStream, GenericError> {
    crate::Hello::start(input_stream)
}

impl Hello {
    async fn task(mut self) -> Result<(), GenericError> {
        // Real user task
        while let Some(Ok(msg)) = self.inputs.msg.next().await {
            self.outputs
                .msg
                .send("This is my return message".to_owned())
                .unwrap();
        }
        Ok(())
    }
}

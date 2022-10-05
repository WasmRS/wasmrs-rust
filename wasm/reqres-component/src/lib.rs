mod generated;

use generated::*;
use guest::*;
use wasmrs_guest as guest;

#[no_mangle]
extern "C" fn __wasmrs_init(
    guest_buffer_size: u32,
    host_buffer_size: u32,
    max_host_frame_len: u32,
) {
    guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);
    // guest::register_fire_and_forget(
    //     "echo",
    //     "fire_and_forget",
    //     crate::GEN_FNF::fire_and_forget_wrapper,
    // );
    guest::register_request_response(
        "echo",
        "request_response",
        crate::GEN_RR::request_response_wrapper,
    );
    guest::register_request_stream(
        "echo",
        "request_stream",
        crate::GEN_RS::request_stream_wrapper,
    );
    guest::register_request_channel(
        "echo",
        "request_channel",
        crate::GEN_RC::request_channel_wrapper,
    );
}

impl GEN_RR {
    async fn task(
        self,
        input: GEN_RR_INPUTS,
        mut output: GEN_RR_OUTPUTS,
    ) -> Result<GEN_RR_OUTPUTS, GenericError> {
        let result = input.await;
        println!("REQUEST_RESPONSE: {:?}", result);
        if let Ok(msg) = result {
            output.success(format!("I got : {}", msg))
        } else {
            output.error(PayloadError::application_error("Did not receive message"))
        };
        Ok(output)
    }
}

impl GEN_RS {
    async fn task(
        self,
        input: GEN_RS_INPUTS,
        output: GEN_RS_OUTPUTS,
    ) -> Result<GEN_RS_OUTPUTS, GenericError> {
        // Real user task
        if let Ok(msg) = input.await {
            output.send(format!("I got: {}", msg)).unwrap();
        }
        Ok(output)
    }
}

impl GEN_RC {
    async fn task(
        self,
        mut inputs: GEN_RC_INPUTS,
        outputs: GEN_RC_OUTPUTS,
    ) -> Result<GEN_RC_OUTPUTS, GenericError> {
        // Real user task
        while let Some(Ok(msg)) = inputs.next().await {
            outputs.send(format!("I got: {}", msg)).unwrap();
        }
        Ok(outputs)
    }
}

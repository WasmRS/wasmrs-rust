use wasmrs_guest::select_all;
use wasmrs_guest::StreamExt;

use crate::guest::*;

#[derive()]
pub(crate) struct HelloInputs {
    pub(crate) msg: FluxStream<String, PayloadError>,
}

pub(crate) struct HelloOutputs {
    pub(crate) msg: FluxChannel<String, PayloadError>,
}

pub(crate) struct Hello {
    pub(crate) inputs: HelloInputs,
    pub(crate) outputs: HelloOutputs,
}

impl Process for Hello {
    fn start(input_stream: IncomingStream) -> ProcessReturnValue {
        // generated
        let hello_msg_channel = FluxChannel::<String, PayloadError>::new();
        let hello_msg_stream = hello_msg_channel.observer().unwrap();

        spawn(async move {
            while let Ok(Some(Ok(payload))) = input_stream.recv().await {
                #[allow(clippy::single_match)]
                match payload.metadata.namespace.as_str() {
                    "greeting" => {
                        hello_msg_channel
                            .send_result(deserialize(&payload.data).map_err(|e| e.into()));
                    }
                    _ => {
                        // how to handle errors?
                    }
                }
            }
        });
        let output_stream = OutgoingStream::new();
        let output_hello_msg_channel = FluxChannel::<String, PayloadError>::new();
        let output_hello_msg_stream = output_hello_msg_channel
            .observer()
            .unwrap()
            .map(|v| v.and_then(|v| Ok(serialize(&v)?)));

        let inner = output_stream.clone();
        spawn(async move {
            let mut futures = select_all(vec![output_hello_msg_stream]);
            while let Some(bytes) = futures.next().await {
                inner.send_result(bytes.map(|b| Payload::new_optional(None, Some(Bytes::from(b)))));
            }
        });

        spawn(async move {
            let _result = Hello {
                inputs: HelloInputs {
                    msg: hello_msg_stream,
                },
                outputs: HelloOutputs {
                    msg: output_hello_msg_channel,
                },
            }
            .task()
            .await;
        });

        Ok(output_stream)
    }
}

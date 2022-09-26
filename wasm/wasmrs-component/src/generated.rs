use futures::{stream::select_all, StreamExt};
use wapc_codec::messagepack::{deserialize, serialize};
use wasmrs::flux::{FluxBox, FluxChannel};

use crate::guest::*;

#[derive()]
pub(crate) struct HelloInputs {
    pub(crate) msg: FluxBox<String, ()>,
}

pub(crate) struct HelloOutputs {
    pub(crate) msg: FluxChannel<String, ()>,
}

pub(crate) struct Hello {
    pub(crate) inputs: HelloInputs,
    pub(crate) outputs: HelloOutputs,
}

impl Process for Hello {
    fn start(input_stream: IncomingStream) -> ProcessReturnValue {
        // generated
        let hello_msg_channel = FluxChannel::<String, ()>::new();
        let hello_msg_stream = hello_msg_channel.take_receiver().unwrap();

        spawn(async move {
            while let Ok(Some(Ok(payload))) = input_stream.recv().await {
                let result = match payload.metadata.namespace.as_str() {
                    "greeting" => match deserialize(&payload.data) {
                        Ok(v) => hello_msg_channel.send(v),
                        Err(_) => hello_msg_channel.error(()),
                    },
                    x => Err(wasmrs::Error::PortNotFound(x.to_owned())),
                };
            }
        });
        let output_stream = OutgoingStream::new();
        let output_hello_msg_channel = FluxChannel::<String, ()>::new();
        let output_hello_msg_stream = output_hello_msg_channel
            .take_receiver()
            .unwrap()
            .map(|v| serialize(&v.unwrap()).unwrap());

        let inner = output_stream.clone();
        spawn(async move {
            let mut futs = select_all(vec![output_hello_msg_stream]);
            while let Some(bytes) = futs.next().await {
                inner.send(bytes.into());
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

mod guest;

use futures::future::select_all;
use futures::stream::StreamExt;
use guest::GenericError;

use wasmflow_codec::messagepack::{deserialize, serialize};
use wasmrs_rsocket::flux::{FluxBox, FluxChannel};

use self::guest::{IncomingStream, OutgoingStream, Process, ProcessReturnValue};

fn init() {
    guest::register_request_response("greeting", "sayHello", hello_wrapper);
}

fn hello_wrapper(input_stream: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let stream = crate::Hello::start(input_stream)?;
    //println!("returning from wrapper");
    Ok(stream)
}

#[derive()]
struct HelloInputs {
    pub msg: FluxBox<String, ()>,
}

struct HelloOutputs {
    pub msg: FluxChannel<String, ()>,
}

struct Hello {
    inputs: HelloInputs,
    outputs: HelloOutputs,
}

impl Hello {
    async fn task(mut self) -> Result<(), GenericError> {
        //println!("in component task");
        while let Some(Ok(msg)) = self.inputs.msg.next().await {
            //println!("Got message! {}", msg);
            self.outputs
                .msg
                .send("This is my return message".to_owned())
                .unwrap();
        }
        Ok(())
    }
}

impl Process for Hello {
    fn start(input_stream: IncomingStream) -> ProcessReturnValue {
        //println!("started task");
        let hello_msg_channel = FluxChannel::<String, ()>::new();
        let hello_msg_stream = hello_msg_channel.take_receiver().unwrap();
        yielding_executor::single_threaded::spawn(async move {
            //println!("in async stream processor");
            while let Ok(Some(Ok(payload))) = input_stream.recv().await {
                let result = match payload.metadata.namespace.as_str() {
                    "greeting" => match deserialize(&payload.data) {
                        Ok(v) => hello_msg_channel.send(v),
                        Err(_) => hello_msg_channel.error(()),
                    },
                    _ => Err(99),
                };
            }
        });
        let inputs = HelloInputs {
            msg: hello_msg_stream,
        };
        let output_stream = OutgoingStream::new();
        let output_hello_msg_channel = FluxChannel::<String, ()>::new();
        let output_hello_msg_stream = output_hello_msg_channel.take_receiver().unwrap();
        let mut output_hello_msg_stream = output_hello_msg_stream.map(|v| serialize(&v).unwrap());
        let inner = output_stream.clone();
        yielding_executor::single_threaded::spawn(async move {
            let futs = vec![output_hello_msg_stream.next()];
            let (result, this, left) = select_all(futs).await;
            if let Some(bytes) = result {
                inner.send(bytes.into());
            }
        });

        let outputs = HelloOutputs {
            msg: output_hello_msg_channel,
        };

        let component = Hello { inputs, outputs };
        // yielding_executor::single_threaded::spawn(async move {
        //   while select
        // });
        yielding_executor::single_threaded::spawn(component.task());

        Ok(output_stream)
    }
}

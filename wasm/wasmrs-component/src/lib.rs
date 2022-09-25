mod guest;

use std::marker::PhantomData;

use bytes::Bytes;
use guest::GenericError;

use rxrust::{
    prelude::{Observer, SubscribeNext},
    subject::LocalSubject,
};
use serde::Serialize;
use wasmflow_codec::messagepack::{deserialize, serialize};

use self::guest::{IncomingStream, OutgoingStream, Process, ProcessReturnValue};

fn init() {
    guest::register_request_response("greeting", "sayHello", hello_wrapper);
}

fn hello_wrapper(input_stream: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let stream = crate::Hello::start(input_stream)?;
    Ok(stream)
}

#[derive(Default, Clone)]
struct HelloInputs<'a> {
    pub msg: LocalSubject<'a, String, ()>,
}

struct HelloOutputs<'a> {
    pub msg: Sink<'a, String>,
}

struct Hello<'a> {
    inputs: HelloInputs<'a>,
    outputs: HelloOutputs<'a>,
}

impl<'a> Hello<'a> {
    fn task(mut self) -> Result<(), GenericError> {
        self.inputs.msg.subscribe(move |msg| {
            self.outputs
                .msg
                .next("This is my return message".to_owned());
        });
        Ok(())
    }
}

pub struct Sink<'a, T> {
    name: String,
    observer: LocalSubject<'a, Bytes, ()>,
    phantom: PhantomData<T>,
    complete: bool,
}
impl<'a, T> Sink<'a, T> {
    pub fn new(name: impl AsRef<str>, observer: LocalSubject<'a, Bytes, ()>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            observer,
            phantom: PhantomData::default(),
            complete: false,
        }
    }
}

impl<'a, T> Observer for Sink<'a, T>
where
    T: Serialize,
{
    type Item = T;

    type Err = ();

    fn next(&mut self, value: Self::Item) {
        if !self.complete {
            match serialize(&value) {
                Ok(bytes) => self.observer.next(bytes.into()),
                Err(_) => self.observer.error(()),
            }
        }
    }

    fn error(&mut self, err: Self::Err) {
        if !self.complete {
            self.observer.error(())
        }
    }

    fn complete(&mut self) {
        self.complete = true;
    }
}

impl<'a> Process for Hello<'a> {
    fn start(input_stream: IncomingStream) -> ProcessReturnValue {
        let inputs = HelloInputs::default();
        let mut output_stream = OutgoingStream::new();
        let outputs = HelloOutputs {
            msg: Sink::new("msg", output_stream.clone()),
        };

        let mut inner = inputs.clone();
        input_stream.subscribe(move |payload| match payload.metadata.namespace.as_str() {
            "greeting" => {
                inner.msg.next(deserialize(&payload.data).unwrap());
            }
            _ => {}
        });

        let component = Hello { inputs, outputs };
        if let Err(e) = component.task() {
            output_stream.error(());
        };

        Ok(output_stream)
    }
}

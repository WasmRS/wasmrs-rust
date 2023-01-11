pub(crate) mod test {
  pub(crate) use super::*;
  pub(crate) mod chars;
  pub(crate) mod echo;
  pub(crate) mod reverse;
  pub(crate) mod test;
}
/************************************************
 * THIS FILE IS GENERATED, DO NOT EDIT          *
 *                                              *
 * See https://apexlang.io for more information *
 ***********************************************/
use wasmrs_guest::FutureExt;

use wasmrs_guest::*;

#[no_mangle]
extern "C" fn __wasmrs_init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  init_exports();
  init_imports();
  wasmrs_guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);
}

fn deserialize_helper<T: serde::de::DeserializeOwned + 'static>(
  i: Mono<ParsedPayload, PayloadError>,
) -> Mono<T, PayloadError> {
  Mono::from_future(async move {
    match i.await {
      Ok(bytes) => match deserialize(&bytes.data) {
        Ok(v) => Ok(v),
        Err(e) => Err(PayloadError::application_error(e.to_string())),
      },
      Err(e) => Err(PayloadError::application_error(e.to_string())),
    }
  })
}

pub(crate) struct TestComponent();

impl TestComponent {
  fn test_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();

    let input = Mono::from_future(input.map(|r| r.map(|v| Ok(deserialize(&v.data)?))?));
    let task = TestComponent::test(input)
      .map(|result| {
        let output = result?;
        Ok(serialize(&output).map(|bytes| Payload::new_data(None, Some(bytes.into())))?)
      })
      .map(|output| tx.send(output).unwrap());

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }

  fn echo_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();

    let input = Mono::from_future(input.map(|r| r.map(|v| Ok(deserialize(&v.data)?))?));
    let task = TestComponent::echo(input)
      .map(|result| {
        let output = result?;
        Ok(serialize(&output).map(|bytes| Payload::new_data(None, Some(bytes.into())))?)
      })
      .map(|output| tx.send(output).unwrap());

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }

  fn chars_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    // generated

    let (out_tx, out_rx) = Flux::new_channels();

    let input = deserialize_helper(input);

    spawn(async move {
      let task = Self {};
      let (outputs_tx, mut outputs_rx) = Flux::new_channels();
      let outputs = outputs_tx;
      match TestComponent::chars(input, outputs).await {
        Ok(_) => {
          while let Some(next) = outputs_rx.next().await {
            let out = match next {
              Ok(output) => match serialize(&output) {
                Ok(bytes) => Ok(Payload::new_data(None, Some(bytes.into()))),
                Err(e) => Err(PayloadError::application_error(e.to_string())),
              },
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      };
    });

    Ok(out_rx)
  }

  fn reverse_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    // generated
    let (inputs_tx, inputs_rx) = Flux::<test_service::reverse::Inputs, PayloadError>::new_channels();

    spawn(async move {
      while let Ok(Some(Ok(payload))) = input.recv().await {
        inputs_tx.send_result(deserialize(&payload.data).map_err(|e| e.into()));
      }
    });
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (outputs_tx, mut outputs_rx) = Flux::new_channels();

    spawn(async move {
      while let Some(result) = outputs_rx.next().await {
        match result {
          Ok(payload) => match serialize(&payload) {
            Ok(bytes) => {
              real_out_tx.send(Payload::new_data(None, Some(Bytes::from(bytes))));
            }
            Err(e) => {
              real_out_tx.error(PayloadError::application_error(e.to_string()));
            }
          },
          Err(err) => {
            real_out_tx.error(err);
          }
        }
      }
    });

    spawn(async move {
      let _result = TestComponent::reverse(inputs_rx, outputs_tx).await;
    });

    Ok(real_out_rx)
  }
}

#[async_trait::async_trait(?Send)]
/// Test interface
pub(crate) trait TestService {
  /// Returns 'test'.
  async fn test(
    inputs: Mono<test_service::test::Inputs, PayloadError>,
  ) -> Result<test_service::test::Outputs, GenericError>;
  /// Returns what is sent.
  async fn echo(
    inputs: Mono<test_service::echo::Inputs, PayloadError>,
  ) -> Result<test_service::echo::Outputs, GenericError>;
  /// Returns a stream of a string's characters.
  async fn chars(
    inputs: Mono<test_service::chars::Inputs, PayloadError>,
    outputs: Flux<test_service::chars::Outputs, PayloadError>,
  ) -> Result<Flux<test_service::chars::Outputs, PayloadError>, GenericError>;
  /// Returns each string in the input stream, reversed.
  async fn reverse(
    inputs: FluxReceiver<test_service::reverse::Inputs, PayloadError>,
    outputs: Flux<test_service::reverse::Outputs, PayloadError>,
  ) -> Result<Flux<test_service::reverse::Outputs, PayloadError>, GenericError>;
}

#[async_trait::async_trait(?Send)]
impl TestService for TestComponent {
  /// Returns 'test'.
  async fn test(
    inputs: Mono<test_service::test::Inputs, PayloadError>,
  ) -> Result<test_service::test::Outputs, GenericError> {
    Ok(crate::actions::test::test::task(inputs.await?).await?)
  }

  /// Returns what is sent.
  async fn echo(
    inputs: Mono<test_service::echo::Inputs, PayloadError>,
  ) -> Result<test_service::echo::Outputs, GenericError> {
    Ok(crate::actions::test::echo::task(inputs.await?).await?)
  }

  /// Returns a stream of a string's characters.
  async fn chars(
    inputs: Mono<test_service::chars::Inputs, PayloadError>,
    outputs: Flux<test_service::chars::Outputs, PayloadError>,
  ) -> Result<Flux<test_service::chars::Outputs, PayloadError>, GenericError> {
    Ok(crate::actions::test::chars::task(inputs.await?, outputs).await?)
  }

  /// Returns each string in the input stream, reversed.
  async fn reverse(
    inputs: FluxReceiver<test_service::reverse::Inputs, PayloadError>,
    outputs: Flux<test_service::reverse::Outputs, PayloadError>,
  ) -> Result<Flux<test_service::reverse::Outputs, PayloadError>, GenericError> {
    Ok(crate::actions::test::reverse::task(inputs, outputs).await?)
  }
}

pub mod test_service {
  #[allow(unused_imports)]
  pub(crate) use super::*;

  pub mod test {
    #[allow(unused_imports)]
    pub(crate) use super::*;
    #[derive(serde::Deserialize)]
    pub(crate) struct Inputs {}

    pub(crate) type Outputs = String;
  }

  pub mod echo {
    #[allow(unused_imports)]
    pub(crate) use super::*;
    #[derive(serde::Deserialize)]
    pub(crate) struct Inputs {
      #[serde(rename = "message")]
      pub(crate) message: String,
    }

    pub(crate) type Outputs = String;
  }

  pub mod chars {
    #[allow(unused_imports)]
    pub(crate) use super::*;
    #[derive(serde::Deserialize)]
    pub(crate) struct Inputs {
      #[serde(rename = "input")]
      pub(crate) input: String,
    }

    pub(crate) type Outputs = String;
  }

  pub mod reverse {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Inputs = String;

    pub(crate) type Outputs = String;
  }
}

pub(crate) fn init_imports() {}
pub(crate) fn init_exports() {
  wasmrs_guest::register_request_response("suite.test", "test", TestComponent::test_wrapper);

  wasmrs_guest::register_request_response("suite.test", "echo", TestComponent::echo_wrapper);

  wasmrs_guest::register_request_stream("suite.test", "chars", TestComponent::chars_wrapper);

  wasmrs_guest::register_request_channel("suite.test", "reverse", TestComponent::reverse_wrapper);
}

/************************************************
 * THIS FILE IS GENERATED, DO NOT EDIT          *
 *                                              *
 * See https://apexlang.io for more information *
 ***********************************************/
pub(crate) mod test {
  pub(crate) use super::*;
  pub(crate) mod chars;
  pub(crate) mod echo;
  pub(crate) mod reverse;
  pub(crate) mod test;
  pub(crate) mod wrap;
}

use wasmrs_guest::FutureExt;

use wasmrs_guest::*;

#[no_mangle]
extern "C" fn __wasmrs_init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  wasmrs_guest::init_logging();

  init_exports();
  init_imports();
  wasmrs_guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);
}

fn deserialize_helper(
  i: Mono<ParsedPayload, PayloadError>,
) -> Mono<std::collections::BTreeMap<String, wasmrs_guest::Value>, PayloadError> {
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

    let input = deserialize_helper(input);

    let task = async move {
      let input_payload = match input.await {
        Ok(i) => i,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      use wasmrs_guest::Value;
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<test_service::test::Inputs, Error> {
        Ok(test_service::test::Inputs {})
      }

      let input = match des(input_payload) {
        Ok(i) => i,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      };

      TestComponent::test(input)
        .await
        .map(|result| Ok(serialize(&result).map(|bytes| Payload::new_data(None, Some(bytes.into())))?))
        .map(|output| tx.send(output).unwrap());
    };

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }

  fn echo_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();

    let input = deserialize_helper(input);

    let task = async move {
      let input_payload = match input.await {
        Ok(i) => i,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      use wasmrs_guest::Value;
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<test_service::echo::Inputs, Error> {
        Ok(test_service::echo::Inputs {
          message: <String as serde::Deserialize>::deserialize(
            map
              .remove("message")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("message".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }

      let input = match des(input_payload) {
        Ok(i) => i,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      };

      TestComponent::echo(input)
        .await
        .map(|result| Ok(serialize(&result).map(|bytes| Payload::new_data(None, Some(bytes.into())))?))
        .map(|output| tx.send(output).unwrap());
    };

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }

  fn chars_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    // generated

    let (out_tx, out_rx) = Flux::new_channels();

    let input = deserialize_helper(input);

    spawn(async move {
      let input_payload = match input.await {
        Ok(i) => i,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      use wasmrs_guest::Value;
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<test_service::chars::Inputs, Error> {
        Ok(test_service::chars::Inputs {
          input: <String as serde::Deserialize>::deserialize(
            map
              .remove("input")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("input".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }

      let input = match des(input_payload) {
        Ok(i) => i,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };

      match TestComponent::chars(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
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

    let (real_input_tx, real_input_rx) = Flux::new_channels();

    let (real_out_tx, real_out_rx) = Flux::new_channels();

    let input_inner_tx = real_input_tx.clone();
    spawn(async move {
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        use wasmrs_guest::Value;
        let des = move |payload: ParsedPayload| -> Result<test_service::reverse::Inputs, Error> {
          println!("deserializing {:2x?}", payload.data);
          let mut map = deserialize_generic(&payload.data)?;
          let input = test_service::reverse::Inputs { input: real_input_rx };
          println!("map: {:?}", map);

          if let Some(v) = map.remove("input") {
            println!("value: {:?}", v);
            input_inner_tx.send_result(
              <String as serde::Deserialize>::deserialize(v)
                .map_err(|e| PayloadError::application_error(e.to_string())),
            );
          }
          Ok(input)
        };

        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("input") {
                real_input_tx.send_result(
                  <String as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });

        match des(first) {
          Ok(i) => i,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      let result = TestComponent::reverse(input_map).await;
      if let Err(e) = result {
        real_out_tx.error(PayloadError::application_error(e.to_string()));
      } else {
        let mut result = result.unwrap();
        while let Some(result) = result.next().await {
          match result {
            Ok(output) => {
              let _ = real_out_tx.send_result(
                serialize(&output)
                  .map(|b| Payload::new_data(None, Some(b.into())))
                  .map_err(|e| PayloadError::application_error(e.to_string())),
              );
            }
            Err(e) => {
              let _ = real_out_tx.error(e);
            }
          }
        }
      }
    });

    Ok(real_out_rx)
  }

  fn wrap_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    // generated
    let (inputs_tx, inputs_rx) = Flux::<test_service::wrap::Inputs, PayloadError>::new_channels();

    let (real_input_tx, real_input_rx) = Flux::new_channels();

    let (real_out_tx, real_out_rx) = Flux::new_channels();

    let input_inner_tx = real_input_tx.clone();
    spawn(async move {
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        use wasmrs_guest::Value;
        let des = move |payload: ParsedPayload| -> Result<test_service::wrap::Inputs, Error> {
          println!("deserializing {:2x?}", payload.data);
          let mut map = deserialize_generic(&payload.data)?;
          let input = test_service::wrap::Inputs {
            left: <String as serde::Deserialize>::deserialize(
              map
                .remove("left")
                .ok_or_else(|| wasmrs_guest::Error::MissingInput("left".to_owned()))?,
            )
            .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
            right: <String as serde::Deserialize>::deserialize(
              map
                .remove("right")
                .ok_or_else(|| wasmrs_guest::Error::MissingInput("right".to_owned()))?,
            )
            .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
            input: real_input_rx,
          };
          println!("map: {:?}", map);

          if let Some(v) = map.remove("input") {
            println!("value: {:?}", v);
            input_inner_tx.send_result(
              <String as serde::Deserialize>::deserialize(v)
                .map_err(|e| PayloadError::application_error(e.to_string())),
            );
          }
          Ok(input)
        };

        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("input") {
                real_input_tx.send_result(
                  <String as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });

        match des(first) {
          Ok(i) => i,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      let result = TestComponent::wrap(input_map).await;
      if let Err(e) = result {
        real_out_tx.error(PayloadError::application_error(e.to_string()));
      } else {
        let mut result = result.unwrap();
        while let Some(result) = result.next().await {
          match result {
            Ok(output) => {
              let _ = real_out_tx.send_result(
                serialize(&output)
                  .map(|b| Payload::new_data(None, Some(b.into())))
                  .map_err(|e| PayloadError::application_error(e.to_string())),
              );
            }
            Err(e) => {
              let _ = real_out_tx.error(e);
            }
          }
        }
      }
    });

    Ok(real_out_rx)
  }
}

#[async_trait::async_trait(?Send)]
/// Test interface
pub(crate) trait TestService {
  /// Returns 'test'.
  async fn test(inputs: test_service::test::Inputs) -> Result<test_service::test::Outputs, GenericError>;
  /// Returns what is sent.
  async fn echo(inputs: test_service::echo::Inputs) -> Result<test_service::echo::Outputs, GenericError>;
  /// Returns a stream of a string's characters.
  async fn chars(inputs: test_service::chars::Inputs) -> Result<test_service::chars::Outputs, GenericError>;
  /// Returns each string in the input stream, reversed.
  async fn reverse(inputs: test_service::reverse::Inputs) -> Result<test_service::reverse::Outputs, GenericError>;
  /// Wrap each string in the input stream with the given left and right strings.
  async fn wrap(inputs: test_service::wrap::Inputs) -> Result<test_service::wrap::Outputs, GenericError>;
}

#[async_trait::async_trait(?Send)]
impl TestService for TestComponent {
  /// Returns 'test'.
  async fn test(inputs: test_service::test::Inputs) -> Result<test_service::test::Outputs, GenericError> {
    Ok(crate::actions::test::test::task(inputs).await?)
  }

  /// Returns what is sent.
  async fn echo(inputs: test_service::echo::Inputs) -> Result<test_service::echo::Outputs, GenericError> {
    Ok(crate::actions::test::echo::task(inputs).await?)
  }

  /// Returns a stream of a string's characters.
  async fn chars(inputs: test_service::chars::Inputs) -> Result<test_service::chars::Outputs, GenericError> {
    Ok(crate::actions::test::chars::task(inputs).await?)
  }

  /// Returns each string in the input stream, reversed.
  async fn reverse(inputs: test_service::reverse::Inputs) -> Result<test_service::reverse::Outputs, GenericError> {
    Ok(crate::actions::test::reverse::task(inputs).await?)
  }

  /// Wrap each string in the input stream with the given left and right strings.
  async fn wrap(inputs: test_service::wrap::Inputs) -> Result<test_service::wrap::Outputs, GenericError> {
    Ok(crate::actions::test::wrap::task(inputs).await?)
  }
}

pub mod test_service {
  #[allow(unused_imports)]
  pub(crate) use super::*;

  pub mod test {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) struct Inputs {}

    pub(crate) type Outputs = String;
  }

  pub mod echo {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) struct Inputs {
      pub(crate) message: String,
    }

    pub(crate) type Outputs = String;
  }

  pub mod chars {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) struct Inputs {
      pub(crate) input: String,
    }

    pub(crate) type Outputs = FluxReceiver<String, PayloadError>;
  }

  pub mod reverse {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) struct Inputs {
      pub(crate) input: FluxReceiver<String, PayloadError>,
    }

    pub(crate) type Outputs = FluxReceiver<String, PayloadError>;
  }

  pub mod wrap {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) struct Inputs {
      pub(crate) left: String,

      pub(crate) right: String,

      pub(crate) input: FluxReceiver<String, PayloadError>,
    }

    pub(crate) type Outputs = FluxReceiver<String, PayloadError>;
  }
}

pub(crate) fn init_imports() {}
pub(crate) fn init_exports() {
  wasmrs_guest::register_request_response("suite.test", "test", TestComponent::test_wrapper);

  wasmrs_guest::register_request_response("suite.test", "echo", TestComponent::echo_wrapper);

  wasmrs_guest::register_request_stream("suite.test", "chars", TestComponent::chars_wrapper);

  wasmrs_guest::register_request_channel("suite.test", "reverse", TestComponent::reverse_wrapper);

  wasmrs_guest::register_request_channel("suite.test", "wrap", TestComponent::wrap_wrapper);
}

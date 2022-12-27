
pub(crate) mod test {
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

pub(crate) struct TestComponent();

impl TestComponent {
  fn test_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();

    let input = Mono::from_future(input.map(|r| r.map(|v| Ok(deserialize(&v.data)?))?));
    let task = TestComponent::test(input)
      .map(|result| {
        let output = result?;
        Ok(serialize(&output).map(|bytes| Payload::new_optional(None, Some(bytes.into())))?)
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
        Ok(serialize(&output).map(|bytes| Payload::new_optional(None, Some(bytes.into())))?)
      })
      .map(|output| tx.send(output).unwrap());

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }

  fn reverse_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();

    let input = Mono::from_future(input.map(|r| r.map(|v| Ok(deserialize(&v.data)?))?));
    let task = TestComponent::reverse(input)
      .map(|result| {
        let output = result?;
        Ok(serialize(&output).map(|bytes| Payload::new_optional(None, Some(bytes.into())))?)
      })
      .map(|output| tx.send(output).unwrap());

    spawn(task);

    Ok(Mono::from_future(async move { rx.await? }))
  }
}

#[async_trait::async_trait(?Send)]
/// Test interface
pub(crate) trait TestService {
  /// Returns 'test'
  async fn test(
    inputs: Mono<test_service::test::Inputs, PayloadError>,
  ) -> Result<test_service::test::Outputs, GenericError>;
  /// Returns what is sent
  async fn echo(
    inputs: Mono<test_service::echo::Inputs, PayloadError>,
  ) -> Result<test_service::echo::Outputs, GenericError>;
  /// Returns the input string reversed
  async fn reverse(
    inputs: Mono<test_service::reverse::Inputs, PayloadError>,
  ) -> Result<test_service::reverse::Outputs, GenericError>;
}

#[async_trait::async_trait(?Send)]
impl TestService for TestComponent {
  /// Returns 'test'
  async fn test(
    inputs: Mono<test_service::test::Inputs, PayloadError>,
  ) -> Result<test_service::test::Outputs, GenericError> {
    Ok(crate::actions::test::test::task(inputs.await?).await?)
  }

  /// Returns what is sent
  async fn echo(
    inputs: Mono<test_service::echo::Inputs, PayloadError>,
  ) -> Result<test_service::echo::Outputs, GenericError> {
    Ok(crate::actions::test::echo::task(inputs.await?).await?)
  }

  /// Returns the input string reversed
  async fn reverse(
    inputs: Mono<test_service::reverse::Inputs, PayloadError>,
  ) -> Result<test_service::reverse::Outputs, GenericError> {
    Ok(crate::actions::test::reverse::task(inputs.await?).await?)
  }
}

pub mod test_service {
  use super::*;

  pub mod test {
    use super::*;
    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct Inputs {}

    pub(crate) type Outputs = String;
  }

  pub mod echo {
    use super::*;
    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct Inputs {
      #[serde(rename = "input")]
      pub(crate) input: String,
    }

    pub(crate) type Outputs = String;
  }

  pub mod reverse {
    use super::*;
    #[derive(serde::Deserialize, Debug)]
    pub(crate) struct Inputs {
      #[serde(rename = "input")]
      pub(crate) input: String,
    }

    pub(crate) type Outputs = String;
  }
}

pub(crate) fn init_imports() {}
pub(crate) fn init_exports() {
  wasmrs_guest::register_request_response("suite.test", "test", TestComponent::test_wrapper);

  wasmrs_guest::register_request_response("suite.test", "echo", TestComponent::echo_wrapper);

  wasmrs_guest::register_request_response("suite.test", "reverse", TestComponent::reverse_wrapper);
}

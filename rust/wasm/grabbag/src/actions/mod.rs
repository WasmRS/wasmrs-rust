/************************************************
 * THIS FILE IS GENERATED, DO NOT EDIT          *
 *                                              *
 * See https://apexlang.io for more information *
 ***********************************************/
pub(crate) mod my_streamer {
  pub(crate) mod request_channel_alias;
  pub(crate) mod request_channel_args_alias;
  pub(crate) mod request_channel_args_bool;
  pub(crate) mod request_channel_args_datetime;
  pub(crate) mod request_channel_args_enum;
  pub(crate) mod request_channel_args_f64;
  pub(crate) mod request_channel_args_i64;
  pub(crate) mod request_channel_args_list;
  pub(crate) mod request_channel_args_map;
  pub(crate) mod request_channel_args_string;
  pub(crate) mod request_channel_args_type;
  pub(crate) mod request_channel_bool;
  pub(crate) mod request_channel_datetime;
  pub(crate) mod request_channel_enum;
  pub(crate) mod request_channel_f64;
  pub(crate) mod request_channel_i64;
  pub(crate) mod request_channel_list;
  pub(crate) mod request_channel_map;
  pub(crate) mod request_channel_non_stream_output;
  pub(crate) mod request_channel_string;
  pub(crate) mod request_channel_type;
  pub(crate) mod request_channel_void;
  pub(crate) mod request_stream_alias;
  pub(crate) mod request_stream_args_alias;
  pub(crate) mod request_stream_args_bool;
  pub(crate) mod request_stream_args_datetime;
  pub(crate) mod request_stream_args_enum;
  pub(crate) mod request_stream_args_f64;
  pub(crate) mod request_stream_args_i64;
  pub(crate) mod request_stream_args_list;
  pub(crate) mod request_stream_args_map;
  pub(crate) mod request_stream_args_string;
  pub(crate) mod request_stream_args_type;
  pub(crate) mod request_stream_args_uuid;
  pub(crate) mod request_stream_bool;
  pub(crate) mod request_stream_datetime;
  pub(crate) mod request_stream_enum;
  pub(crate) mod request_stream_f64;
  pub(crate) mod request_stream_i64;
  pub(crate) mod request_stream_list;
  pub(crate) mod request_stream_map;
  pub(crate) mod request_stream_string;
  pub(crate) mod request_stream_type;
  pub(crate) mod request_stream_uuid;
}

pub(crate) mod my_service {
  pub(crate) mod empty_void;
  pub(crate) mod func_alias;
  pub(crate) mod func_bytes;
  pub(crate) mod func_datetime;
  pub(crate) mod func_enum;
  pub(crate) mod func_f32;
  pub(crate) mod func_f64;
  pub(crate) mod func_i16;
  pub(crate) mod func_i32;
  pub(crate) mod func_i64;
  pub(crate) mod func_i8;
  pub(crate) mod func_list;
  pub(crate) mod func_map;
  pub(crate) mod func_string;
  pub(crate) mod func_type;
  pub(crate) mod func_u16;
  pub(crate) mod func_u32;
  pub(crate) mod func_u64;
  pub(crate) mod func_u8;
  pub(crate) mod func_uuid;
  pub(crate) mod unary_alias;
  pub(crate) mod unary_bytes;
  pub(crate) mod unary_datetime;
  pub(crate) mod unary_enum;
  pub(crate) mod unary_f32;
  pub(crate) mod unary_f64;
  pub(crate) mod unary_i16;
  pub(crate) mod unary_i32;
  pub(crate) mod unary_i64;
  pub(crate) mod unary_i8;
  pub(crate) mod unary_list;
  pub(crate) mod unary_map;
  pub(crate) mod unary_string;
  pub(crate) mod unary_type;
  pub(crate) mod unary_u16;
  pub(crate) mod unary_u32;
  pub(crate) mod unary_u64;
  pub(crate) mod unary_u8;
  pub(crate) mod unary_uuid;
}

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

pub type Uuid = String;

pub type MyAlias = String;

pub(crate) struct MyStreamerComponent();

impl MyStreamerComponent {
  fn request_stream_i64_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_i64::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_i64(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_f64_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_f64::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_f64(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_type_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_type::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_type(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_enum_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_enum::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_enum(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_uuid_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_uuid::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_uuid(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_alias_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_alias::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_alias(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_string_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_string::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_string(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_bool_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_bool::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_bool(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_datetime_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_datetime::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_datetime(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_list_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_list::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_list(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_map_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        _map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_map::Input, Error> {
        unreachable!()
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_map(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_i64_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_i64::Input, Error> {
        Ok(my_streamer_service::request_stream_args_i64::Input {
          value: <i64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_i64(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_f64_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_f64::Input, Error> {
        Ok(my_streamer_service::request_stream_args_f64::Input {
          value: <f64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_f64(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_type_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_type::Input, Error> {
        Ok(my_streamer_service::request_stream_args_type::Input {
          value: <MyType as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_type(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_enum_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_enum::Input, Error> {
        Ok(my_streamer_service::request_stream_args_enum::Input {
          value: <MyEnum as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_enum(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_uuid_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_uuid::Input, Error> {
        Ok(my_streamer_service::request_stream_args_uuid::Input {
          value: <Uuid as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_uuid(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_alias_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_alias::Input, Error> {
        Ok(my_streamer_service::request_stream_args_alias::Input {
          value: <MyAlias as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_alias(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_string_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_string::Input, Error> {
        Ok(my_streamer_service::request_stream_args_string::Input {
          value: <String as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_string(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_bool_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_bool::Input, Error> {
        Ok(my_streamer_service::request_stream_args_bool::Input {
          value: <bool as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_bool(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_datetime_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_datetime::Input, Error> {
        Ok(my_streamer_service::request_stream_args_datetime::Input {
          value: <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_datetime(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_list_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_list::Input, Error> {
        Ok(my_streamer_service::request_stream_args_list::Input {
          value: <Vec<String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_list(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_stream_args_map_wrapper(input: IncomingMono) -> Result<OutgoingStream, GenericError> {
    let (out_tx, out_rx) = Flux::new_channels();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(e);
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_streamer_service::request_stream_args_map::Input, Error> {
        Ok(my_streamer_service::request_stream_args_map::Input {
          value: <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let input = match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
      };
      match MyStreamerComponent::request_stream_args_map(input).await {
        Ok(mut result) => {
          while let Some(next) = result.next().await {
            let out = match next {
              Ok(output) => serialize(&output)
                .map(|b| Payload::new_data(None, Some(b.into())))
                .map_err(|e| PayloadError::application_error(e.to_string())),
              Err(e) => Err(e),
            };
            let _ = out_tx.send_result(out);
          }
          out_tx.complete();
        }
        Err(e) => {
          let _ = out_tx.error(PayloadError::application_error(e.to_string()));
        }
      }
    });
    Ok(out_rx)
  }
  fn request_channel_i64_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_i64::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_i64::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <i64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <i64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_i64(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_f64_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_f64::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_f64::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <f64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <f64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_f64(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_type_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_type::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_type::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <MyType as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <MyType as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_type(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_enum_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_enum::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_enum::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <MyEnum as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <MyEnum as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_enum(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_alias_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_alias::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_alias::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <Uuid as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <Uuid as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_alias(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_string_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_string::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_string::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <String as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
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
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_string(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_bool_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_bool::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_bool::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <bool as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <bool as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_bool(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_datetime_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_datetime::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_datetime::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(v)
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_datetime(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_list_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_list::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_list::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <Vec<String> as serde::Deserialize>::deserialize(v)
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <Vec<String> as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_list(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_map_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_map::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_map::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(v)
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_map(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_i64_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_i64::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_i64::Input {
          value: <i64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <i64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <i64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_i64(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_f64_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_f64::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_f64::Input {
          value: <f64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <f64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <f64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_f64(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_type_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_type::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_type::Input {
          value: <MyType as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <MyType as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <MyType as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_type(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_enum_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_enum::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_enum::Input {
          value: <MyEnum as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <MyEnum as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <MyEnum as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_enum(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_alias_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_alias::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_alias::Input {
          value: <Uuid as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <Uuid as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <Uuid as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_alias(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_string_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des =
        move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_string::Input, Error> {
          let mut map = deserialize_generic(&payload.data)?;
          let input = my_streamer_service::request_channel_args_string::Input {
            value: <String as serde::Deserialize>::deserialize(
              map
                .remove("value")
                .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
            )
            .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
            r#in: real_in_rx,
          };

          if let Some(v) = map.remove("in") {
            let _ = in_inner_tx.send_result(
              <String as serde::Deserialize>::deserialize(v)
                .map_err(|e| PayloadError::application_error(e.to_string())),
            );
          }
          Ok(input)
        };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
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
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_string(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_bool_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_bool::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_bool::Input {
          value: <bool as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <bool as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <bool as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_bool(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_datetime_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des =
        move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_datetime::Input, Error> {
          let mut map = deserialize_generic(&payload.data)?;
          let input = my_streamer_service::request_channel_args_datetime::Input {
            value: <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(
              map
                .remove("value")
                .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
            )
            .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
            r#in: real_in_rx,
          };

          if let Some(v) = map.remove("in") {
            let _ = in_inner_tx.send_result(
              <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(v)
                .map_err(|e| PayloadError::application_error(e.to_string())),
            );
          }
          Ok(input)
        };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_datetime(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_list_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_list::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_list::Input {
          value: <Vec<String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <Vec<String> as serde::Deserialize>::deserialize(v)
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <Vec<String> as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_list(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_args_map_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_args_map::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_args_map::Input {
          value: <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          r#in: real_in_rx,
        };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(v)
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_args_map(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
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
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_void_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des = move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_void::Input, Error> {
        let mut map = deserialize_generic(&payload.data)?;
        let input = my_streamer_service::request_channel_void::Input { r#in: real_in_rx };

        if let Some(v) = map.remove("in") {
          let _ = in_inner_tx.send_result(
            <i64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
        Ok(input)
      };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <i64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_void(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
          let _ = real_out_tx.send_result(
            serialize(&result)
              .map(|b| Payload::new_data(None, Some(b.into())))
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
      }
    });
    Ok(real_out_rx)
  }
  fn request_channel_non_stream_output_wrapper(input: IncomingStream) -> Result<OutgoingStream, GenericError> {
    let (real_out_tx, real_out_rx) = Flux::new_channels();
    let (real_in_tx, real_in_rx) = Flux::new_channels();
    let in_inner_tx = real_in_tx.clone();
    spawn(async move {
      let des =
        move |payload: ParsedPayload| -> Result<my_streamer_service::request_channel_non_stream_output::Input, Error> {
          let mut map = deserialize_generic(&payload.data)?;
          let input = my_streamer_service::request_channel_non_stream_output::Input { r#in: real_in_rx };

          if let Some(v) = map.remove("in") {
            let _ = in_inner_tx.send_result(
              <i64 as serde::Deserialize>::deserialize(v).map_err(|e| PayloadError::application_error(e.to_string())),
            );
          }
          Ok(input)
        };
      let input_map = if let Ok(Some(Ok(first))) = input.recv().await {
        spawn(async move {
          while let Ok(Some(Ok(payload))) = input.recv().await {
            if let Ok(mut payload) = deserialize_generic(&payload.data) {
              if let Some(a) = payload.remove("in") {
                let _ = real_in_tx.send_result(
                  <i64 as serde::Deserialize>::deserialize(a)
                    .map_err(|e| PayloadError::application_error(e.to_string())),
                );
              }
            } else {
              break;
            }
          }
        });
        match des(first) {
          Ok(o) => o,
          Err(e) => {
            let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
            return;
          }
        }
      } else {
        return;
      };
      match MyStreamerComponent::request_channel_non_stream_output(input_map).await {
        Err(e) => {
          let _ = real_out_tx.error(PayloadError::application_error(e.to_string()));
          return;
        }
        Ok(mut result) => {
          let _ = real_out_tx.send_result(
            serialize(&result)
              .map(|b| Payload::new_data(None, Some(b.into())))
              .map_err(|e| PayloadError::application_error(e.to_string())),
          );
        }
      }
    });
    Ok(real_out_rx)
  }
}

#[async_trait::async_trait(?Send)]

pub(crate) trait MyStreamerService {
  async fn request_stream_i64(
    input: my_streamer_service::request_stream_i64::Input,
  ) -> Result<my_streamer_service::request_stream_i64::Output, GenericError>;

  async fn request_stream_f64(
    input: my_streamer_service::request_stream_f64::Input,
  ) -> Result<my_streamer_service::request_stream_f64::Output, GenericError>;

  async fn request_stream_type(
    input: my_streamer_service::request_stream_type::Input,
  ) -> Result<my_streamer_service::request_stream_type::Output, GenericError>;

  async fn request_stream_enum(
    input: my_streamer_service::request_stream_enum::Input,
  ) -> Result<my_streamer_service::request_stream_enum::Output, GenericError>;

  async fn request_stream_uuid(
    input: my_streamer_service::request_stream_uuid::Input,
  ) -> Result<my_streamer_service::request_stream_uuid::Output, GenericError>;

  async fn request_stream_alias(
    input: my_streamer_service::request_stream_alias::Input,
  ) -> Result<my_streamer_service::request_stream_alias::Output, GenericError>;

  async fn request_stream_string(
    input: my_streamer_service::request_stream_string::Input,
  ) -> Result<my_streamer_service::request_stream_string::Output, GenericError>;

  async fn request_stream_bool(
    input: my_streamer_service::request_stream_bool::Input,
  ) -> Result<my_streamer_service::request_stream_bool::Output, GenericError>;

  async fn request_stream_datetime(
    input: my_streamer_service::request_stream_datetime::Input,
  ) -> Result<my_streamer_service::request_stream_datetime::Output, GenericError>;

  async fn request_stream_list(
    input: my_streamer_service::request_stream_list::Input,
  ) -> Result<my_streamer_service::request_stream_list::Output, GenericError>;

  async fn request_stream_map(
    input: my_streamer_service::request_stream_map::Input,
  ) -> Result<my_streamer_service::request_stream_map::Output, GenericError>;

  async fn request_stream_args_i64(
    input: my_streamer_service::request_stream_args_i64::Input,
  ) -> Result<my_streamer_service::request_stream_args_i64::Output, GenericError>;

  async fn request_stream_args_f64(
    input: my_streamer_service::request_stream_args_f64::Input,
  ) -> Result<my_streamer_service::request_stream_args_f64::Output, GenericError>;

  async fn request_stream_args_type(
    input: my_streamer_service::request_stream_args_type::Input,
  ) -> Result<my_streamer_service::request_stream_args_type::Output, GenericError>;

  async fn request_stream_args_enum(
    input: my_streamer_service::request_stream_args_enum::Input,
  ) -> Result<my_streamer_service::request_stream_args_enum::Output, GenericError>;

  async fn request_stream_args_uuid(
    input: my_streamer_service::request_stream_args_uuid::Input,
  ) -> Result<my_streamer_service::request_stream_args_uuid::Output, GenericError>;

  async fn request_stream_args_alias(
    input: my_streamer_service::request_stream_args_alias::Input,
  ) -> Result<my_streamer_service::request_stream_args_alias::Output, GenericError>;

  async fn request_stream_args_string(
    input: my_streamer_service::request_stream_args_string::Input,
  ) -> Result<my_streamer_service::request_stream_args_string::Output, GenericError>;

  async fn request_stream_args_bool(
    input: my_streamer_service::request_stream_args_bool::Input,
  ) -> Result<my_streamer_service::request_stream_args_bool::Output, GenericError>;

  async fn request_stream_args_datetime(
    input: my_streamer_service::request_stream_args_datetime::Input,
  ) -> Result<my_streamer_service::request_stream_args_datetime::Output, GenericError>;

  async fn request_stream_args_list(
    input: my_streamer_service::request_stream_args_list::Input,
  ) -> Result<my_streamer_service::request_stream_args_list::Output, GenericError>;

  async fn request_stream_args_map(
    input: my_streamer_service::request_stream_args_map::Input,
  ) -> Result<my_streamer_service::request_stream_args_map::Output, GenericError>;

  async fn request_channel_i64(
    input: my_streamer_service::request_channel_i64::Input,
  ) -> Result<my_streamer_service::request_channel_i64::Output, GenericError>;

  async fn request_channel_f64(
    input: my_streamer_service::request_channel_f64::Input,
  ) -> Result<my_streamer_service::request_channel_f64::Output, GenericError>;

  async fn request_channel_type(
    input: my_streamer_service::request_channel_type::Input,
  ) -> Result<my_streamer_service::request_channel_type::Output, GenericError>;

  async fn request_channel_enum(
    input: my_streamer_service::request_channel_enum::Input,
  ) -> Result<my_streamer_service::request_channel_enum::Output, GenericError>;

  async fn request_channel_alias(
    input: my_streamer_service::request_channel_alias::Input,
  ) -> Result<my_streamer_service::request_channel_alias::Output, GenericError>;

  async fn request_channel_string(
    input: my_streamer_service::request_channel_string::Input,
  ) -> Result<my_streamer_service::request_channel_string::Output, GenericError>;

  async fn request_channel_bool(
    input: my_streamer_service::request_channel_bool::Input,
  ) -> Result<my_streamer_service::request_channel_bool::Output, GenericError>;

  async fn request_channel_datetime(
    input: my_streamer_service::request_channel_datetime::Input,
  ) -> Result<my_streamer_service::request_channel_datetime::Output, GenericError>;

  async fn request_channel_list(
    input: my_streamer_service::request_channel_list::Input,
  ) -> Result<my_streamer_service::request_channel_list::Output, GenericError>;

  async fn request_channel_map(
    input: my_streamer_service::request_channel_map::Input,
  ) -> Result<my_streamer_service::request_channel_map::Output, GenericError>;

  async fn request_channel_args_i64(
    input: my_streamer_service::request_channel_args_i64::Input,
  ) -> Result<my_streamer_service::request_channel_args_i64::Output, GenericError>;

  async fn request_channel_args_f64(
    input: my_streamer_service::request_channel_args_f64::Input,
  ) -> Result<my_streamer_service::request_channel_args_f64::Output, GenericError>;

  async fn request_channel_args_type(
    input: my_streamer_service::request_channel_args_type::Input,
  ) -> Result<my_streamer_service::request_channel_args_type::Output, GenericError>;

  async fn request_channel_args_enum(
    input: my_streamer_service::request_channel_args_enum::Input,
  ) -> Result<my_streamer_service::request_channel_args_enum::Output, GenericError>;

  async fn request_channel_args_alias(
    input: my_streamer_service::request_channel_args_alias::Input,
  ) -> Result<my_streamer_service::request_channel_args_alias::Output, GenericError>;

  async fn request_channel_args_string(
    input: my_streamer_service::request_channel_args_string::Input,
  ) -> Result<my_streamer_service::request_channel_args_string::Output, GenericError>;

  async fn request_channel_args_bool(
    input: my_streamer_service::request_channel_args_bool::Input,
  ) -> Result<my_streamer_service::request_channel_args_bool::Output, GenericError>;

  async fn request_channel_args_datetime(
    input: my_streamer_service::request_channel_args_datetime::Input,
  ) -> Result<my_streamer_service::request_channel_args_datetime::Output, GenericError>;

  async fn request_channel_args_list(
    input: my_streamer_service::request_channel_args_list::Input,
  ) -> Result<my_streamer_service::request_channel_args_list::Output, GenericError>;

  async fn request_channel_args_map(
    input: my_streamer_service::request_channel_args_map::Input,
  ) -> Result<my_streamer_service::request_channel_args_map::Output, GenericError>;

  async fn request_channel_void(
    input: my_streamer_service::request_channel_void::Input,
  ) -> Result<my_streamer_service::request_channel_void::Output, GenericError>;

  async fn request_channel_non_stream_output(
    input: my_streamer_service::request_channel_non_stream_output::Input,
  ) -> Result<my_streamer_service::request_channel_non_stream_output::Output, GenericError>;
}

#[async_trait::async_trait(?Send)]
impl MyStreamerService for MyStreamerComponent {
  async fn request_stream_i64(
    input: my_streamer_service::request_stream_i64::Input,
  ) -> Result<my_streamer_service::request_stream_i64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_i64::task(input).await?)
  }

  async fn request_stream_f64(
    input: my_streamer_service::request_stream_f64::Input,
  ) -> Result<my_streamer_service::request_stream_f64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_f64::task(input).await?)
  }

  async fn request_stream_type(
    input: my_streamer_service::request_stream_type::Input,
  ) -> Result<my_streamer_service::request_stream_type::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_type::task(input).await?)
  }

  async fn request_stream_enum(
    input: my_streamer_service::request_stream_enum::Input,
  ) -> Result<my_streamer_service::request_stream_enum::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_enum::task(input).await?)
  }

  async fn request_stream_uuid(
    input: my_streamer_service::request_stream_uuid::Input,
  ) -> Result<my_streamer_service::request_stream_uuid::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_uuid::task(input).await?)
  }

  async fn request_stream_alias(
    input: my_streamer_service::request_stream_alias::Input,
  ) -> Result<my_streamer_service::request_stream_alias::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_alias::task(input).await?)
  }

  async fn request_stream_string(
    input: my_streamer_service::request_stream_string::Input,
  ) -> Result<my_streamer_service::request_stream_string::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_string::task(input).await?)
  }

  async fn request_stream_bool(
    input: my_streamer_service::request_stream_bool::Input,
  ) -> Result<my_streamer_service::request_stream_bool::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_bool::task(input).await?)
  }

  async fn request_stream_datetime(
    input: my_streamer_service::request_stream_datetime::Input,
  ) -> Result<my_streamer_service::request_stream_datetime::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_datetime::task(input).await?)
  }

  async fn request_stream_list(
    input: my_streamer_service::request_stream_list::Input,
  ) -> Result<my_streamer_service::request_stream_list::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_list::task(input).await?)
  }

  async fn request_stream_map(
    input: my_streamer_service::request_stream_map::Input,
  ) -> Result<my_streamer_service::request_stream_map::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_map::task(input).await?)
  }

  async fn request_stream_args_i64(
    input: my_streamer_service::request_stream_args_i64::Input,
  ) -> Result<my_streamer_service::request_stream_args_i64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_i64::task(input).await?)
  }

  async fn request_stream_args_f64(
    input: my_streamer_service::request_stream_args_f64::Input,
  ) -> Result<my_streamer_service::request_stream_args_f64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_f64::task(input).await?)
  }

  async fn request_stream_args_type(
    input: my_streamer_service::request_stream_args_type::Input,
  ) -> Result<my_streamer_service::request_stream_args_type::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_type::task(input).await?)
  }

  async fn request_stream_args_enum(
    input: my_streamer_service::request_stream_args_enum::Input,
  ) -> Result<my_streamer_service::request_stream_args_enum::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_enum::task(input).await?)
  }

  async fn request_stream_args_uuid(
    input: my_streamer_service::request_stream_args_uuid::Input,
  ) -> Result<my_streamer_service::request_stream_args_uuid::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_uuid::task(input).await?)
  }

  async fn request_stream_args_alias(
    input: my_streamer_service::request_stream_args_alias::Input,
  ) -> Result<my_streamer_service::request_stream_args_alias::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_alias::task(input).await?)
  }

  async fn request_stream_args_string(
    input: my_streamer_service::request_stream_args_string::Input,
  ) -> Result<my_streamer_service::request_stream_args_string::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_string::task(input).await?)
  }

  async fn request_stream_args_bool(
    input: my_streamer_service::request_stream_args_bool::Input,
  ) -> Result<my_streamer_service::request_stream_args_bool::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_bool::task(input).await?)
  }

  async fn request_stream_args_datetime(
    input: my_streamer_service::request_stream_args_datetime::Input,
  ) -> Result<my_streamer_service::request_stream_args_datetime::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_datetime::task(input).await?)
  }

  async fn request_stream_args_list(
    input: my_streamer_service::request_stream_args_list::Input,
  ) -> Result<my_streamer_service::request_stream_args_list::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_list::task(input).await?)
  }

  async fn request_stream_args_map(
    input: my_streamer_service::request_stream_args_map::Input,
  ) -> Result<my_streamer_service::request_stream_args_map::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_stream_args_map::task(input).await?)
  }

  async fn request_channel_i64(
    input: my_streamer_service::request_channel_i64::Input,
  ) -> Result<my_streamer_service::request_channel_i64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_i64::task(input).await?)
  }

  async fn request_channel_f64(
    input: my_streamer_service::request_channel_f64::Input,
  ) -> Result<my_streamer_service::request_channel_f64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_f64::task(input).await?)
  }

  async fn request_channel_type(
    input: my_streamer_service::request_channel_type::Input,
  ) -> Result<my_streamer_service::request_channel_type::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_type::task(input).await?)
  }

  async fn request_channel_enum(
    input: my_streamer_service::request_channel_enum::Input,
  ) -> Result<my_streamer_service::request_channel_enum::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_enum::task(input).await?)
  }

  async fn request_channel_alias(
    input: my_streamer_service::request_channel_alias::Input,
  ) -> Result<my_streamer_service::request_channel_alias::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_alias::task(input).await?)
  }

  async fn request_channel_string(
    input: my_streamer_service::request_channel_string::Input,
  ) -> Result<my_streamer_service::request_channel_string::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_string::task(input).await?)
  }

  async fn request_channel_bool(
    input: my_streamer_service::request_channel_bool::Input,
  ) -> Result<my_streamer_service::request_channel_bool::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_bool::task(input).await?)
  }

  async fn request_channel_datetime(
    input: my_streamer_service::request_channel_datetime::Input,
  ) -> Result<my_streamer_service::request_channel_datetime::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_datetime::task(input).await?)
  }

  async fn request_channel_list(
    input: my_streamer_service::request_channel_list::Input,
  ) -> Result<my_streamer_service::request_channel_list::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_list::task(input).await?)
  }

  async fn request_channel_map(
    input: my_streamer_service::request_channel_map::Input,
  ) -> Result<my_streamer_service::request_channel_map::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_map::task(input).await?)
  }

  async fn request_channel_args_i64(
    input: my_streamer_service::request_channel_args_i64::Input,
  ) -> Result<my_streamer_service::request_channel_args_i64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_i64::task(input).await?)
  }

  async fn request_channel_args_f64(
    input: my_streamer_service::request_channel_args_f64::Input,
  ) -> Result<my_streamer_service::request_channel_args_f64::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_f64::task(input).await?)
  }

  async fn request_channel_args_type(
    input: my_streamer_service::request_channel_args_type::Input,
  ) -> Result<my_streamer_service::request_channel_args_type::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_type::task(input).await?)
  }

  async fn request_channel_args_enum(
    input: my_streamer_service::request_channel_args_enum::Input,
  ) -> Result<my_streamer_service::request_channel_args_enum::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_enum::task(input).await?)
  }

  async fn request_channel_args_alias(
    input: my_streamer_service::request_channel_args_alias::Input,
  ) -> Result<my_streamer_service::request_channel_args_alias::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_alias::task(input).await?)
  }

  async fn request_channel_args_string(
    input: my_streamer_service::request_channel_args_string::Input,
  ) -> Result<my_streamer_service::request_channel_args_string::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_string::task(input).await?)
  }

  async fn request_channel_args_bool(
    input: my_streamer_service::request_channel_args_bool::Input,
  ) -> Result<my_streamer_service::request_channel_args_bool::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_bool::task(input).await?)
  }

  async fn request_channel_args_datetime(
    input: my_streamer_service::request_channel_args_datetime::Input,
  ) -> Result<my_streamer_service::request_channel_args_datetime::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_datetime::task(input).await?)
  }

  async fn request_channel_args_list(
    input: my_streamer_service::request_channel_args_list::Input,
  ) -> Result<my_streamer_service::request_channel_args_list::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_list::task(input).await?)
  }

  async fn request_channel_args_map(
    input: my_streamer_service::request_channel_args_map::Input,
  ) -> Result<my_streamer_service::request_channel_args_map::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_args_map::task(input).await?)
  }

  async fn request_channel_void(
    input: my_streamer_service::request_channel_void::Input,
  ) -> Result<my_streamer_service::request_channel_void::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_void::task(input).await?)
  }

  async fn request_channel_non_stream_output(
    input: my_streamer_service::request_channel_non_stream_output::Input,
  ) -> Result<my_streamer_service::request_channel_non_stream_output::Output, GenericError> {
    Ok(crate::actions::my_streamer::request_channel_non_stream_output::task(input).await?)
  }
}

pub mod my_streamer_service {
  #[allow(unused_imports)]
  pub(crate) use super::*;

  pub mod request_stream_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<i64, PayloadError>;
  }

  pub mod request_stream_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<f64, PayloadError>;
  }

  pub mod request_stream_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<MyType, PayloadError>;
  }

  pub mod request_stream_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<MyEnum, PayloadError>;
  }

  pub mod request_stream_uuid {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<Uuid, PayloadError>;
  }

  pub mod request_stream_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<MyAlias, PayloadError>;
  }

  pub mod request_stream_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<String, PayloadError>;
  }

  pub mod request_stream_bool {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<bool, PayloadError>;
  }

  pub mod request_stream_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<wasmrs_guest::Timestamp, PayloadError>;
  }

  pub mod request_stream_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<Vec<String>, PayloadError>;
  }

  pub mod request_stream_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = FluxReceiver<std::collections::HashMap<String, String>, PayloadError>;
  }

  pub mod request_stream_args_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i64,
    }

    pub(crate) type Output = FluxReceiver<i64, PayloadError>;
  }

  pub mod request_stream_args_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: f64,
    }

    pub(crate) type Output = FluxReceiver<f64, PayloadError>;
  }

  pub mod request_stream_args_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyType,
    }

    pub(crate) type Output = FluxReceiver<MyType, PayloadError>;
  }

  pub mod request_stream_args_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyEnum,
    }

    pub(crate) type Output = FluxReceiver<MyEnum, PayloadError>;
  }

  pub mod request_stream_args_uuid {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Uuid,
    }

    pub(crate) type Output = FluxReceiver<Uuid, PayloadError>;
  }

  pub mod request_stream_args_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyAlias,
    }

    pub(crate) type Output = FluxReceiver<MyAlias, PayloadError>;
  }

  pub mod request_stream_args_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: String,
    }

    pub(crate) type Output = FluxReceiver<String, PayloadError>;
  }

  pub mod request_stream_args_bool {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: bool,
    }

    pub(crate) type Output = FluxReceiver<bool, PayloadError>;
  }

  pub mod request_stream_args_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: wasmrs_guest::Timestamp,
    }

    pub(crate) type Output = FluxReceiver<wasmrs_guest::Timestamp, PayloadError>;
  }

  pub mod request_stream_args_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Vec<String>,
    }

    pub(crate) type Output = FluxReceiver<Vec<String>, PayloadError>;
  }

  pub mod request_stream_args_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: std::collections::HashMap<String, String>,
    }

    pub(crate) type Output = FluxReceiver<std::collections::HashMap<String, String>, PayloadError>;
  }

  pub mod request_channel_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<i64, PayloadError>;
  }

  pub mod request_channel_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<f64, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<f64, PayloadError>;
  }

  pub mod request_channel_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<MyType, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<MyType, PayloadError>;
  }

  pub mod request_channel_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<MyEnum, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<MyEnum, PayloadError>;
  }

  pub mod request_channel_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<Uuid, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<Uuid, PayloadError>;
  }

  pub mod request_channel_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<String, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<String, PayloadError>;
  }

  pub mod request_channel_bool {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<bool, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<bool, PayloadError>;
  }

  pub mod request_channel_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<wasmrs_guest::Timestamp, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<wasmrs_guest::Timestamp, PayloadError>;
  }

  pub mod request_channel_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<Vec<String>, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<Vec<String>, PayloadError>;
  }

  pub mod request_channel_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<std::collections::HashMap<String, String>, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<std::collections::HashMap<String, String>, PayloadError>;
  }

  pub mod request_channel_args_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i64,

      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<i64, PayloadError>;
  }

  pub mod request_channel_args_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: f64,

      pub(crate) r#in: FluxReceiver<f64, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<f64, PayloadError>;
  }

  pub mod request_channel_args_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyType,

      pub(crate) r#in: FluxReceiver<MyType, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<MyType, PayloadError>;
  }

  pub mod request_channel_args_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyEnum,

      pub(crate) r#in: FluxReceiver<MyEnum, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<MyEnum, PayloadError>;
  }

  pub mod request_channel_args_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Uuid,

      pub(crate) r#in: FluxReceiver<Uuid, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<Uuid, PayloadError>;
  }

  pub mod request_channel_args_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: String,

      pub(crate) r#in: FluxReceiver<String, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<String, PayloadError>;
  }

  pub mod request_channel_args_bool {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: bool,

      pub(crate) r#in: FluxReceiver<bool, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<bool, PayloadError>;
  }

  pub mod request_channel_args_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: wasmrs_guest::Timestamp,

      pub(crate) r#in: FluxReceiver<wasmrs_guest::Timestamp, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<wasmrs_guest::Timestamp, PayloadError>;
  }

  pub mod request_channel_args_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Vec<String>,

      pub(crate) r#in: FluxReceiver<Vec<String>, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<Vec<String>, PayloadError>;
  }

  pub mod request_channel_args_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: std::collections::HashMap<String, String>,

      pub(crate) r#in: FluxReceiver<std::collections::HashMap<String, String>, PayloadError>,
    }

    pub(crate) type Output = FluxReceiver<std::collections::HashMap<String, String>, PayloadError>;
  }

  pub mod request_channel_void {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    pub(crate) type Output = ();
  }

  pub mod request_channel_non_stream_output {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    pub(crate) type Output = String;
  }
}

static MY_PROVIDER_REQUEST_STREAM_I64_INDEX_BYTES: [u8; 4] = 0u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_F64_INDEX_BYTES: [u8; 4] = 1u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_TYPE_INDEX_BYTES: [u8; 4] = 2u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ENUM_INDEX_BYTES: [u8; 4] = 3u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_UUID_INDEX_BYTES: [u8; 4] = 4u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ALIAS_INDEX_BYTES: [u8; 4] = 5u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_STRING_INDEX_BYTES: [u8; 4] = 6u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_BOOL_INDEX_BYTES: [u8; 4] = 7u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_DATETIME_INDEX_BYTES: [u8; 4] = 8u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_LIST_INDEX_BYTES: [u8; 4] = 9u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_MAP_INDEX_BYTES: [u8; 4] = 10u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_I64_INDEX_BYTES: [u8; 4] = 11u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_F64_INDEX_BYTES: [u8; 4] = 12u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_TYPE_INDEX_BYTES: [u8; 4] = 13u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_ENUM_INDEX_BYTES: [u8; 4] = 14u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_UUID_INDEX_BYTES: [u8; 4] = 15u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_ALIAS_INDEX_BYTES: [u8; 4] = 16u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_STRING_INDEX_BYTES: [u8; 4] = 17u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_BOOL_INDEX_BYTES: [u8; 4] = 18u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_DATETIME_INDEX_BYTES: [u8; 4] = 19u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_LIST_INDEX_BYTES: [u8; 4] = 20u32.to_be_bytes();
static MY_PROVIDER_REQUEST_STREAM_ARGS_MAP_INDEX_BYTES: [u8; 4] = 21u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_I64_INDEX_BYTES: [u8; 4] = 22u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_F64_INDEX_BYTES: [u8; 4] = 23u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_TYPE_INDEX_BYTES: [u8; 4] = 24u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ENUM_INDEX_BYTES: [u8; 4] = 25u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ALIAS_INDEX_BYTES: [u8; 4] = 26u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_STRING_INDEX_BYTES: [u8; 4] = 27u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_BOOL_INDEX_BYTES: [u8; 4] = 28u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_DATETIME_INDEX_BYTES: [u8; 4] = 29u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_LIST_INDEX_BYTES: [u8; 4] = 30u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_MAP_INDEX_BYTES: [u8; 4] = 31u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_I64_INDEX_BYTES: [u8; 4] = 32u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_F64_INDEX_BYTES: [u8; 4] = 33u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_TYPE_INDEX_BYTES: [u8; 4] = 34u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_ENUM_INDEX_BYTES: [u8; 4] = 35u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_ALIAS_INDEX_BYTES: [u8; 4] = 36u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_STRING_INDEX_BYTES: [u8; 4] = 37u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_BOOL_INDEX_BYTES: [u8; 4] = 38u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_DATETIME_INDEX_BYTES: [u8; 4] = 39u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_LIST_INDEX_BYTES: [u8; 4] = 40u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_ARGS_MAP_INDEX_BYTES: [u8; 4] = 41u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_VOID_INDEX_BYTES: [u8; 4] = 42u32.to_be_bytes();
static MY_PROVIDER_REQUEST_CHANNEL_NON_STREAM_OUTPUT_INDEX_BYTES: [u8; 4] = 43u32.to_be_bytes();

pub mod my_provider {
  use super::*;

  pub(crate) fn request_stream_i64(
    input: request_stream_i64::Input,
  ) -> impl Stream<Item = Result<request_stream_i64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_I64_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_i64::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_i64 {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = i64;
  }

  pub(crate) fn request_stream_f64(
    input: request_stream_f64::Input,
  ) -> impl Stream<Item = Result<request_stream_f64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_F64_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_f64::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_f64 {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = f64;
  }

  pub(crate) fn request_stream_type(
    input: request_stream_type::Input,
  ) -> impl Stream<Item = Result<request_stream_type::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_TYPE_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_type::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_type {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = MyType;
  }

  pub(crate) fn request_stream_enum(
    input: request_stream_enum::Input,
  ) -> impl Stream<Item = Result<request_stream_enum::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ENUM_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_enum::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_enum {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = MyEnum;
  }

  pub(crate) fn request_stream_uuid(
    input: request_stream_uuid::Input,
  ) -> impl Stream<Item = Result<request_stream_uuid::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_UUID_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_uuid::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_uuid {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = Uuid;
  }

  pub(crate) fn request_stream_alias(
    input: request_stream_alias::Input,
  ) -> impl Stream<Item = Result<request_stream_alias::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ALIAS_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_alias::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_alias {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = MyAlias;
  }

  pub(crate) fn request_stream_string(
    input: request_stream_string::Input,
  ) -> impl Stream<Item = Result<request_stream_string::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_STRING_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_string::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_string {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = String;
  }

  pub(crate) fn request_stream_bool(
    input: request_stream_bool::Input,
  ) -> impl Stream<Item = Result<request_stream_bool::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_BOOL_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_bool::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_bool {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = bool;
  }

  pub(crate) fn request_stream_datetime(
    input: request_stream_datetime::Input,
  ) -> impl Stream<Item = Result<request_stream_datetime::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_DATETIME_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_datetime::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_datetime {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub(crate) fn request_stream_list(
    input: request_stream_list::Input,
  ) -> impl Stream<Item = Result<request_stream_list::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_LIST_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_list::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_list {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = Vec<String>;
  }

  pub(crate) fn request_stream_map(
    input: request_stream_map::Input,
  ) -> impl Stream<Item = Result<request_stream_map::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_MAP_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default()
      .request_stream(payload)
      .map(|result| result.map(|payload| Ok(deserialize::<request_stream_map::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_stream_map {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {}

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }

  pub(crate) fn request_stream_args_i64(
    input: request_stream_args_i64::Input,
  ) -> impl Stream<Item = Result<request_stream_args_i64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_I64_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_i64::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_i64 {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: i64,
    }

    pub(crate) type Output = i64;
  }

  pub(crate) fn request_stream_args_f64(
    input: request_stream_args_f64::Input,
  ) -> impl Stream<Item = Result<request_stream_args_f64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_F64_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_f64::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_f64 {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: f64,
    }

    pub(crate) type Output = f64;
  }

  pub(crate) fn request_stream_args_type(
    input: request_stream_args_type::Input,
  ) -> impl Stream<Item = Result<request_stream_args_type::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_TYPE_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_type::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_type {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: MyType,
    }

    pub(crate) type Output = MyType;
  }

  pub(crate) fn request_stream_args_enum(
    input: request_stream_args_enum::Input,
  ) -> impl Stream<Item = Result<request_stream_args_enum::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_ENUM_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_enum::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_enum {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: MyEnum,
    }

    pub(crate) type Output = MyEnum;
  }

  pub(crate) fn request_stream_args_uuid(
    input: request_stream_args_uuid::Input,
  ) -> impl Stream<Item = Result<request_stream_args_uuid::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_UUID_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_uuid::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_uuid {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: Uuid,
    }

    pub(crate) type Output = Uuid;
  }

  pub(crate) fn request_stream_args_alias(
    input: request_stream_args_alias::Input,
  ) -> impl Stream<Item = Result<request_stream_args_alias::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_ALIAS_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_stream_args_alias::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_stream_args_alias {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: MyAlias,
    }

    pub(crate) type Output = MyAlias;
  }

  pub(crate) fn request_stream_args_string(
    input: request_stream_args_string::Input,
  ) -> impl Stream<Item = Result<request_stream_args_string::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_STRING_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_stream_args_string::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_stream_args_string {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: String,
    }

    pub(crate) type Output = String;
  }

  pub(crate) fn request_stream_args_bool(
    input: request_stream_args_bool::Input,
  ) -> impl Stream<Item = Result<request_stream_args_bool::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_BOOL_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_bool::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_bool {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: bool,
    }

    pub(crate) type Output = bool;
  }

  pub(crate) fn request_stream_args_datetime(
    input: request_stream_args_datetime::Input,
  ) -> impl Stream<Item = Result<request_stream_args_datetime::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_DATETIME_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_stream_args_datetime::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_stream_args_datetime {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: wasmrs_guest::Timestamp,
    }

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub(crate) fn request_stream_args_list(
    input: request_stream_args_list::Input,
  ) -> impl Stream<Item = Result<request_stream_args_list::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_LIST_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_list::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_list {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: Vec<String>,
    }

    pub(crate) type Output = Vec<String>;
  }

  pub(crate) fn request_stream_args_map(
    input: request_stream_args_map::Input,
  ) -> impl Stream<Item = Result<request_stream_args_map::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_STREAM_ARGS_MAP_INDEX_BYTES.as_slice();
    let payload = wasmrs_guest::serialize(&input)
      .map(|bytes| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), bytes.into()))
      .unwrap();
    Host::default().request_stream(payload).map(|result| {
      result.map(|payload| Ok(deserialize::<request_stream_args_map::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_stream_args_map {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct Input {
      pub(crate) value: std::collections::HashMap<String, String>,
    }

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }

  pub(crate) fn request_channel_i64(
    mut input: request_channel_i64::Input,
  ) -> impl Stream<Item = Result<request_channel_i64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_I64_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_i64::InputFirst),
      In(i64),
    }
    let first = OpInputs::Params(request_channel_i64::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_i64::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_i64 {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = i64;
  }

  pub(crate) fn request_channel_f64(
    mut input: request_channel_f64::Input,
  ) -> impl Stream<Item = Result<request_channel_f64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_F64_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_f64::InputFirst),
      In(f64),
    }
    let first = OpInputs::Params(request_channel_f64::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_f64::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_f64 {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<f64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = f64;
  }

  pub(crate) fn request_channel_type(
    mut input: request_channel_type::Input,
  ) -> impl Stream<Item = Result<request_channel_type::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_TYPE_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_type::InputFirst),
      In(MyType),
    }
    let first = OpInputs::Params(request_channel_type::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_type::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_type {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<MyType, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = MyType;
  }

  pub(crate) fn request_channel_enum(
    mut input: request_channel_enum::Input,
  ) -> impl Stream<Item = Result<request_channel_enum::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ENUM_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_enum::InputFirst),
      In(MyEnum),
    }
    let first = OpInputs::Params(request_channel_enum::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_enum::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_enum {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<MyEnum, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = MyEnum;
  }

  pub(crate) fn request_channel_alias(
    mut input: request_channel_alias::Input,
  ) -> impl Stream<Item = Result<request_channel_alias::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ALIAS_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_alias::InputFirst),
      In(Uuid),
    }
    let first = OpInputs::Params(request_channel_alias::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_alias::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_alias {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<Uuid, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = Uuid;
  }

  pub(crate) fn request_channel_string(
    mut input: request_channel_string::Input,
  ) -> impl Stream<Item = Result<request_channel_string::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_STRING_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_string::InputFirst),
      In(String),
    }
    let first = OpInputs::Params(request_channel_string::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_string::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_string {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<String, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = String;
  }

  pub(crate) fn request_channel_bool(
    mut input: request_channel_bool::Input,
  ) -> impl Stream<Item = Result<request_channel_bool::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_BOOL_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_bool::InputFirst),
      In(bool),
    }
    let first = OpInputs::Params(request_channel_bool::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_bool::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_bool {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<bool, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = bool;
  }

  pub(crate) fn request_channel_datetime(
    mut input: request_channel_datetime::Input,
  ) -> impl Stream<Item = Result<request_channel_datetime::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_DATETIME_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_datetime::InputFirst),
      In(wasmrs_guest::Timestamp),
    }
    let first = OpInputs::Params(request_channel_datetime::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| Ok(deserialize::<request_channel_datetime::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_channel_datetime {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<wasmrs_guest::Timestamp, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub(crate) fn request_channel_list(
    mut input: request_channel_list::Input,
  ) -> impl Stream<Item = Result<request_channel_list::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_LIST_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_list::InputFirst),
      In(Vec<String>),
    }
    let first = OpInputs::Params(request_channel_list::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_list::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_list {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<Vec<String>, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = Vec<String>;
  }

  pub(crate) fn request_channel_map(
    mut input: request_channel_map::Input,
  ) -> impl Stream<Item = Result<request_channel_map::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_MAP_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_map::InputFirst),
      In(std::collections::HashMap<String, String>),
    }
    let first = OpInputs::Params(request_channel_map::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_map::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_map {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<std::collections::HashMap<String, String>, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }

  pub(crate) fn request_channel_args_i64(
    mut input: request_channel_args_i64::Input,
  ) -> impl Stream<Item = Result<request_channel_args_i64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_I64_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_i64::InputFirst),
      In(i64),
    }
    let first = OpInputs::Params(request_channel_args_i64::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| Ok(deserialize::<request_channel_args_i64::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_channel_args_i64 {
    use super::*;

    pub struct Input {
      pub(crate) value: i64,
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: i64,
    }

    pub(crate) type Output = i64;
  }

  pub(crate) fn request_channel_args_f64(
    mut input: request_channel_args_f64::Input,
  ) -> impl Stream<Item = Result<request_channel_args_f64::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_F64_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_f64::InputFirst),
      In(f64),
    }
    let first = OpInputs::Params(request_channel_args_f64::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| Ok(deserialize::<request_channel_args_f64::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_channel_args_f64 {
    use super::*;

    pub struct Input {
      pub(crate) value: f64,
      pub(crate) r#in: FluxReceiver<f64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: f64,
    }

    pub(crate) type Output = f64;
  }

  pub(crate) fn request_channel_args_type(
    mut input: request_channel_args_type::Input,
  ) -> impl Stream<Item = Result<request_channel_args_type::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_TYPE_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_type::InputFirst),
      In(MyType),
    }
    let first = OpInputs::Params(request_channel_args_type::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_type::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_type {
    use super::*;

    pub struct Input {
      pub(crate) value: MyType,
      pub(crate) r#in: FluxReceiver<MyType, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: MyType,
    }

    pub(crate) type Output = MyType;
  }

  pub(crate) fn request_channel_args_enum(
    mut input: request_channel_args_enum::Input,
  ) -> impl Stream<Item = Result<request_channel_args_enum::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_ENUM_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_enum::InputFirst),
      In(MyEnum),
    }
    let first = OpInputs::Params(request_channel_args_enum::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_enum::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_enum {
    use super::*;

    pub struct Input {
      pub(crate) value: MyEnum,
      pub(crate) r#in: FluxReceiver<MyEnum, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: MyEnum,
    }

    pub(crate) type Output = MyEnum;
  }

  pub(crate) fn request_channel_args_alias(
    mut input: request_channel_args_alias::Input,
  ) -> impl Stream<Item = Result<request_channel_args_alias::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_ALIAS_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_alias::InputFirst),
      In(Uuid),
    }
    let first = OpInputs::Params(request_channel_args_alias::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_alias::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_alias {
    use super::*;

    pub struct Input {
      pub(crate) value: Uuid,
      pub(crate) r#in: FluxReceiver<Uuid, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: Uuid,
    }

    pub(crate) type Output = Uuid;
  }

  pub(crate) fn request_channel_args_string(
    mut input: request_channel_args_string::Input,
  ) -> impl Stream<Item = Result<request_channel_args_string::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_STRING_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_string::InputFirst),
      In(String),
    }
    let first = OpInputs::Params(request_channel_args_string::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_string::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_string {
    use super::*;

    pub struct Input {
      pub(crate) value: String,
      pub(crate) r#in: FluxReceiver<String, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: String,
    }

    pub(crate) type Output = String;
  }

  pub(crate) fn request_channel_args_bool(
    mut input: request_channel_args_bool::Input,
  ) -> impl Stream<Item = Result<request_channel_args_bool::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_BOOL_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_bool::InputFirst),
      In(bool),
    }
    let first = OpInputs::Params(request_channel_args_bool::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_bool::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_bool {
    use super::*;

    pub struct Input {
      pub(crate) value: bool,
      pub(crate) r#in: FluxReceiver<bool, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: bool,
    }

    pub(crate) type Output = bool;
  }

  pub(crate) fn request_channel_args_datetime(
    mut input: request_channel_args_datetime::Input,
  ) -> impl Stream<Item = Result<request_channel_args_datetime::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_DATETIME_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_datetime::InputFirst),
      In(wasmrs_guest::Timestamp),
    }
    let first = OpInputs::Params(request_channel_args_datetime::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_datetime::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_datetime {
    use super::*;

    pub struct Input {
      pub(crate) value: wasmrs_guest::Timestamp,
      pub(crate) r#in: FluxReceiver<wasmrs_guest::Timestamp, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: wasmrs_guest::Timestamp,
    }

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub(crate) fn request_channel_args_list(
    mut input: request_channel_args_list::Input,
  ) -> impl Stream<Item = Result<request_channel_args_list::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_LIST_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_list::InputFirst),
      In(Vec<String>),
    }
    let first = OpInputs::Params(request_channel_args_list::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_args_list::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_args_list {
    use super::*;

    pub struct Input {
      pub(crate) value: Vec<String>,
      pub(crate) r#in: FluxReceiver<Vec<String>, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: Vec<String>,
    }

    pub(crate) type Output = Vec<String>;
  }

  pub(crate) fn request_channel_args_map(
    mut input: request_channel_args_map::Input,
  ) -> impl Stream<Item = Result<request_channel_args_map::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_ARGS_MAP_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_args_map::InputFirst),
      In(std::collections::HashMap<String, String>),
    }
    let first = OpInputs::Params(request_channel_args_map::InputFirst { value: input.value });

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| Ok(deserialize::<request_channel_args_map::Output>(&payload.data.unwrap())?))?
    })
  }

  pub(crate) mod request_channel_args_map {
    use super::*;

    pub struct Input {
      pub(crate) value: std::collections::HashMap<String, String>,
      pub(crate) r#in: FluxReceiver<std::collections::HashMap<String, String>, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {
      pub(crate) value: std::collections::HashMap<String, String>,
    }

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }

  pub(crate) fn request_channel_void(
    mut input: request_channel_void::Input,
  ) -> impl Stream<Item = Result<request_channel_void::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_VOID_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_void::InputFirst),
      In(i64),
    }
    let first = OpInputs::Params(request_channel_void::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default()
      .request_channel(rx)
      .map(|result| result.map(|payload| Ok(deserialize::<request_channel_void::Output>(&payload.data.unwrap())?))?)
  }

  pub(crate) mod request_channel_void {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = ();
  }

  pub(crate) fn request_channel_non_stream_output(
    mut input: request_channel_non_stream_output::Input,
  ) -> impl Stream<Item = Result<request_channel_non_stream_output::Output, PayloadError>> {
    let op_id_bytes = MY_PROVIDER_REQUEST_CHANNEL_NON_STREAM_OUTPUT_INDEX_BYTES.as_slice();
    let (tx, rx) = Flux::new_channels();
    #[derive(serde::Serialize, serde::Deserialize)]
    #[serde(untagged)]
    enum OpInputs {
      Params(request_channel_non_stream_output::InputFirst),
      In(i64),
    }
    let first = OpInputs::Params(request_channel_non_stream_output::InputFirst {});

    let tx_inner = tx.clone();
    spawn(async move {
      while let Some(payload) = input.r#in.next().await {
        let payload = match payload {
          Ok(o) => o,
          Err(e) => {
            let _ = tx_inner.error(e);
            continue;
          }
        };
        let message = OpInputs::In(payload);
        let payload = wasmrs_guest::serialize(&message)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()));
        let _ = tx_inner.send_result(payload);
      }
    });

    let payload = wasmrs_guest::serialize(&first)
      .map(|b| Payload::new([op_id_bytes, &[0, 0, 0, 0]].concat().into(), b.into()))
      .map_err(|e| PayloadError::application_error(e.to_string()));
    let _ = tx.send_result(payload);

    Host::default().request_channel(rx).map(|result| {
      result.map(|payload| {
        Ok(deserialize::<request_channel_non_stream_output::Output>(
          &payload.data.unwrap(),
        )?)
      })?
    })
  }

  pub(crate) mod request_channel_non_stream_output {
    use super::*;

    pub struct Input {
      pub(crate) r#in: FluxReceiver<i64, PayloadError>,
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    pub struct InputFirst {}

    pub(crate) type Output = String;
  }
}

pub(crate) struct MyServiceComponent();

impl MyServiceComponent {
  fn empty_void_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(_map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::empty_void::Input, Error> {
        unreachable!()
      }
      let _ = MyServiceComponent::empty_void(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_type_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<MyType, Error> {
        Ok(
          <MyType as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_type(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_enum_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<MyEnum, Error> {
        Ok(
          <MyEnum as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_enum(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_uuid_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<Uuid, Error> {
        Ok(
          <Uuid as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_uuid(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_alias_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<MyAlias, Error> {
        Ok(
          <MyAlias as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_alias(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_string_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<String, Error> {
        Ok(
          <String as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_string(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_i64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<i64, Error> {
        Ok(
          <i64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_i64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_i32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<i32, Error> {
        Ok(
          <i32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_i32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_i16_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<i16, Error> {
        Ok(
          <i16 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_i16(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_i8_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<i8, Error> {
        Ok(
          <i8 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_i8(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_u64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<u64, Error> {
        Ok(
          <u64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_u64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_u32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<u32, Error> {
        Ok(
          <u32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_u32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_u16_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<u16, Error> {
        Ok(
          <u16 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_u16(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_u8_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<u8, Error> {
        Ok(
          <u8 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_u8(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_f64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<f64, Error> {
        Ok(
          <f64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_f64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_f32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<f32, Error> {
        Ok(
          <f32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_f32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_bytes_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<wasmrs_guest::Bytes, Error> {
        Ok(
          <wasmrs_guest::Bytes as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_bytes(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_datetime_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<wasmrs_guest::Timestamp, Error> {
        Ok(
          <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_datetime(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_list_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<Vec<String>, Error> {
        Ok(
          <Vec<String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_list(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn unary_map_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<std::collections::HashMap<String, String>, Error> {
        Ok(
          <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        )
      }
      let _ = MyServiceComponent::unary_map(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_type_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_type::Input, Error> {
        Ok(my_service_service::func_type::Input {
          value: <MyType as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<MyType> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_type(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_enum_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_enum::Input, Error> {
        Ok(my_service_service::func_enum::Input {
          value: <MyEnum as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<MyEnum> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_enum(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_uuid_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_uuid::Input, Error> {
        Ok(my_service_service::func_uuid::Input {
          value: <Uuid as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<Uuid> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_uuid(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_alias_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_alias::Input, Error> {
        Ok(my_service_service::func_alias::Input {
          value: <MyAlias as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<MyAlias> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_alias(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_string_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_string::Input, Error> {
        Ok(my_service_service::func_string::Input {
          value: <String as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<String> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_string(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_i64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_i64::Input, Error> {
        Ok(my_service_service::func_i64::Input {
          value: <i64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<i64> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_i64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_i32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_i32::Input, Error> {
        Ok(my_service_service::func_i32::Input {
          value: <i32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<i32> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_i32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_i16_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_i16::Input, Error> {
        Ok(my_service_service::func_i16::Input {
          value: <i16 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<i16> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_i16(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_i8_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_i8::Input, Error> {
        Ok(my_service_service::func_i8::Input {
          value: <i8 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<i8> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_i8(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_u64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_u64::Input, Error> {
        Ok(my_service_service::func_u64::Input {
          value: <u64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<u64> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_u64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_u32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_u32::Input, Error> {
        Ok(my_service_service::func_u32::Input {
          value: <u32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<u32> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_u32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_u16_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_u16::Input, Error> {
        Ok(my_service_service::func_u16::Input {
          value: <u16 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<u16> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_u16(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_u8_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_u8::Input, Error> {
        Ok(my_service_service::func_u8::Input {
          value: <u8 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<u8> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_u8(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_f64_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_f64::Input, Error> {
        Ok(my_service_service::func_f64::Input {
          value: <f64 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<f64> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_f64(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_f32_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_f32::Input, Error> {
        Ok(my_service_service::func_f32::Input {
          value: <f32 as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<f32> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_f32(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_bytes_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_bytes::Input, Error> {
        Ok(my_service_service::func_bytes::Input {
          value: <wasmrs_guest::Bytes as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<wasmrs_guest::Bytes> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_bytes(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_datetime_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_datetime::Input, Error> {
        Ok(my_service_service::func_datetime::Input {
          value: <wasmrs_guest::Timestamp as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<wasmrs_guest::Timestamp> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_datetime(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_list_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(
        mut map: std::collections::BTreeMap<String, Value>,
      ) -> Result<my_service_service::func_list::Input, Error> {
        Ok(my_service_service::func_list::Input {
          value: <Vec<String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<Vec<String>> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_list(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
  fn func_map_wrapper(input: IncomingMono) -> Result<OutgoingMono, GenericError> {
    let (tx, rx) = runtime::oneshot();
    let input = deserialize_helper(input);
    spawn(async move {
      let input_payload = match input.await {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(e));
          return;
        }
      };
      fn des(mut map: std::collections::BTreeMap<String, Value>) -> Result<my_service_service::func_map::Input, Error> {
        Ok(my_service_service::func_map::Input {
          value: <std::collections::HashMap<String, String> as serde::Deserialize>::deserialize(
            map
              .remove("value")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("value".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
          optional: <Option<std::collections::HashMap<String, String>> as serde::Deserialize>::deserialize(
            map
              .remove("optional")
              .ok_or_else(|| wasmrs_guest::Error::MissingInput("optional".to_owned()))?,
          )
          .map_err(|e| wasmrs_guest::Error::Decode(e.to_string()))?,
        })
      }
      let _ = MyServiceComponent::func_map(match des(input_payload) {
        Ok(o) => o,
        Err(e) => {
          let _ = tx.send(Err(PayloadError::application_error(e.to_string())));
          return;
        }
      })
      .await
      .map(|result| {
        serialize(&result)
          .map(|b| Payload::new_data(None, Some(b.into())))
          .map_err(|e| PayloadError::application_error(e.to_string()))
      })
      .map(|output| {
        let _ = tx.send(output);
      });
    });
    Ok(Mono::from_future(async move { rx.await? }))
  }
}

#[async_trait::async_trait(?Send)]

pub(crate) trait MyServiceService {
  async fn empty_void(
    input: my_service_service::empty_void::Input,
  ) -> Result<my_service_service::empty_void::Output, GenericError>;

  async fn unary_type(input: MyType) -> Result<my_service_service::unary_type::Output, GenericError>;

  async fn unary_enum(input: MyEnum) -> Result<my_service_service::unary_enum::Output, GenericError>;

  async fn unary_uuid(input: Uuid) -> Result<my_service_service::unary_uuid::Output, GenericError>;

  async fn unary_alias(input: MyAlias) -> Result<my_service_service::unary_alias::Output, GenericError>;

  async fn unary_string(input: String) -> Result<my_service_service::unary_string::Output, GenericError>;

  async fn unary_i64(input: i64) -> Result<my_service_service::unary_i64::Output, GenericError>;

  async fn unary_i32(input: i32) -> Result<my_service_service::unary_i32::Output, GenericError>;

  async fn unary_i16(input: i16) -> Result<my_service_service::unary_i16::Output, GenericError>;

  async fn unary_i8(input: i8) -> Result<my_service_service::unary_i8::Output, GenericError>;

  async fn unary_u64(input: u64) -> Result<my_service_service::unary_u64::Output, GenericError>;

  async fn unary_u32(input: u32) -> Result<my_service_service::unary_u32::Output, GenericError>;

  async fn unary_u16(input: u16) -> Result<my_service_service::unary_u16::Output, GenericError>;

  async fn unary_u8(input: u8) -> Result<my_service_service::unary_u8::Output, GenericError>;

  async fn unary_f64(input: f64) -> Result<my_service_service::unary_f64::Output, GenericError>;

  async fn unary_f32(input: f32) -> Result<my_service_service::unary_f32::Output, GenericError>;

  async fn unary_bytes(input: wasmrs_guest::Bytes) -> Result<my_service_service::unary_bytes::Output, GenericError>;

  async fn unary_datetime(
    input: wasmrs_guest::Timestamp,
  ) -> Result<my_service_service::unary_datetime::Output, GenericError>;

  async fn unary_list(input: Vec<String>) -> Result<my_service_service::unary_list::Output, GenericError>;

  async fn unary_map(
    input: std::collections::HashMap<String, String>,
  ) -> Result<my_service_service::unary_map::Output, GenericError>;

  async fn func_type(
    input: my_service_service::func_type::Input,
  ) -> Result<my_service_service::func_type::Output, GenericError>;

  async fn func_enum(
    input: my_service_service::func_enum::Input,
  ) -> Result<my_service_service::func_enum::Output, GenericError>;

  async fn func_uuid(
    input: my_service_service::func_uuid::Input,
  ) -> Result<my_service_service::func_uuid::Output, GenericError>;

  async fn func_alias(
    input: my_service_service::func_alias::Input,
  ) -> Result<my_service_service::func_alias::Output, GenericError>;

  async fn func_string(
    input: my_service_service::func_string::Input,
  ) -> Result<my_service_service::func_string::Output, GenericError>;

  async fn func_i64(
    input: my_service_service::func_i64::Input,
  ) -> Result<my_service_service::func_i64::Output, GenericError>;

  async fn func_i32(
    input: my_service_service::func_i32::Input,
  ) -> Result<my_service_service::func_i32::Output, GenericError>;

  async fn func_i16(
    input: my_service_service::func_i16::Input,
  ) -> Result<my_service_service::func_i16::Output, GenericError>;

  async fn func_i8(
    input: my_service_service::func_i8::Input,
  ) -> Result<my_service_service::func_i8::Output, GenericError>;

  async fn func_u64(
    input: my_service_service::func_u64::Input,
  ) -> Result<my_service_service::func_u64::Output, GenericError>;

  async fn func_u32(
    input: my_service_service::func_u32::Input,
  ) -> Result<my_service_service::func_u32::Output, GenericError>;

  async fn func_u16(
    input: my_service_service::func_u16::Input,
  ) -> Result<my_service_service::func_u16::Output, GenericError>;

  async fn func_u8(
    input: my_service_service::func_u8::Input,
  ) -> Result<my_service_service::func_u8::Output, GenericError>;

  async fn func_f64(
    input: my_service_service::func_f64::Input,
  ) -> Result<my_service_service::func_f64::Output, GenericError>;

  async fn func_f32(
    input: my_service_service::func_f32::Input,
  ) -> Result<my_service_service::func_f32::Output, GenericError>;

  async fn func_bytes(
    input: my_service_service::func_bytes::Input,
  ) -> Result<my_service_service::func_bytes::Output, GenericError>;

  async fn func_datetime(
    input: my_service_service::func_datetime::Input,
  ) -> Result<my_service_service::func_datetime::Output, GenericError>;

  async fn func_list(
    input: my_service_service::func_list::Input,
  ) -> Result<my_service_service::func_list::Output, GenericError>;

  async fn func_map(
    input: my_service_service::func_map::Input,
  ) -> Result<my_service_service::func_map::Output, GenericError>;
}

#[async_trait::async_trait(?Send)]
impl MyServiceService for MyServiceComponent {
  async fn empty_void(
    input: my_service_service::empty_void::Input,
  ) -> Result<my_service_service::empty_void::Output, GenericError> {
    Ok(crate::actions::my_service::empty_void::task(input).await?)
  }

  async fn unary_type(input: MyType) -> Result<my_service_service::unary_type::Output, GenericError> {
    Ok(crate::actions::my_service::unary_type::task(input).await?)
  }

  async fn unary_enum(input: MyEnum) -> Result<my_service_service::unary_enum::Output, GenericError> {
    Ok(crate::actions::my_service::unary_enum::task(input).await?)
  }

  async fn unary_uuid(input: Uuid) -> Result<my_service_service::unary_uuid::Output, GenericError> {
    Ok(crate::actions::my_service::unary_uuid::task(input).await?)
  }

  async fn unary_alias(input: MyAlias) -> Result<my_service_service::unary_alias::Output, GenericError> {
    Ok(crate::actions::my_service::unary_alias::task(input).await?)
  }

  async fn unary_string(input: String) -> Result<my_service_service::unary_string::Output, GenericError> {
    Ok(crate::actions::my_service::unary_string::task(input).await?)
  }

  async fn unary_i64(input: i64) -> Result<my_service_service::unary_i64::Output, GenericError> {
    Ok(crate::actions::my_service::unary_i64::task(input).await?)
  }

  async fn unary_i32(input: i32) -> Result<my_service_service::unary_i32::Output, GenericError> {
    Ok(crate::actions::my_service::unary_i32::task(input).await?)
  }

  async fn unary_i16(input: i16) -> Result<my_service_service::unary_i16::Output, GenericError> {
    Ok(crate::actions::my_service::unary_i16::task(input).await?)
  }

  async fn unary_i8(input: i8) -> Result<my_service_service::unary_i8::Output, GenericError> {
    Ok(crate::actions::my_service::unary_i8::task(input).await?)
  }

  async fn unary_u64(input: u64) -> Result<my_service_service::unary_u64::Output, GenericError> {
    Ok(crate::actions::my_service::unary_u64::task(input).await?)
  }

  async fn unary_u32(input: u32) -> Result<my_service_service::unary_u32::Output, GenericError> {
    Ok(crate::actions::my_service::unary_u32::task(input).await?)
  }

  async fn unary_u16(input: u16) -> Result<my_service_service::unary_u16::Output, GenericError> {
    Ok(crate::actions::my_service::unary_u16::task(input).await?)
  }

  async fn unary_u8(input: u8) -> Result<my_service_service::unary_u8::Output, GenericError> {
    Ok(crate::actions::my_service::unary_u8::task(input).await?)
  }

  async fn unary_f64(input: f64) -> Result<my_service_service::unary_f64::Output, GenericError> {
    Ok(crate::actions::my_service::unary_f64::task(input).await?)
  }

  async fn unary_f32(input: f32) -> Result<my_service_service::unary_f32::Output, GenericError> {
    Ok(crate::actions::my_service::unary_f32::task(input).await?)
  }

  async fn unary_bytes(input: wasmrs_guest::Bytes) -> Result<my_service_service::unary_bytes::Output, GenericError> {
    Ok(crate::actions::my_service::unary_bytes::task(input).await?)
  }

  async fn unary_datetime(
    input: wasmrs_guest::Timestamp,
  ) -> Result<my_service_service::unary_datetime::Output, GenericError> {
    Ok(crate::actions::my_service::unary_datetime::task(input).await?)
  }

  async fn unary_list(input: Vec<String>) -> Result<my_service_service::unary_list::Output, GenericError> {
    Ok(crate::actions::my_service::unary_list::task(input).await?)
  }

  async fn unary_map(
    input: std::collections::HashMap<String, String>,
  ) -> Result<my_service_service::unary_map::Output, GenericError> {
    Ok(crate::actions::my_service::unary_map::task(input).await?)
  }

  async fn func_type(
    input: my_service_service::func_type::Input,
  ) -> Result<my_service_service::func_type::Output, GenericError> {
    Ok(crate::actions::my_service::func_type::task(input).await?)
  }

  async fn func_enum(
    input: my_service_service::func_enum::Input,
  ) -> Result<my_service_service::func_enum::Output, GenericError> {
    Ok(crate::actions::my_service::func_enum::task(input).await?)
  }

  async fn func_uuid(
    input: my_service_service::func_uuid::Input,
  ) -> Result<my_service_service::func_uuid::Output, GenericError> {
    Ok(crate::actions::my_service::func_uuid::task(input).await?)
  }

  async fn func_alias(
    input: my_service_service::func_alias::Input,
  ) -> Result<my_service_service::func_alias::Output, GenericError> {
    Ok(crate::actions::my_service::func_alias::task(input).await?)
  }

  async fn func_string(
    input: my_service_service::func_string::Input,
  ) -> Result<my_service_service::func_string::Output, GenericError> {
    Ok(crate::actions::my_service::func_string::task(input).await?)
  }

  async fn func_i64(
    input: my_service_service::func_i64::Input,
  ) -> Result<my_service_service::func_i64::Output, GenericError> {
    Ok(crate::actions::my_service::func_i64::task(input).await?)
  }

  async fn func_i32(
    input: my_service_service::func_i32::Input,
  ) -> Result<my_service_service::func_i32::Output, GenericError> {
    Ok(crate::actions::my_service::func_i32::task(input).await?)
  }

  async fn func_i16(
    input: my_service_service::func_i16::Input,
  ) -> Result<my_service_service::func_i16::Output, GenericError> {
    Ok(crate::actions::my_service::func_i16::task(input).await?)
  }

  async fn func_i8(
    input: my_service_service::func_i8::Input,
  ) -> Result<my_service_service::func_i8::Output, GenericError> {
    Ok(crate::actions::my_service::func_i8::task(input).await?)
  }

  async fn func_u64(
    input: my_service_service::func_u64::Input,
  ) -> Result<my_service_service::func_u64::Output, GenericError> {
    Ok(crate::actions::my_service::func_u64::task(input).await?)
  }

  async fn func_u32(
    input: my_service_service::func_u32::Input,
  ) -> Result<my_service_service::func_u32::Output, GenericError> {
    Ok(crate::actions::my_service::func_u32::task(input).await?)
  }

  async fn func_u16(
    input: my_service_service::func_u16::Input,
  ) -> Result<my_service_service::func_u16::Output, GenericError> {
    Ok(crate::actions::my_service::func_u16::task(input).await?)
  }

  async fn func_u8(
    input: my_service_service::func_u8::Input,
  ) -> Result<my_service_service::func_u8::Output, GenericError> {
    Ok(crate::actions::my_service::func_u8::task(input).await?)
  }

  async fn func_f64(
    input: my_service_service::func_f64::Input,
  ) -> Result<my_service_service::func_f64::Output, GenericError> {
    Ok(crate::actions::my_service::func_f64::task(input).await?)
  }

  async fn func_f32(
    input: my_service_service::func_f32::Input,
  ) -> Result<my_service_service::func_f32::Output, GenericError> {
    Ok(crate::actions::my_service::func_f32::task(input).await?)
  }

  async fn func_bytes(
    input: my_service_service::func_bytes::Input,
  ) -> Result<my_service_service::func_bytes::Output, GenericError> {
    Ok(crate::actions::my_service::func_bytes::task(input).await?)
  }

  async fn func_datetime(
    input: my_service_service::func_datetime::Input,
  ) -> Result<my_service_service::func_datetime::Output, GenericError> {
    Ok(crate::actions::my_service::func_datetime::task(input).await?)
  }

  async fn func_list(
    input: my_service_service::func_list::Input,
  ) -> Result<my_service_service::func_list::Output, GenericError> {
    Ok(crate::actions::my_service::func_list::task(input).await?)
  }

  async fn func_map(
    input: my_service_service::func_map::Input,
  ) -> Result<my_service_service::func_map::Output, GenericError> {
    Ok(crate::actions::my_service::func_map::task(input).await?)
  }
}

pub mod my_service_service {
  #[allow(unused_imports)]
  pub(crate) use super::*;

  pub mod empty_void {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {}

    pub(crate) type Output = ();
  }

  pub mod unary_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = MyType;

    pub(crate) type Output = MyType;
  }

  pub mod unary_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = MyEnum;

    pub(crate) type Output = MyEnum;
  }

  pub mod unary_uuid {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = Uuid;

    pub(crate) type Output = Uuid;
  }

  pub mod unary_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = MyAlias;

    pub(crate) type Output = MyAlias;
  }

  pub mod unary_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = String;

    pub(crate) type Output = String;
  }

  pub mod unary_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = i64;

    pub(crate) type Output = i64;
  }

  pub mod unary_i32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = i32;

    pub(crate) type Output = i32;
  }

  pub mod unary_i16 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = i16;

    pub(crate) type Output = i16;
  }

  pub mod unary_i8 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = i8;

    pub(crate) type Output = i8;
  }

  pub mod unary_u64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = u64;

    pub(crate) type Output = u64;
  }

  pub mod unary_u32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = u32;

    pub(crate) type Output = u32;
  }

  pub mod unary_u16 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = u16;

    pub(crate) type Output = u16;
  }

  pub mod unary_u8 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = u8;

    pub(crate) type Output = u8;
  }

  pub mod unary_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = f64;

    pub(crate) type Output = f64;
  }

  pub mod unary_f32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = f32;

    pub(crate) type Output = f32;
  }

  pub mod unary_bytes {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = wasmrs_guest::Bytes;

    pub(crate) type Output = wasmrs_guest::Bytes;
  }

  pub mod unary_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = wasmrs_guest::Timestamp;

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub mod unary_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = Vec<String>;

    pub(crate) type Output = Vec<String>;
  }

  pub mod unary_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    pub(crate) type Input = std::collections::HashMap<String, String>;

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }

  pub mod func_type {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyType,

      pub(crate) optional: Option<MyType>,
    }

    pub(crate) type Output = MyType;
  }

  pub mod func_enum {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyEnum,

      pub(crate) optional: Option<MyEnum>,
    }

    pub(crate) type Output = MyEnum;
  }

  pub mod func_uuid {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Uuid,

      pub(crate) optional: Option<Uuid>,
    }

    pub(crate) type Output = Uuid;
  }

  pub mod func_alias {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: MyAlias,

      pub(crate) optional: Option<MyAlias>,
    }

    pub(crate) type Output = MyAlias;
  }

  pub mod func_string {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: String,

      pub(crate) optional: Option<String>,
    }

    pub(crate) type Output = String;
  }

  pub mod func_i64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i64,

      pub(crate) optional: Option<i64>,
    }

    pub(crate) type Output = i64;
  }

  pub mod func_i32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i32,

      pub(crate) optional: Option<i32>,
    }

    pub(crate) type Output = i32;
  }

  pub mod func_i16 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i16,

      pub(crate) optional: Option<i16>,
    }

    pub(crate) type Output = i16;
  }

  pub mod func_i8 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: i8,

      pub(crate) optional: Option<i8>,
    }

    pub(crate) type Output = i8;
  }

  pub mod func_u64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: u64,

      pub(crate) optional: Option<u64>,
    }

    pub(crate) type Output = u64;
  }

  pub mod func_u32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: u32,

      pub(crate) optional: Option<u32>,
    }

    pub(crate) type Output = u32;
  }

  pub mod func_u16 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: u16,

      pub(crate) optional: Option<u16>,
    }

    pub(crate) type Output = u16;
  }

  pub mod func_u8 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: u8,

      pub(crate) optional: Option<u8>,
    }

    pub(crate) type Output = u8;
  }

  pub mod func_f64 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: f64,

      pub(crate) optional: Option<f64>,
    }

    pub(crate) type Output = f64;
  }

  pub mod func_f32 {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: f32,

      pub(crate) optional: Option<f32>,
    }

    pub(crate) type Output = f32;
  }

  pub mod func_bytes {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: wasmrs_guest::Bytes,

      pub(crate) optional: Option<wasmrs_guest::Bytes>,
    }

    pub(crate) type Output = wasmrs_guest::Bytes;
  }

  pub mod func_datetime {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: wasmrs_guest::Timestamp,

      pub(crate) optional: Option<wasmrs_guest::Timestamp>,
    }

    pub(crate) type Output = wasmrs_guest::Timestamp;
  }

  pub mod func_list {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: Vec<String>,

      pub(crate) optional: Option<Vec<String>>,
    }

    pub(crate) type Output = Vec<String>;
  }

  pub mod func_map {
    #[allow(unused_imports)]
    pub(crate) use super::*;

    #[allow(unused)]
    pub(crate) struct Input {
      pub(crate) value: std::collections::HashMap<String, String>,

      pub(crate) optional: Option<std::collections::HashMap<String, String>>,
    }

    pub(crate) type Output = std::collections::HashMap<String, String>;
  }
}

pub trait Repository {
  fn get_data() -> MyType;
}

/// MyType is a class
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyType {
  /// same type value
  #[serde(rename = "sameValue")]
  pub same_value: Box<Option<MyType>>,
  /// type value
  #[serde(rename = "typeValue")]
  pub type_value: MyOtherType,
  /// string value
  #[serde(rename = "stringValue")]
  pub string_value: String,
  /// string option
  #[serde(rename = "stringOption")]
  pub string_option: Option<String>,
  /// i64 value
  #[serde(rename = "i64Value")]
  pub i64_value: i64,
  /// i64 option
  #[serde(rename = "i64Option")]
  pub i64_option: Option<i64>,
  /// i32 value
  #[serde(rename = "i32Value")]
  pub i32_value: i32,
  /// i32 option
  #[serde(rename = "i32Option")]
  pub i32_option: Option<i32>,
  /// i16 value
  #[serde(rename = "i16Value")]
  pub i16_value: i16,
  /// i16 option
  #[serde(rename = "i16Option")]
  pub i16_option: Option<i16>,
  /// i8 value
  #[serde(rename = "i8Value")]
  pub i8_value: i8,
  /// i8 option
  #[serde(rename = "i8Option")]
  pub i8_option: Option<i8>,
  /// u64 value
  #[serde(rename = "u64Value")]
  pub u64_value: u64,
  /// u64 option
  #[serde(rename = "u64Option")]
  pub u64_option: Option<u64>,
  /// u32 value
  #[serde(rename = "u32Value")]
  pub u32_value: u32,
  /// u32 option
  #[serde(rename = "u32Option")]
  pub u32_option: Option<u32>,
  /// u16 value
  #[serde(rename = "u16Value")]
  pub u16_value: u16,
  /// u16 option
  #[serde(rename = "u16Option")]
  pub u16_option: Option<u16>,
  /// u8 value
  #[serde(rename = "u8Value")]
  pub u8_value: u8,
  /// u8 option
  #[serde(rename = "u8Option")]
  pub u8_option: Option<u8>,
  /// f64 value
  #[serde(rename = "f64Value")]
  pub f64_value: f64,
  /// f64 option
  #[serde(rename = "f64Option")]
  pub f64_option: Option<f64>,
  /// f32 value
  #[serde(rename = "f32Value")]
  pub f32_value: f32,
  /// f32 option
  #[serde(rename = "f32Option")]
  pub f32_option: Option<f32>,
  /// datetime value
  #[serde(rename = "datetimeValue")]
  pub datetime_value: wasmrs_guest::Timestamp,
  /// datetime option
  #[serde(rename = "datetimeOption")]
  pub datetime_option: Option<wasmrs_guest::Timestamp>,
  /// bytes value
  #[serde(rename = "bytesValue")]
  pub bytes_value: wasmrs_guest::Bytes,
  /// bytes option
  #[serde(rename = "bytesOption")]
  pub bytes_option: Option<wasmrs_guest::Bytes>,
  /// map value
  #[serde(rename = "mapValue")]
  pub map_value: std::collections::HashMap<String, i64>,
  /// map of types
  #[serde(rename = "mapOfTypes")]
  pub map_of_types: std::collections::HashMap<String, MyType>,
  /// array value
  #[serde(rename = "arrayValue")]
  pub array_value: Vec<String>,
  /// array of types
  #[serde(rename = "arrayOfTypes")]
  pub array_of_types: Vec<MyType>,
  /// union value
  #[serde(rename = "unionValue")]
  pub union_value: Box<MyUnion>,
  /// union option
  #[serde(rename = "unionOption")]
  pub union_option: Box<Option<MyUnion>>,
  /// enum value
  #[serde(rename = "enumValue")]
  pub enum_value: MyEnum,
  /// enum option
  #[serde(rename = "enumOption")]
  pub enum_option: Option<MyEnum>,
  /// enum value
  #[serde(rename = "aliasValue")]
  pub alias_value: Uuid,
  /// enum option
  #[serde(rename = "aliasOption")]
  pub alias_option: Option<Uuid>,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MyOtherType {
  #[serde(rename = "foo")]
  pub foo: String,
  #[serde(rename = "bar")]
  pub bar: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyUnion {
  MyType(Box<MyType>),
  MyEnum(MyEnum),
  string(String),
}

/// MyEnum is an emuneration
#[derive(serde::Serialize, serde::Deserialize)]
pub enum MyEnum {
  /// ONE value
  One,
  /// TWO value
  Two,
  /// THREE value
  Three,
}
impl std::fmt::Display for MyEnum {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::One => "one",
        Self::Two => unimplemented!("No display value provided in schema"),
        Self::Three => "three",
      }
    )
  }
}
impl std::convert::TryFrom<u32> for MyEnum {
  type Error = String;
  fn try_from(index: u32) -> Result<Self, Self::Error> {
    match index {
      0 => Ok(Self::One),
      1 => Ok(Self::Two),
      2 => Ok(Self::Three),
      _ => Err(format!("{} is not a valid index for MyEnum", index)),
    }
  }
}
impl Into<u32> for MyEnum {
  fn into(self) -> u32 {
    match self {
      Self::One => unreachable!(),
      Self::Two => 1,
      Self::Three => 2,
    }
  }
}

pub(crate) fn init_imports() {
  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_I64_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamI64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_F64_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamF64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_TYPE_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamType",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ENUM_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamEnum",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_UUID_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamUUID",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ALIAS_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamAlias",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_STRING_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamString",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_BOOL_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamBool",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_DATETIME_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamDatetime",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_LIST_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamList",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_MAP_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamMap",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_I64_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsI64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_F64_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsF64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_TYPE_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsType",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_ENUM_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsEnum",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_UUID_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsUUID",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_ALIAS_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsAlias",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_STRING_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsString",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_BOOL_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsBool",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_DATETIME_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsDatetime",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_LIST_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsList",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_STREAM_ARGS_MAP_INDEX_BYTES),
    OperationType::RequestStream,
    "iota.testing.MyProvider",
    "requestStreamArgsMap",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_I64_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelI64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_F64_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelF64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_TYPE_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelType",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ENUM_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelEnum",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ALIAS_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelAlias",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_STRING_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelString",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_BOOL_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelBool",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_DATETIME_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelDatetime",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_LIST_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelList",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_MAP_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelMap",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_I64_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsI64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_F64_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsF64",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_TYPE_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsType",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_ENUM_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsEnum",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_ALIAS_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsAlias",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_STRING_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsString",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_BOOL_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsBool",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_DATETIME_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsDatetime",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_LIST_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsList",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_ARGS_MAP_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelArgsMap",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_VOID_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelVoid",
  );

  wasmrs_guest::add_import(
    u32::from_be_bytes(MY_PROVIDER_REQUEST_CHANNEL_NON_STREAM_OUTPUT_INDEX_BYTES),
    OperationType::RequestChannel,
    "iota.testing.MyProvider",
    "requestChannelNonStreamOutput",
  );
}
pub(crate) fn init_exports() {
  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "emptyVoid",
    MyServiceComponent::empty_void_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryType",
    MyServiceComponent::unary_type_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryEnum",
    MyServiceComponent::unary_enum_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryUUID",
    MyServiceComponent::unary_uuid_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryAlias",
    MyServiceComponent::unary_alias_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryString",
    MyServiceComponent::unary_string_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryI64",
    MyServiceComponent::unary_i64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryI32",
    MyServiceComponent::unary_i32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryI16",
    MyServiceComponent::unary_i16_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryI8",
    MyServiceComponent::unary_i8_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryU64",
    MyServiceComponent::unary_u64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryU32",
    MyServiceComponent::unary_u32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryU16",
    MyServiceComponent::unary_u16_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryU8",
    MyServiceComponent::unary_u8_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryF64",
    MyServiceComponent::unary_f64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryF32",
    MyServiceComponent::unary_f32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryBytes",
    MyServiceComponent::unary_bytes_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryDatetime",
    MyServiceComponent::unary_datetime_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryList",
    MyServiceComponent::unary_list_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "unaryMap",
    MyServiceComponent::unary_map_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcType",
    MyServiceComponent::func_type_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcEnum",
    MyServiceComponent::func_enum_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcUUID",
    MyServiceComponent::func_uuid_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcAlias",
    MyServiceComponent::func_alias_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcString",
    MyServiceComponent::func_string_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcI64",
    MyServiceComponent::func_i64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcI32",
    MyServiceComponent::func_i32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcI16",
    MyServiceComponent::func_i16_wrapper,
  );

  wasmrs_guest::register_request_response("iota.testing.MyService", "funcI8", MyServiceComponent::func_i8_wrapper);

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcU64",
    MyServiceComponent::func_u64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcU32",
    MyServiceComponent::func_u32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcU16",
    MyServiceComponent::func_u16_wrapper,
  );

  wasmrs_guest::register_request_response("iota.testing.MyService", "funcU8", MyServiceComponent::func_u8_wrapper);

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcF64",
    MyServiceComponent::func_f64_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcF32",
    MyServiceComponent::func_f32_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcBytes",
    MyServiceComponent::func_bytes_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcDatetime",
    MyServiceComponent::func_datetime_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcList",
    MyServiceComponent::func_list_wrapper,
  );

  wasmrs_guest::register_request_response(
    "iota.testing.MyService",
    "funcMap",
    MyServiceComponent::func_map_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamI64",
    MyStreamerComponent::request_stream_i64_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamF64",
    MyStreamerComponent::request_stream_f64_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamType",
    MyStreamerComponent::request_stream_type_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamEnum",
    MyStreamerComponent::request_stream_enum_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamUUID",
    MyStreamerComponent::request_stream_uuid_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamAlias",
    MyStreamerComponent::request_stream_alias_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamString",
    MyStreamerComponent::request_stream_string_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamBool",
    MyStreamerComponent::request_stream_bool_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamDatetime",
    MyStreamerComponent::request_stream_datetime_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamList",
    MyStreamerComponent::request_stream_list_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamMap",
    MyStreamerComponent::request_stream_map_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsI64",
    MyStreamerComponent::request_stream_args_i64_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsF64",
    MyStreamerComponent::request_stream_args_f64_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsType",
    MyStreamerComponent::request_stream_args_type_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsEnum",
    MyStreamerComponent::request_stream_args_enum_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsUUID",
    MyStreamerComponent::request_stream_args_uuid_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsAlias",
    MyStreamerComponent::request_stream_args_alias_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsString",
    MyStreamerComponent::request_stream_args_string_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsBool",
    MyStreamerComponent::request_stream_args_bool_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsDatetime",
    MyStreamerComponent::request_stream_args_datetime_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsList",
    MyStreamerComponent::request_stream_args_list_wrapper,
  );

  wasmrs_guest::register_request_stream(
    "iota.testing.MyStreamer",
    "requestStreamArgsMap",
    MyStreamerComponent::request_stream_args_map_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelI64",
    MyStreamerComponent::request_channel_i64_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelF64",
    MyStreamerComponent::request_channel_f64_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelType",
    MyStreamerComponent::request_channel_type_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelEnum",
    MyStreamerComponent::request_channel_enum_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelAlias",
    MyStreamerComponent::request_channel_alias_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelString",
    MyStreamerComponent::request_channel_string_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelBool",
    MyStreamerComponent::request_channel_bool_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelDatetime",
    MyStreamerComponent::request_channel_datetime_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelList",
    MyStreamerComponent::request_channel_list_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelMap",
    MyStreamerComponent::request_channel_map_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsI64",
    MyStreamerComponent::request_channel_args_i64_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsF64",
    MyStreamerComponent::request_channel_args_f64_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsType",
    MyStreamerComponent::request_channel_args_type_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsEnum",
    MyStreamerComponent::request_channel_args_enum_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsAlias",
    MyStreamerComponent::request_channel_args_alias_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsString",
    MyStreamerComponent::request_channel_args_string_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsBool",
    MyStreamerComponent::request_channel_args_bool_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsDatetime",
    MyStreamerComponent::request_channel_args_datetime_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsList",
    MyStreamerComponent::request_channel_args_list_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelArgsMap",
    MyStreamerComponent::request_channel_args_map_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelVoid",
    MyStreamerComponent::request_channel_void_wrapper,
  );

  wasmrs_guest::register_request_channel(
    "iota.testing.MyStreamer",
    "requestChannelNonStreamOutput",
    MyStreamerComponent::request_channel_non_stream_output_wrapper,
  );
}

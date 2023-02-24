use guest::*;
use wasmrs_guest as guest;

#[no_mangle]
extern "C" fn __wasmrs_init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);

  guest::register_request_response("greeting", "sayHello", request_response);
  guest::register_request_stream("echo", "chars", request_stream);
  guest::register_request_channel("echo", "reverse", request_channel);
}

fn request_response(input: Mono<ParsedPayload, PayloadError>) -> Result<Mono<Payload, PayloadError>, GenericError> {
  Ok(Mono::from_future(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    let output = format!("Hello, {}!", input);
    Ok(Payload::new_data(None, Some(serialize(&output).unwrap().into())))
  }))
}

fn request_stream(
  input: Mono<ParsedPayload, PayloadError>,
) -> Result<FluxReceiver<Payload, PayloadError>, GenericError> {
  let channel = Flux::<Payload, PayloadError>::new();
  let rx = channel.take_rx().unwrap();
  spawn(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    for char in input.chars() {
      channel
        .send(Payload::new_data(None, Some(serialize(&char).unwrap().into())))
        .unwrap();
    }
  });

  Ok(rx)
}
fn request_channel(
  mut input: FluxReceiver<ParsedPayload, PayloadError>,
) -> Result<FluxReceiver<Payload, PayloadError>, GenericError> {
  let channel = Flux::<Payload, PayloadError>::new();
  let rx = channel.take_rx().unwrap();
  spawn(async move {
    while let Some(payload) = input.next().await {
      if let Err(e) = payload {
        println!("{}", e);
        continue;
      }
      let payload = payload.unwrap();
      let input = deserialize::<String>(&payload.data).unwrap();
      let output: String = input.chars().rev().collect();
      if let Err(e) = channel.send(Payload::new_data(None, Some(serialize(&output).unwrap().into()))) {
        println!("{}", e);
      }
    }
  });

  Ok(rx)
}

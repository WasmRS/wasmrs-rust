use guest::*;
use wasmrs_guest as guest;

#[no_mangle]
extern "C" fn __wasmrs_init(guest_buffer_size: u32, host_buffer_size: u32, max_host_frame_len: u32) {
  guest::init(guest_buffer_size, host_buffer_size, max_host_frame_len);

  guest::register_request_response("greeting", "sayHello", request_response);
  guest::register_request_stream("echo", "chars", request_stream);
  guest::register_request_channel("echo", "reverse", request_channel);
  guest::register_request_channel("test", "callback", channel_callback);
  guest::add_import(0, OperationType::RequestChannel, "test", "echo");
}

fn request_response(input: Mono<Payload, PayloadError>) -> Result<Mono<RawPayload, PayloadError>, GenericError> {
  Ok(Mono::from_future(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    let output = format!("Hello, {}!", input);
    Ok(RawPayload::new_data(None, Some(serialize(&output).unwrap().into())))
  }))
}

fn request_stream(input: Mono<Payload, PayloadError>) -> Result<FluxReceiver<RawPayload, PayloadError>, GenericError> {
  let channel = FluxChannel::<RawPayload, PayloadError>::new();
  let rx = channel.take_rx().unwrap();
  spawn(async move {
    let input = deserialize::<String>(&input.await.unwrap().data).unwrap();
    for char in input.chars() {
      channel
        .send(RawPayload::new_data(None, Some(serialize(&char).unwrap().into())))
        .unwrap();
    }
  });

  Ok(rx)
}

fn request_channel(
  mut input: FluxReceiver<Payload, PayloadError>,
) -> Result<FluxReceiver<RawPayload, PayloadError>, GenericError> {
  let channel = FluxChannel::<RawPayload, PayloadError>::new();
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
      if let Err(e) = channel.send(RawPayload::new_data(None, Some(serialize(&output).unwrap().into()))) {
        println!("{}", e);
      }
    }
  });

  Ok(rx)
}

fn channel_callback(
  mut input: FluxReceiver<Payload, PayloadError>,
) -> Result<FluxReceiver<RawPayload, PayloadError>, GenericError> {
  let (job_tx, job_rx) = FluxChannel::<RawPayload, PayloadError>::new_parts();
  let (host_tx, host_rx) = FluxChannel::<RawPayload, PayloadError>::new_parts();
  let mut host_stream = Host::default().request_channel(Box::new(host_rx));
  spawn(async move {
    println!("waiting for input...");
    while let Some(payload) = input.next().await {
      if let Err(e) = payload {
        println!("{}", e);
        continue;
      }
      let payload = payload.unwrap();
      println!("got payload: {:?}", payload);
      let input = deserialize::<String>(&payload.data).unwrap();
      let md = Metadata::new(0);
      host_tx
        .send(RawPayload::new_data(
          Some(md.encode()),
          Some(serialize(&input).unwrap().into()),
        ))
        .unwrap();
    }
  });
  spawn(async move {
    println!("waiting for host output...");
    while let Some(Ok(payload)) = host_stream.next().await {
      let output = deserialize::<String>(&payload.data.unwrap()).unwrap();

      println!("sending final output...");
      job_tx
        .send(RawPayload::new_data(None, Some(serialize(&output).unwrap().into())))
        .unwrap();
    }
  });

  Ok(job_rx)
}

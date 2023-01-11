use crate::actions::test_service::chars::*;

pub(crate) async fn task(
  input: Inputs,
  outputs: Flux<Outputs, PayloadError>,
) -> Result<Flux<Outputs, PayloadError>, crate::Error> {
  for c in input.input.chars() {
    outputs.send(c.to_string()).unwrap();
  }
  outputs.complete();
  Ok(outputs)
}

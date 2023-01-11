use crate::actions::test_service::reverse::*;

pub(crate) async fn task(
  mut input: FluxReceiver<Inputs, PayloadError>,
  outputs: Flux<Outputs, PayloadError>,
) -> Result<Flux<Outputs, PayloadError>, crate::Error> {
  while let Some(c) = input.next().await {
    match c {
      Ok(c) => {
        outputs.send(c.chars().rev().collect()).unwrap();
      }
      Err(e) => outputs.error(PayloadError::application_error(e.to_string())).unwrap(),
    }
  }
  outputs.complete();
  Ok(outputs)
}

use crate::actions::test_service::chars::*;

pub(crate) async fn task(input: Input) -> Result<Output, crate::Error> {
  let (tx, output) = FluxChannel::new_parts();
  for c in input.input.chars() {
    tx.send(c.to_string()).unwrap();
  }
  tx.complete();
  Ok(output)
}

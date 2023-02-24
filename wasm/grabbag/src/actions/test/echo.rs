use crate::actions::test_service::echo::*;

pub(crate) async fn task(input: Inputs) -> Result<Outputs, crate::Error> {
  Ok(input.message)
}

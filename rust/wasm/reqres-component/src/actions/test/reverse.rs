use crate::actions::test_service::reverse::*;

pub(crate) async fn task(input: Inputs) -> Result<Outputs, crate::Error> {
  Ok(input.input.chars().rev().collect())
}

use crate::actions::test_service::echo::*;

pub(crate) async fn task(input: Input) -> Result<Output, crate::Error> {
  Ok(input.message)
}

use crate::actions::test_service::test::*;

pub(crate) async fn task(input: Inputs) -> Result<Outputs, crate::Error> {
  Ok("test".to_owned())
}

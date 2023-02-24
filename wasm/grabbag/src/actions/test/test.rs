use crate::actions::test_service::test::*;

pub(crate) async fn task(_input: Inputs) -> Result<Outputs, crate::Error> {
  Ok("test".to_owned())
}

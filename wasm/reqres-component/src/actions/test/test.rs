use crate::actions::test_service::test::*;

pub(crate) async fn task(_input: Input) -> Result<Output, crate::Error> {
  Ok("test".to_owned())
}

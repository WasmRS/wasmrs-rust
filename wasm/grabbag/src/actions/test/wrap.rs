use crate::actions::test_service::wrap::*;

pub(crate) async fn task(mut input: Inputs) -> Result<Outputs, crate::Error> {
  let output = Flux::new();
  while let Some(msg) = input.input.next().await {
    let msg = msg?;
    let wrapped = format!("{}{}{}", input.left, msg, input.right);
    println!("sending output {:?}", wrapped);
    output.send(wrapped).unwrap();
  }
  Ok(output.take_rx()?)
}

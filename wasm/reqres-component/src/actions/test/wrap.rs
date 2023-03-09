use crate::actions::test_service::wrap::*;

pub(crate) async fn task(mut input: Input) -> Result<Output, crate::Error> {
  let (tx, output) = FluxChannel::new_parts();
  while let Some(msg) = input.input.next().await {
    let msg = msg?;
    let wrapped = format!("{}{}{}", input.left, msg, input.right);
    println!("sending output {:?}", wrapped);
    tx.send(wrapped).unwrap();
  }
  Ok(output)
}

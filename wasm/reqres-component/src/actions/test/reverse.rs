use crate::actions::test_service::reverse::*;

pub(crate) async fn task(mut input: Input) -> Result<Output, crate::Error> {
  println!("starting task");
  let (tx, rx) = FluxChannel::new_parts();
  spawn(async move {
    while let Some(c) = input.input.next().await {
      println!("got input {:?}", c);
      match c {
        Ok(c) => {
          let reversed = c.chars().rev().collect();
          println!("sending output {:?}", reversed);
          tx.send(reversed).unwrap();
        }
        Err(e) => tx.error(PayloadError::application_error(e.to_string(), None)).unwrap(),
      }
    }
    tx.complete();
    println!("done");
  });

  Ok(rx)
}

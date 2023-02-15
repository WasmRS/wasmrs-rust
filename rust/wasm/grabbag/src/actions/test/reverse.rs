use crate::actions::test_service::reverse::*;

pub(crate) async fn task(mut input: Inputs) -> Result<Outputs, crate::Error> {
  println!("starting task");
  let (tx, rx) = Flux::new_channels();
  spawn(async move {
    while let Some(c) = input.input.next().await {
      println!("got input {:?}", c);
      match c {
        Ok(c) => {
          let reversed = c.chars().rev().collect();
          println!("sending output {:?}", reversed);
          tx.send(reversed).unwrap();
        }
        Err(e) => tx.error(PayloadError::application_error(e.to_string())).unwrap(),
      }
    }
    tx.complete();
    println!("done");
  });

  Ok(rx)
}

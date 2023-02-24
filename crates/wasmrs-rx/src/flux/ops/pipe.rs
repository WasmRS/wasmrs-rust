use std::pin::Pin;
use std::task::Poll;

use futures::{Stream, TryStreamExt};
use pin_project_lite::pin_project;

use crate::flux::Flux;
use wasmrs_runtime::ConditionallySafe;

pin_project! {
/// A [FluxPipe] is the result of piping one [Flux] into another.
pub struct FluxPipe<Item, Err, From>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    #[pin]
    from: From,
    to: Flux<Item, Err>,
}
}

impl<Item, Err, From> FluxPipe<Item, Err, From>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  /// Create a new [FluxPipe]
  pub fn new(from: From, to: Flux<Item, Err>) -> Self {
    Self { from, to }
  }
}

impl<Item, Err, From> FluxPipe<Item, Err, From>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
  From: Stream<Item = Result<Item, Err>>,
{
}

impl<Item, Err, From> Stream for FluxPipe<Item, Err, From>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
  From: Stream<Item = Result<Item, Err>> + Unpin,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    self.get_mut().from.try_poll_next_unpin(cx)
  }
}
#[cfg(all(test, not(target_family = "wasm")))]
mod test {

  use anyhow::Result;
  use futures::StreamExt;

  use super::*;
  use crate::flux::Observer;
  use crate::Observable;

  #[tokio::test]
  async fn test_pipes() -> Result<()> {
    let (flux, observer) = Flux::new_channels();

    flux.send("First".to_owned())?;

    let second_flux = Flux::<String, String>::new();

    let mut pipe = observer.pipe(second_flux);

    let value = pipe.next().await;
    assert_eq!(value, Some(Ok("First".to_owned())));
    Ok(())
  }
}

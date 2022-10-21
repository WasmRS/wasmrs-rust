use std::pin::Pin;
use std::sync::atomic::AtomicBool;
use std::task::Poll;

use futures::{Future, FutureExt, Stream};

use crate::runtime::*;
use crate::Error;

mod ops;
pub use ops::*;
mod receiver;
pub use receiver::*;
mod signal;
pub use signal::*;
mod observer;
pub use observer::*;
mod observable;
pub use observable::*;

type FutureResult<Item, Err> = BoxFuture<Result<Option<Result<Item, Err>>, Error>>;

pub type FluxBox<Item, Err> = Pin<Box<dyn Observable<Item, Err>>>;

pub trait MonoFuture<Item, Err>: Future<Output = Result<Item, Err>> + ConditionallySafe {}

#[allow(missing_debug_implementations)]
#[must_use]
pub struct Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  inner: Option<Pin<Box<dyn MonoFuture<Item, Err>>>>,
  is_complete: bool,
}

impl<Item, Err> Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  pub fn new() -> Self {
    Self {
      inner: None,
      is_complete: false,
    }
  }

  pub fn from_future<Fut>(fut: Fut) -> Self
  where Fut: MonoFuture<Item, Err> {
    Self {
      inner: Some(Box::pin(fut)),
      is_complete: false,
    }
  }

  pub fn new_error(err: Err) -> Self {
    Self {
      inner: Some(Box::pin(futures::future::ready(Err(err)))),
      is_complete: true,
    }
  }

  pub fn new_success(ok: Item) -> Self {
    Self {
      inner: Some(Box::pin(futures::future::ready(Ok(ok)))),
      is_complete: true,
    }
  }

  pub fn success(&mut self, ok: Item) {
    assert!(self.inner.is_none(), "Can not push more than one value to a Mono");
    self.inner = Some(Box::pin(futures::future::ready(Ok(ok))));
  }

  pub fn error(&mut self, error: Err) {
    assert!(self.inner.is_none(), "Can not push more than one value to a Mono");
    self.inner = Some(Box::pin(futures::future::ready(Err(error))));
  }
}

impl<Item, Err> Default for Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<Item, Err, T> MonoFuture<Item, Err> for T
where
  T: Future<Output = Result<Item, Err>> + ConditionallySafe,
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
}

impl<Item, Err> Future for Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  type Output = Result<Item, Err>;

  fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
    warn!("polling");
    match self.get_mut().inner.as_mut() {
      Some(mut inner_future) => inner_future.poll_unpin(cx),
      None => {
        unreachable!();
        // cx.waker().wake_by_ref();
        // Poll::Pending
      }
    }
  }
}

#[must_use]
#[allow(missing_debug_implementations)]
pub struct Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  complete: AtomicBool,
  tx: UnboundedSender<Signal<Item, Err>>,
  rx: FluxReceiver<Item, Err>,
}

impl<Item, Err> Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  pub fn new() -> Self {
    let (tx, rx) = unbounded_channel();

    Self {
      complete: AtomicBool::new(false),
      tx,
      rx: FluxReceiver::new(rx),
    }
  }

  pub fn new_parts() -> (Self, FluxReceiver<Item, Err>) {
    let (tx, rx) = unbounded_channel();

    (
      Self {
        complete: AtomicBool::new(false),
        tx,
        rx: FluxReceiver::none(),
      },
      FluxReceiver::new(rx),
    )
  }

  #[must_use]
  pub fn is_closed(&self) -> bool {
    self.tx.is_closed()
  }

  #[must_use]
  pub fn recv(&self) -> FutureResult<Item, Err>
  where
    Err: 'static,
    Item: 'static,
  {
    let val = self.rx.recv();
    Box::pin(async move { val.await })
  }

  pub fn split_receiver(&self) -> Result<FluxReceiver<Item, Err>, Error> {
    self.rx.eject().ok_or(Error::ReceiverAlreadyGone)
  }
}

impl<Item, Err> Clone for Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn clone(&self) -> Self {
    Self {
      complete: AtomicBool::new(self.complete.load(std::sync::atomic::Ordering::SeqCst)),
      tx: self.tx.clone(),
      rx: self.rx.clone(),
    }
  }
}

impl<Item, Err> Observable<Item, Err> for Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
}

impl<Item, Err> Observer<Item, Err> for Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn send_signal(&self, signal: Signal<Item, Err>) -> Result<(), Error> {
    self.tx.send(signal)
  }

  fn is_complete(&self) -> bool {
    self.tx.is_closed()
  }

  fn complete(&self) {
    self.complete.store(false, std::sync::atomic::Ordering::SeqCst);
    let _ = self.send_signal(Signal::Complete);
  }
}

impl<Item, Err> Default for Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<Item, Err> Stream for Flux<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    self.rx.poll_recv(cx)
  }
}

fn signal_into_result<Item, Err>(signal: Option<Signal<Item, Err>>) -> Option<Result<Item, Err>>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  match signal {
    Some(Signal::Complete) => None,
    Some(Signal::Ok(v)) => Some(Ok(v)),
    Some(Signal::Err(e)) => Some(Err(e)),
    None => None,
  }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {

  use anyhow::Result;
  use futures::StreamExt;

  use super::*;

  #[tokio::test]
  async fn test_flux() -> Result<()> {
    let mut flux = Flux::<u32, u32>::new();
    flux.send(1)?;
    let value = flux.next().await;
    assert_eq!(value, Some(Ok(1)));
    let stream = flux.split_receiver().unwrap();

    flux.send(2)?;
    let value = stream.recv().await?;
    assert_eq!(value, Some(Ok(2)));
    let stream = flux.split_receiver();
    assert!(stream.is_err());
    Ok(())
  }

  #[tokio::test]
  async fn test_mono() -> Result<()> {
    let mut mono = Mono::<String, String>::new();
    mono.success("Hello".to_owned());
    let value = mono.await;
    assert_eq!(value, Ok("Hello".to_owned()));
    Ok(())
  }

  // #[tokio::test]
  // async fn test_from_receiver() -> Result<()> {
  //     let mut flux = Flux::<u32, u32>::new();
  //     flux.send(1)?;
  //     let stream = flux.split_receiver().unwrap();
  //     flux.send(2)?;
  //     let flux2 = Flux::from(stream);
  //     flux.send(3)?;
  //     flux2.send(4)?;
  //     let value = flux2.recv().await?;
  //     assert_eq!(value, Some(Ok(1)));
  //     let value = flux2.recv().await?;
  //     assert_eq!(value, Some(Ok(2)));
  //     let value = flux2.recv().await?;
  //     assert_eq!(value, Some(Ok(3)));
  //     let value = flux2.recv().await?;
  //     assert_eq!(value, Some(Ok(4)));
  //     Ok(())
  // }
}

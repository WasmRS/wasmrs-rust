use std::io::Write;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::task::Poll;

use futures::AsyncRead;
use futures::{Future, FutureExt, Stream};
use wasmrs_runtime::unbounded_channel;
use wasmrs_runtime::BoxFuture;
use wasmrs_runtime::ConditionallySafe;
use wasmrs_runtime::UnboundedSender;

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

/// A pinned, boxed [Flux].
pub type FluxBox<Item, Err> = Pin<Box<dyn Observable<Item, Err>>>;

/// A [Future] that wraps a [Result] and can be used as a [Mono].
pub trait MonoFuture<Item, Err>: Future<Output = Result<Item, Err>> + ConditionallySafe {}

#[allow(missing_debug_implementations)]
#[must_use]
/// An implementation of [Mono] as seen in RSocket and reactive streams. It is similar to a [Future<Output = Result<Item, Err>>] that can be pushed to after instantiation.
pub struct Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  inner: Option<Pin<Box<dyn MonoFuture<Item, Err>>>>,
  done: AtomicBool,
}

impl<Item, Err> Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  /// Create a new [Mono].
  pub fn new() -> Self {
    Self {
      inner: None,
      done: AtomicBool::new(false),
    }
  }

  /// Create a [Pin<Box<Mono>>] from a [Mono].
  #[must_use]
  pub fn boxed(self) -> Pin<Box<Self>> {
    Box::pin(self)
  }

  /// Create a [Mono] from a [Future].
  pub fn from_future<Fut>(fut: Fut) -> Self
  where
    Fut: MonoFuture<Item, Err>,
  {
    Self {
      inner: Some(Box::pin(fut)),
      done: AtomicBool::new(false),
    }
  }

  /// Create a new [Mono] that holds an [Err] value.
  pub fn new_error(err: Err) -> Self {
    Self {
      inner: Some(Box::pin(futures::future::ready(Err(err)))),
      done: AtomicBool::new(false),
    }
  }

  /// Create a new [Mono] that holds an [Item].
  pub fn new_success(ok: Item) -> Self {
    Self {
      inner: Some(Box::pin(futures::future::ready(Ok(ok)))),
      done: AtomicBool::new(false),
    }
  }

  /// Push an `Item` to the [Mono]
  pub fn success(&mut self, ok: Item) {
    assert!(self.inner.is_none(), "Can not push more than one value to a Mono");
    self.inner = Some(Box::pin(futures::future::ready(Ok(ok))));
  }

  /// Push an `Error` to the [Mono]
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

impl<Item, Err> Stream for Mono<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe + Sync,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    if self.done.load(Ordering::SeqCst) {
      return Poll::Ready(None);
    }
    let s = self.get_mut();
    match s.inner.as_mut() {
      Some(inner_future) => match inner_future.poll_unpin(cx) {
        Poll::Ready(v) => {
          s.done.store(true, Ordering::SeqCst);
          Poll::Ready(Some(v))
        }
        Poll::Pending => Poll::Pending,
      },
      None => Poll::Pending,
    }
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
    self
      .get_mut()
      .inner
      .as_mut()
      .map_or_else(|| Poll::Pending, |inner_future| inner_future.poll_unpin(cx))
  }
}

#[must_use]
#[allow(missing_debug_implementations)]
/// An implementation of the `Flux` as seen in RSocket and reactive streams. It is similar to a [Stream<Item = Result<Item, Err>>] or an unbounded channel.
pub struct FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  tx: UnboundedSender<Signal<Item, Err>>,
  rx: FluxReceiver<Item, Err>,
}

impl<Item, Err> FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  /// Create a new [Flux]
  pub fn new() -> Self {
    let (tx, rx) = unbounded_channel();

    Self {
      tx,
      rx: FluxReceiver::new(rx),
    }
  }

  /// Create a [Pin<Box<FluxChannel>>] from a [FluxChannel].
  #[must_use]
  pub fn boxed(self) -> Pin<Box<Self>> {
    Box::pin(self)
  }

  /// Create a new [Flux] and return the parts, pre-separated.
  pub fn new_parts() -> (Self, FluxReceiver<Item, Err>) {
    let (tx, rx) = unbounded_channel();

    (
      Self {
        tx,
        rx: FluxReceiver::none(),
      },
      FluxReceiver::new(rx),
    )
  }

  #[must_use]
  /// Check if the [Flux] is complete.
  pub fn is_closed(&self) -> bool {
    self.tx.is_closed()
  }

  #[must_use]
  /// Await the next value in the [Flux].
  pub fn recv(&self) -> FutureResult<Item, Err>
  where
    Err: 'static + std::fmt::Debug,
    Item: 'static + std::fmt::Debug,
  {
    let val = self.rx.recv();
    Box::pin(async move { val.await })
  }

  /// Return and remove the receiving channel from this [Flux].
  pub fn take_rx(&self) -> Result<FluxReceiver<Item, Err>, Error> {
    self.rx.eject().ok_or(Error::ReceiverAlreadyGone)
  }
}

impl<Item, Err> TryFrom<FluxChannel<Item, Err>> for FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  type Error = Error;

  fn try_from(value: FluxChannel<Item, Err>) -> Result<Self, Self::Error> {
    value.take_rx()
  }
}

impl<Item, Err> Clone for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn clone(&self) -> Self {
    Self {
      tx: self.tx.clone(),
      rx: self.rx.clone(),
    }
  }
}

impl<Err> AsyncRead for FluxChannel<Vec<u8>, Err>
where
  Err: ConditionallySafe,
{
  fn poll_read(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
    match Pin::new(&mut self.get_mut().rx).poll_next(cx) {
      Poll::Ready(Some(Ok(item))) => {
        let len = item.len();
        let mut buf = std::io::Cursor::new(buf);
        buf.write_all(&item).unwrap();
        Poll::Ready(Ok(len))
      }
      Poll::Ready(Some(Err(_err))) => Poll::Ready(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        crate::Error::RecvFailed(98),
      ))),
      Poll::Ready(None) => Poll::Ready(Ok(0)),
      Poll::Pending => Poll::Pending,
    }
  }
}

impl<Item, Err> Observable<Item, Err> for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
}

impl<Item, Err> Observer<Item, Err> for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn send_signal(&self, signal: Signal<Item, Err>) -> Result<(), Error> {
    Ok(self.tx.send(signal)?)
  }

  fn is_complete(&self) -> bool {
    self.tx.is_closed()
  }

  fn complete(&self) {
    let _ = self.send_signal(Signal::Complete);
  }
}

impl<Item, Err> Default for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<Item, Err> Stream for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    self.rx.poll_recv(cx)
  }
}

impl<Item, Err> From<Vec<Result<Item, Err>>> for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn from(value: Vec<Result<Item, Err>>) -> Self {
    Self::from_iter(value.into_iter())
  }
}

impl<Item, Err, const N: usize> From<[Result<Item, Err>; N]> for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn from(value: [Result<Item, Err>; N]) -> Self {
    Self::from_iter(value.into_iter())
  }
}

impl<Item, Err> FromIterator<Result<Item, Err>> for FluxChannel<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn from_iter<T: IntoIterator<Item = Result<Item, Err>>>(iter: T) -> Self {
    let (tx, mut rx) = Self::new_parts();
    for item in iter {
      tx.send_result(item);
    }
    tx.complete();
    tx
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
    let mut flux = FluxChannel::<u32, u32>::new();
    flux.send(1)?;
    let value = flux.next().await;
    assert_eq!(value, Some(Ok(1)));
    let stream = flux.take_rx().unwrap();

    flux.send(2)?;
    let value = stream.recv().await?;
    assert_eq!(value, Some(Ok(2)));
    let stream = flux.take_rx();
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
}

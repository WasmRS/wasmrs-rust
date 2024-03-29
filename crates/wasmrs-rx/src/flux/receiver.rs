use std::task::Poll;
use std::{io::Write, pin::Pin};

use futures::Stream;

use super::{signal_into_result, FutureResult, Signal};
use crate::{Error, FluxChannel, Observable, Observer};
use wasmrs_runtime::{ConditionallySendSync, OptionalMut, UnboundedReceiver};

#[must_use]
#[allow(missing_debug_implementations)]
/// The receving end-only of a [crate::Flux]
pub struct FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
  rx: OptionalMut<UnboundedReceiver<Signal<Item, Err>>>,
}

impl<Item, Err> FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
  /// Create a new [FluxReceiver].
  pub fn new(rx: UnboundedReceiver<Signal<Item, Err>>) -> Self {
    Self {
      rx: OptionalMut::new(rx),
    }
  }

  /// Create a [Pin<Box<FluxReceiver>>] from a [FluxReceiver].
  #[must_use]
  pub fn boxed(self) -> Pin<Box<Self>> {
    Box::pin(self)
  }

  /// Create a new [FluxReceiver] that is immediately closed.
  pub fn none() -> Self {
    Self {
      rx: OptionalMut::none(),
    }
  }

  /// Create a new [FluxReceiver] that is immediately closed with the passed item.
  pub fn one<I, E>(item: Result<I, E>) -> FluxReceiver<I, E>
  where
    I: ConditionallySendSync,
    E: ConditionallySendSync,
  {
    let (tx, rx) = FluxChannel::new_parts();
    tx.send_result(item).unwrap();
    rx
  }
}

impl<Err> futures::io::AsyncRead for FluxReceiver<Vec<u8>, Err>
where
  Err: ConditionallySendSync,
{
  fn poll_read(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
    match Pin::new(&mut self.get_mut()).poll_next(cx) {
      Poll::Ready(Some(Ok(item))) => {
        let len = item.len();
        let mut buf = std::io::Cursor::new(buf);
        buf.write_all(&item).unwrap();
        Poll::Ready(Ok(len))
      }
      Poll::Ready(Some(Err(_err))) => Poll::Ready(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        crate::Error::RecvFailed(99),
      ))),
      Poll::Ready(None) => Poll::Ready(Ok(0)),
      Poll::Pending => Poll::Pending,
    }
  }
}

impl<Err> futures::io::AsyncRead for FluxReceiver<bytes::Bytes, Err>
where
  Err: ConditionallySendSync,
{
  fn poll_read(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>, buf: &mut [u8]) -> Poll<std::io::Result<usize>> {
    match Pin::new(&mut self.get_mut()).poll_next(cx) {
      Poll::Ready(Some(Ok(item))) => {
        let len = item.len();
        let mut buf = std::io::Cursor::new(buf);
        buf.write_all(&item).unwrap();
        Poll::Ready(Ok(len))
      }
      Poll::Ready(Some(Err(_err))) => Poll::Ready(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        crate::Error::RecvFailed(99),
      ))),
      Poll::Ready(None) => Poll::Ready(Ok(0)),
      Poll::Pending => Poll::Pending,
    }
  }
}

impl<Item, Err> Clone for FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
  fn clone(&self) -> Self {
    Self { rx: self.rx.clone() }
  }
}

impl<Item, Err> Observable<Item, Err> for FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
}

impl<Item, Err> FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
  #[must_use]
  /// Receive the next value from the [FluxReceiver].
  pub fn recv(&self) -> FutureResult<Item, Err>
  where
    Err: ConditionallySendSync,
    Item: ConditionallySendSync,
  {
    let root_rx = self.rx.clone();
    let opt = root_rx.take();
    Box::pin(async move {
      match opt {
        Some(mut rx) => {
          let signal = rx.recv().await;
          root_rx.insert(rx);
          Ok(signal_into_result(signal))
        }
        None => Err(Error::RecvFailed(0)),
      }
    })
  }

  /// Poll the [FluxReceiver] to see if there is a value available.
  pub fn poll_recv(&self, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<Item, Err>>> {
    let opt = self.rx.take();
    opt.map_or(std::task::Poll::Ready(None), |mut rx| {
      let poll = rx.poll_recv(cx);
      match poll {
        Poll::Ready(Some(Signal::Complete)) => Poll::Ready(None),
        Poll::Ready(Some(Signal::Ok(v))) => {
          self.rx.insert(rx);
          Poll::Ready(Some(Ok(v)))
        }
        Poll::Ready(Some(Signal::Err(e))) => {
          self.rx.insert(rx);
          Poll::Ready(Some(Err(e)))
        }
        Poll::Ready(None) => Poll::Ready(None),
        Poll::Pending => {
          self.rx.insert(rx);
          Poll::Pending
        }
      }
    })
  }

  #[must_use]
  /// Remove the inner channel from the [FluxReceiver]
  pub fn eject(&self) -> Option<Self> {
    self.rx.take().map(|inner| Self {
      rx: OptionalMut::new(inner),
    })
  }
}

impl<Item, Err> Stream for FluxReceiver<Item, Err>
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    self.poll_recv(cx)
  }
}

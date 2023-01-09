use std::pin::Pin;
use std::task::Poll;

use futures::Stream;

use super::{signal_into_result, FutureResult, Signal};
use crate::{Error, Observable};
use wasmrs_runtime::{ConditionallySafe, OptionalMut, UnboundedReceiver};

#[must_use]
#[allow(missing_debug_implementations)]
/// The receving end-only of a [crate::Flux]
pub struct FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  rx: OptionalMut<UnboundedReceiver<Signal<Item, Err>>>,
}

impl<Item, Err> FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  /// Create a new [FluxReceiver].
  pub fn new(rx: UnboundedReceiver<Signal<Item, Err>>) -> Self {
    Self {
      rx: OptionalMut::new(rx),
    }
  }

  /// Create a new [FluxReceiver] that is immediately closed.
  pub fn none() -> Self {
    Self {
      rx: OptionalMut::none(),
    }
  }
}

impl<Item, Err> Clone for FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  fn clone(&self) -> Self {
    Self { rx: self.rx.clone() }
  }
}

impl<Item, Err> Observable<Item, Err> for FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
}

impl<Item, Err> FluxReceiver<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  #[must_use]
  /// Receive the next value from the [FluxReceiver].
  pub fn recv(&self) -> FutureResult<Item, Err>
  where
    Err: ConditionallySafe,
    Item: ConditionallySafe,
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
      self.rx.insert(rx);
      match poll {
        Poll::Ready(Some(Signal::Complete)) => Poll::Ready(None),
        Poll::Ready(Some(Signal::Ok(v))) => Poll::Ready(Some(Ok(v))),
        Poll::Ready(Some(Signal::Err(e))) => Poll::Ready(Some(Err(e))),
        Poll::Ready(None) => Poll::Ready(None),
        Poll::Pending => Poll::Pending,
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
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  type Item = Result<Item, Err>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Option<Self::Item>> {
    self.poll_recv(cx)
  }
}

#[cfg(target_family = "wasm")]
mod wasm;
use std::task::{Context, Poll};

use futures::{Future, FutureExt};
#[cfg(target_family = "wasm")]
pub use wasm::*;

#[cfg(not(target_family = "wasm"))]
mod native;
#[cfg(not(target_family = "wasm"))]
pub use native::*;

use crate::Error;

#[must_use]
/// Create an unbounded channel.
pub fn unbounded_channel<Item>() -> (UnboundedSender<Item>, UnboundedReceiver<Item>)
where
  Item: ConditionallySafe,
{
  let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

  (UnboundedSender(tx), UnboundedReceiver(rx))
}

#[allow(missing_debug_implementations)]
/// A Unbounded Sender that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct UnboundedSender<Item>(tokio::sync::mpsc::UnboundedSender<Item>)
where
  Item: ConditionallySafe;

impl<Item> Clone for UnboundedSender<Item>
where
  Item: ConditionallySafe,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<Item> UnboundedSender<Item>
where
  Item: ConditionallySafe,
{
  /// Send an `Item` to the channel.
  pub fn send(&self, message: Item) -> Result<(), Error> {
    self.0.send(message).map_err(|_| Error::SendFailed(0))
  }

  #[must_use]
  /// Check if the channel is closed.
  pub fn is_closed(&self) -> bool {
    self.0.is_closed()
  }
}

#[allow(missing_debug_implementations)]
/// A Unbounded Receiver that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct UnboundedReceiver<Item>(tokio::sync::mpsc::UnboundedReceiver<Item>)
where
  Item: ConditionallySafe;

impl<Item> UnboundedReceiver<Item>
where
  Item: ConditionallySafe,
{
  /// Receive the next `Item` on the channel.
  pub async fn recv(&mut self) -> Option<Item> {
    self.0.recv().await
  }

  /// Poll the channel to see if an `Item` is ready.
  pub fn poll_recv(&mut self, cx: &mut Context) -> Poll<Option<Item>> {
    self.0.poll_recv(cx)
  }
}

#[must_use]
/// A oneshot channel similar to [tokio::sync::oneshot::channel] but works the same way in single-threaded WebAssembly as multi-threaded native.
pub fn oneshot<Item>() -> (OneShotSender<Item>, OneShotReceiver<Item>)
where
  Item: ConditionallySafe,
{
  let (tx, rx) = tokio::sync::oneshot::channel();

  (OneShotSender(tx), OneShotReceiver(rx))
}

#[allow(missing_debug_implementations)]
/// A Unbounded Sender that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct OneShotSender<Item>(tokio::sync::oneshot::Sender<Item>)
where
  Item: ConditionallySafe;

impl<Item> OneShotSender<Item>
where
  Item: ConditionallySafe,
{
  /// Send an item on the channel.
  pub fn send(self, message: Item) -> Result<(), Error> {
    self.0.send(message).map_err(|_| Error::SendFailed(0))
  }

  #[must_use]
  /// Check if the channel is closed.
  pub fn is_closed(&self) -> bool {
    self.0.is_closed()
  }
}

#[allow(missing_debug_implementations)]
/// A OneShort Receiver that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct OneShotReceiver<Item>(tokio::sync::oneshot::Receiver<Item>)
where
  Item: ConditionallySafe;

impl<Item> OneShotReceiver<Item>
where
  Item: ConditionallySafe,
{
  /// Receive the next item on the channel.
  pub async fn recv(self) -> Result<Item, Error> {
    self.0.await.map_err(|_e| Error::RecvFailed(80))
  }
}

impl<Item> Future for OneShotReceiver<Item>
where
  Item: ConditionallySafe,
{
  type Output = Result<Item, Error>;

  fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let poll = self.get_mut().0.poll_unpin(cx);
    poll.map_err(|_e| Error::RecvFailed(95))
  }
}

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
  Item: ConditionallySendSync,
{
  let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

  (UnboundedSender(tx), UnboundedReceiver(rx))
}

#[allow(missing_debug_implementations)]
/// A Unbounded Sender that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct UnboundedSender<Item>(tokio::sync::mpsc::UnboundedSender<Item>)
where
  Item: ConditionallySendSync;

impl<Item> Clone for UnboundedSender<Item>
where
  Item: ConditionallySendSync,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<Item> UnboundedSender<Item>
where
  Item: ConditionallySendSync,
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
  Item: ConditionallySendSync;

impl<Item> UnboundedReceiver<Item>
where
  Item: ConditionallySendSync,
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

impl<T> futures::Stream for UnboundedReceiver<T>
where
  T: ConditionallySendSync,
{
  type Item = T;

  fn poll_next(mut self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    self.0.poll_recv(cx)
  }
}

#[must_use]
/// A oneshot channel similar to [tokio::sync::oneshot::channel] but works the same way in single-threaded WebAssembly as multi-threaded native.
pub fn oneshot<Item>() -> (OneShotSender<Item>, OneShotReceiver<Item>)
where
  Item: ConditionallySendSync,
{
  let (tx, rx) = tokio::sync::oneshot::channel();

  (OneShotSender(tx), OneShotReceiver(rx))
}

#[allow(missing_debug_implementations)]
/// A Unbounded Sender that works the same way in single-threaded WebAssembly as multi-threaded native.
pub struct OneShotSender<Item>(tokio::sync::oneshot::Sender<Item>)
where
  Item: ConditionallySendSync;

impl<Item> OneShotSender<Item>
where
  Item: ConditionallySendSync,
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
  Item: ConditionallySendSync;

impl<Item> OneShotReceiver<Item>
where
  Item: ConditionallySendSync,
{
  /// Receive the next item on the channel.
  pub async fn recv(self) -> Result<Item, Error> {
    self.0.await.map_err(|_e| Error::RecvFailed(80))
  }
}

impl<Item> Future for OneShotReceiver<Item>
where
  Item: ConditionallySendSync,
{
  type Output = Result<Item, Error>;

  fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let poll = self.get_mut().0.poll_unpin(cx);
    poll.map_err(|_e| Error::RecvFailed(95))
  }
}

impl<T> std::fmt::Debug for MutRc<T>
where
  T: std::fmt::Debug,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("MutRc").field(&self.0).finish()
  }
}

impl<T> Clone for MutRc<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;

  #[test]
  fn test_rc() -> Result<()> {
    let one = RtRc::new("Hello World".to_owned());
    let two = RtRc::new("Hello World".to_owned());

    assert_eq!(one, two);
    Ok(())
  }
}

#[cfg(target_family = "wasm")]
pub mod wasm;
use std::task::{Context, Poll};

#[cfg(target_family = "wasm")]
pub use wasm::*;

#[cfg(not(target_family = "wasm"))]
pub mod native;
#[cfg(not(target_family = "wasm"))]
pub use native::*;

use crate::Error;

#[allow(missing_debug_implementations)]
pub struct Sender<Item>(tokio::sync::mpsc::UnboundedSender<Item>)
where
    Item: ConditionallySafe;

impl<Item> Clone for Sender<Item>
where
    Item: ConditionallySafe,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Item> Sender<Item>
where
    Item: ConditionallySafe,
{
    pub(crate) fn send(&self, message: Item) -> Result<(), Error> {
        self.0.send(message).map_err(|_| Error::SendFailed(0))
    }
    pub(crate) fn is_closed(&self) -> bool {
        self.0.is_closed()
    }
}

#[allow(missing_debug_implementations)]
pub struct Receiver<Item>(tokio::sync::mpsc::UnboundedReceiver<Item>)
where
    Item: ConditionallySafe;

pub(crate) fn channel<Item>() -> (Sender<Item>, Receiver<Item>)
where
    Item: ConditionallySafe,
{
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    (Sender(tx), Receiver(rx))
}

impl<Item> Receiver<Item>
where
    Item: ConditionallySafe,
{
    pub async fn recv(&mut self) -> Option<Item> {
        self.0.recv().await
    }
    pub fn poll_recv(&mut self, cx: &mut Context) -> Poll<Option<Item>> {
        self.0.poll_recv(cx)
    }
}

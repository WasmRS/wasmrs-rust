#[cfg(target_family = "wasm")]
pub mod wasm;
use std::task::{Context, Poll};

#[cfg(target_family = "wasm")]
pub use wasm::*;

#[cfg(not(target_family = "wasm"))]
pub mod native;
#[cfg(not(target_family = "wasm"))]
pub use native::*;

use crate::{flux::Signal, Error};

#[allow(missing_debug_implementations)]
pub struct Sender<Item, Err>(tokio::sync::mpsc::UnboundedSender<Signal<Item, Err>>)
where
    Item: ConditionallySafe,
    Err: ConditionallySafe;

impl<Item, Err> Clone for Sender<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

#[allow(missing_debug_implementations)]
pub struct Receiver<Item, Err>(tokio::sync::mpsc::UnboundedReceiver<Signal<Item, Err>>)
where
    Item: ConditionallySafe,
    Err: ConditionallySafe;

pub(crate) fn channel<Item, Err>() -> (Sender<Item, Err>, Receiver<Item, Err>)
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();

    (Sender(tx), Receiver(rx))
}

impl<Item, Err> Sender<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    pub(crate) fn send(&self, message: Signal<Item, Err>) -> Result<(), Error> {
        self.0.send(message).map_err(|_| Error::SendFailed(0))
    }
}

impl<Item, Err> Receiver<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    pub(crate) async fn recv(&mut self) -> Option<Signal<Item, Err>> {
        self.0.recv().await
    }
    pub(crate) fn poll_recv(&mut self, cx: &mut Context) -> Poll<Option<Signal<Item, Err>>> {
        self.0.poll_recv(cx)
    }
}

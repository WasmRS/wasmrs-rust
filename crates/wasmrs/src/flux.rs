use futures::{pin_mut, Stream, StreamExt, TryStreamExt};
use pin_project_lite::pin_project;
use std::sync::atomic::AtomicBool;
use std::{pin::Pin, task::Poll};

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

impl<Item, Err> From<Box<dyn Stream<Item = Result<Item, Err>>>> for Flux<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    fn from(stream: Box<dyn Stream<Item = Result<Item, Err>>>) -> Self {
        todo!()
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
        self.complete
            .store(false, std::sync::atomic::Ordering::SeqCst);
        self.send_signal(Signal::Complete);
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

impl<Item, Err> Stream for Flux<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    type Item = Result<Item, Err>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
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

    use super::*;
    use anyhow::Result;
    use futures::StreamExt;

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

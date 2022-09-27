mod signal;
pub use signal::*;
mod observer;
use futures_core::Future;
pub use observer::*;
use std::{pin::Pin, task::Poll};

use futures_core::Stream;

type FutureResult<Item, Err> =
    Pin<Box<dyn Future<Output = Result<Option<Result<Item, Err>>, Error>>>>;

use crate::{
    runtime::{channel, ConditionallySafe, OptionalMut, Receiver, Sender},
    Error,
};

pub trait Flux<Item, Err>: Stream<Item = Result<Item, Err>> + ConditionallySafe
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
}

pub type FluxBox<Item, Err> = Pin<Box<dyn Flux<Item, Err>>>;

#[must_use]
#[allow(missing_debug_implementations)]
pub struct FluxChannel<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    tx: Sender<Item, Err>,
    rx: FluxStream<Item, Err>,
}

impl<Item, Err> Flux<Item, Err> for FluxChannel<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
}

impl<Item, Err> FluxChannel<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
            // rx: FluxStream::new(runtime::SafeReceiver::new(rx)),
            rx: FluxStream::new(rx),
        }
    }

    pub fn send(&self, item: Item) -> Result<(), Error> {
        self.tx
            .send(Signal::Ok(item))
            .map_err(|_| Error::SendFailed(0))
    }

    pub fn error(&self, err: Err) -> Result<(), Error> {
        self.tx
            .send(Signal::Err(err))
            .map_err(|_| Error::SendFailed(1))
    }

    pub fn complete(&self) {
        let _ = self.tx.send(Signal::Complete);
    }

    // #[cfg(target_family = "wasm")]
    // #[allow(clippy::type_complexity)]
    // #[must_use]
    // pub fn recv(&self) -> FutureResult<Item, Err> {
    //     let val = self.rx.recv();
    //     Box::pin(async move { val.await })
    // }

    // #[cfg(not(target_family = "wasm"))]
    #[must_use]
    pub fn recv(&self) -> FutureResult<Item, Err>
    where
        Err: 'static,
        Item: 'static,
    {
        let val = self.rx.recv();
        Box::pin(async move { val.await })
    }

    pub fn take_receiver(&self) -> Result<Pin<Box<FluxStream<Item, Err>>>, Error> {
        self.rx
            .take()
            .map(Box::pin)
            .ok_or(Error::ReceiverAlreadyGone)
    }

    #[must_use]
    pub fn clone_box(&self) -> Pin<Box<FluxChannel<Item, Err>>> {
        Box::pin(self.clone())
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

impl<Item, Err> Stream for FluxChannel<Item, Err>
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

#[must_use]
#[allow(missing_debug_implementations)]
pub struct FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    rx: OptionalMut<Receiver<Item, Err>>,
}

impl<Item, Err> Clone for FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
        }
    }
}

impl<Item, Err> Flux<Item, Err> for FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
}

impl<Item, Err> FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    pub fn new(rx: Receiver<Item, Err>) -> Self {
        Self {
            rx: OptionalMut::new(rx),
        }
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

impl<Item, Err> FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    #[must_use]
    pub fn recv(&self) -> FutureResult<Item, Err>
    where
        Err: 'static,
        Item: 'static,
    {
        let root_rx = self.rx.clone();
        let opt = root_rx.take();
        Box::pin(async move {
            match opt {
                Some(mut rx) => {
                    let signal = rx.recv().await;
                    let _ = root_rx.insert(rx);
                    Ok(signal_into_result(signal))
                }
                None => Err(Error::RecvFailed(0)),
            }
        })
    }

    // #[cfg(feature = "tokio")]
    pub fn poll_recv(&self, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<Item, Err>>> {
        let opt = self.rx.take();
        opt.map_or(std::task::Poll::Ready(None), |mut rx| {
            let poll = rx.poll_recv(cx);
            let _ = self.rx.insert(rx);
            match poll {
                Poll::Ready(Some(Signal::Complete)) => Poll::Ready(None),
                Poll::Ready(Some(Signal::Ok(v))) => Poll::Ready(Some(Ok(v))),
                Poll::Ready(Some(Signal::Err(e))) => Poll::Ready(Some(Err(e))),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        })
    }

    #[cfg(feature = "async-channel")]
    pub fn poll_recv(&self, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<Item, Err>>> {
        let opt = self.rx.lock().take();
        opt.map_or(std::task::Poll::Ready(None), |mut rx| {
            let result = rx.try_recv();
            self.rx.lock().insert(rx);

            match result {
                Ok(s) => match s {
                    Signal::Ok(v) => Poll::Ready(Some(Ok(v))),
                    Signal::Err(v) => Poll::Ready(Some(Err(v))),
                    Signal::Complete => Poll::Ready(None),
                },
                Err(e) => Poll::Ready(None),
            }
        })
    }

    #[must_use]
    pub fn take(&self) -> Option<Self> {
        self.rx.take().map(|inner| Self {
            rx: OptionalMut::new(inner),
        })
    }
}

impl<Item, Err> Stream for FluxStream<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    type Item = Result<Item, Err>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.poll_recv(cx)
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {

    use super::*;
    use anyhow::Result;
    use futures::StreamExt;

    async fn recv_flux<Item, Err>(mut flux: FluxBox<Item, Err>) -> Option<Result<Item, Err>>
    where
        Item: ConditionallySafe + 'static,
        Err: ConditionallySafe + 'static,
    {
        // let fut = flux.next();
        let handle = tokio::spawn(async move { flux.next().await });
        let result = handle.await;
        result.unwrap()
    }

    #[tokio::test]
    async fn test_fluxchannel() -> Result<()> {
        let flux = FluxChannel::<u32, u32>::new();
        flux.send(1)?;
        let value = recv_flux(flux.clone_box()).await;
        assert_eq!(value, Some(Ok(1)));
        let stream = flux.take_receiver().unwrap();
        flux.send(2)?;
        let value = recv_flux(stream).await;
        assert_eq!(value, Some(Ok(2)));
        let stream = flux.take_receiver();
        assert!(stream.is_err());
        Ok(())
    }
}

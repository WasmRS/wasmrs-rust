mod signal;
pub use signal::*;
mod observer;
pub use observer::*;
use std::{pin::Pin, sync::Arc, task::Poll};

// use futures_lite::Stream;
use futures_core::Stream;

// pub type Flux<Item, Err> = dyn Send + Stream<Item = Result<Item, Err>>;

use parking_lot::Mutex;

// use async_channel::{unbounded as channel, RecvError};
// pub type Sender<Item, Err> = async_channel::Sender<Signal<Item, Err>>;
// pub type Receiver<Item, Err> = async_channel::Receiver<Signal<Item, Err>>;
pub use tokio::sync::mpsc::unbounded_channel as channel;

use crate::Error;
pub type Sender<Item, Err> = tokio::sync::mpsc::UnboundedSender<Signal<Item, Err>>;
pub type Receiver<Item, Err> = tokio::sync::mpsc::UnboundedReceiver<Signal<Item, Err>>;

pub trait Flux<Item, Err>: Stream<Item = Result<Item, Err>> + Send
where
    Item: Send + Sync,
    Err: Send + Sync,
{
}

pub type FluxBox<Item, Err> = Pin<Box<dyn Flux<Item, Err>>>;

#[must_use]
#[allow(missing_debug_implementations)]
pub struct FluxChannel<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    tx: Sender<Item, Err>,
    rx: FluxStream<Item, Err>,
}

impl<Item, Err> Flux<Item, Err> for FluxChannel<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
}

impl<Item, Err> FluxChannel<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    pub fn new() -> Self {
        let (tx, rx) = channel();
        Self {
            tx,
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

    pub async fn recv(&self) -> Result<Option<Result<Item, Err>>, Error> {
        self.rx.recv().await
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
    Item: Send + Sync,
    Err: Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Item, Err> Clone for FluxChannel<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
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
    Item: Send + Sync,
    Err: Send + Sync,
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
    Item: Send + Sync,
    Err: Send + Sync,
{
    rx: Arc<Mutex<Option<Receiver<Item, Err>>>>,
}

impl<Item, Err> Clone for FluxStream<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
        }
    }
}

impl<Item, Err> Flux<Item, Err> for FluxStream<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
}

impl<Item, Err> FluxStream<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    pub fn new(rx: Receiver<Item, Err>) -> Self {
        Self {
            rx: Arc::new(Mutex::new(Some(rx))),
        }
    }
}

fn signal_into_result<Item, Err>(signal: Option<Signal<Item, Err>>) -> Option<Result<Item, Err>>
where
    Item: Send,
    Err: Send,
{
    match signal {
        Some(Signal::Complete) => None,
        Some(Signal::Ok(v)) => Some(Ok(v)),
        Some(Signal::Err(e)) => Some(Err(e)),
        None => None,
    }
}

#[cfg(feature = "async-channel")]
fn result_signal_into_result<Item, Err>(
    signal: Result<Signal<Item, Err>, RecvError>,
) -> Option<Result<Item, Err>>
where
    Item: Send,
    Err: Send,
{
    match signal {
        Ok(Signal::Complete) => None,
        Ok(Signal::Ok(v)) => Some(Ok(v)),
        Ok(Signal::Err(e)) => Some(Err(e)),
        Err(_) => None,
    }
}
impl<Item, Err> FluxStream<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    pub async fn recv(&self) -> Result<Option<Result<Item, Err>>, Error> {
        let opt = self.rx.lock().take();
        match opt {
            Some(mut rx) => {
                let signal = rx.recv().await;
                let _ = self.rx.lock().insert(rx);
                Ok(signal_into_result(signal))
            }
            None => Err(Error::RecvFailed(0)),
        }
    }

    // #[cfg(feature = "tokio")]
    pub fn poll_recv(&self, cx: &mut std::task::Context<'_>) -> Poll<Option<Result<Item, Err>>> {
        let opt = self.rx.lock().take();
        opt.map_or(std::task::Poll::Ready(None), |mut rx| {
            let poll = rx.poll_recv(cx);
            let _ = self.rx.lock().insert(rx);
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
        self.rx.lock().take().map(|inner| Self {
            rx: Arc::new(Mutex::new(Some(inner))),
        })
    }
}

impl<Item, Err> Stream for FluxStream<Item, Err>
where
    Item: Send + Sync,
    Err: Send + Sync,
{
    type Item = Result<Item, Err>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.poll_recv(cx)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use futures::StreamExt;

    async fn recv_flux<Item, Err>(mut flux: FluxBox<Item, Err>) -> Option<Result<Item, Err>>
    where
        Item: Send + Sync + 'static,
        Err: Send + Sync + 'static,
    {
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

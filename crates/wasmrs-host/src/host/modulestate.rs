use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Arc;

use parking_lot::Mutex;
use rxrust::ops::box_it::BoxOp;
use rxrust::ops::ref_count::RefCount;
use rxrust::prelude::*;
use rxrust::rc::MutArc;
use rxrust::shared::{Shared, SharedObservable};
use rxrust::subject::SharedSubject;
use rxrust::subscription::{SingleSubscription, SubscriptionLike, SubscriptionWrapper};
use tracing::instrument::WithSubscriber;
use wasmrs_rsocket::Frame;

use crate::errors::Error;
use crate::{AsyncHostCallback, HostCallback, Invocation};
pub type OutgoingStream = SharedSubject<Vec<u8>, ()>;
pub type ShareableStream = ConnectableObservable<OutgoingStream, OutgoingStream>;

pub struct StreamState(OutgoingStream, ());
// pub type OutgoingStream = LocalSubject<'static, Vec<u8>, ()>;

#[allow(missing_debug_implementations)]
#[derive(Default)]
/// Module state is essentially a 'handle' that is passed to a runtime engine to allow it
/// to read and write relevant data as different low-level functions are executed during
/// a wasmRS conversation
pub struct ModuleState {
    pub(super) streams: Arc<Mutex<HashMap<u32, StreamState>>>,
    pub(super) host_buffer_size: u32,
    pub(super) host_buffer_start: AtomicU32,
    pub(super) guest_buffer_size: u32,
    pub(super) guest_buffer_start: AtomicU32,
    pub(super) guest_buffer_pos: AtomicU32,
    pub(super) stream_index: AtomicU32,
    pub(super) id: u64,
}

impl std::fmt::Debug for ModuleState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleState")
            .field("# pending streams", &self.streams.lock().len())
            .field("stream_index", &self.stream_index)
            .field("id", &self.id)
            .finish()
    }
}

impl ModuleState {
    pub(crate) fn new(id: u64) -> ModuleState {
        ModuleState {
            streams: Arc::new(Mutex::new(HashMap::new())),
            host_buffer_size: 1024,
            host_buffer_start: AtomicU32::new(0),
            guest_buffer_size: 1024,
            guest_buffer_start: AtomicU32::new(0),
            guest_buffer_pos: AtomicU32::new(0),
            stream_index: AtomicU32::new(1),
            id,
        }
    }

    pub fn get_host_buffer_size(&self) -> u32 {
        self.host_buffer_size
    }

    pub fn get_host_buffer_start(&self) -> u32 {
        self.host_buffer_start.load(Ordering::Relaxed)
    }

    pub fn get_guest_buffer_size(&self) -> u32 {
        self.guest_buffer_size
    }

    pub fn get_guest_buffer_start(&self) -> u32 {
        self.guest_buffer_start.load(Ordering::Relaxed)
    }

    pub fn get_guest_buffer_pos(&self) -> u32 {
        self.guest_buffer_pos.load(Ordering::Relaxed)
    }

    pub fn update_guest_buffer_pos(&self, position: u32) {
        self.guest_buffer_pos.store(position, Ordering::Relaxed);
    }

    pub(crate) fn new_stream(&self) -> (u32, ShareableStream) {
        let id = self.stream_index.fetch_add(2, Ordering::SeqCst);
        trace!(module_id = self.id, id, "initializing new stream");
        let mut lock = self.streams.lock();
        let stream = OutgoingStream::default();

        let state = StreamState(stream.clone(), ());

        lock.insert(id, state);
        (id, stream.publish())
    }

    pub(crate) fn get_stream(&self, stream_id: u32) -> Option<OutgoingStream> {
        trace!(module_id = self.id, stream_id, "getting stream");
        let mut lock = self.streams.lock();

        lock.get(&stream_id).map(|v| v.0.clone())
    }
}

impl ModuleState {
    /// Invoked after a guest has completed its initialization.
    pub fn do_host_init(
        &self,
        guest_buff_ptr: u32,
        host_buff_ptr: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.stream_index.fetch_add(1, Ordering::SeqCst);
        trace!(
            module_id = self.id,
            id,
            guest_buff_ptr,
            host_buff_ptr,
            "do_host_init"
        );
        self.host_buffer_start
            .store(host_buff_ptr, Ordering::SeqCst);
        self.guest_buffer_start
            .store(guest_buff_ptr, Ordering::SeqCst);
        Ok(())
    }

    /// Invoked when the guest module wishes to send a stream frame to the host.
    pub fn do_host_send(&self, frame_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.stream_index.fetch_add(1, Ordering::SeqCst);
        trace!(module_id = self.id, id, ?frame_bytes, "do_host_send");
        match Frame::decode(frame_bytes) {
            Ok(frame) => match frame {
                Frame::Payload(frame) => {
                    let mut stream = self
                        .get_stream(frame.stream_id)
                        .ok_or(Error::StreamNotFound(frame.stream_id))?;
                    stream.next(frame.data);
                }
                Frame::Cancel(_) => todo!(),
                Frame::ErrorFrame(_) => todo!(),
                Frame::RequestN(_) => todo!(),
                Frame::RequestResponse(_) => todo!(),
                Frame::FireAndForget(_) => todo!(),
                Frame::RequestStream(_) => todo!(),
                Frame::RequestChannel(_) => todo!(),
            },
            Err((stream_id, err)) => {
                let stream = self.get_stream(stream_id);
                return Err(Box::new(err));
            }
        }
        Ok(())
    }

    /// Invoked when the guest module wants to write a message to the host's `stdout`
    pub fn do_console_log(&self, msg: &str) {
        trace!(id = self.id, msg, "guest console_log");
        println!("{}", msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use futures_core::future::BoxFuture;

    #[test]
    fn test_basic() -> Result<()> {
        let mut i = Vec::new();
        let mut i2 = Vec::new();

        let mut stream: OutgoingStream = OutgoingStream::new();
        let sharable = stream.clone().publish().into_ref_count();

        stream.next(vec![1]);
        sharable.clone().into_shared().subscribe(move |v| {
            println!("first: {:?}", v);
            i.extend(vec![0]);
        });
        sharable.clone().into_shared().subscribe(move |v| {
            println!("second: {:?}", v);
            i2.extend(vec![0]);
        });
        stream.next(vec![2]);

        Ok(())
    }

    #[test]
    fn test_other() -> Result<()> {
        let mut accept1 = 0;
        let mut accept2 = 0;
        {
            let ref_count = of(1).publish::<LocalSubject<'_, _, _>>().into_ref_count();
            ref_count.clone().subscribe(|v| {
                println!("other first");
                accept1 = v;
            });
            ref_count.clone().subscribe(|v| {
                println!("other second");
                accept2 = v;
            });
            ref_count.clone().subscribe(|v| {
                println!("other third");
            });
        }

        assert_eq!(accept1, 1);
        assert_eq!(accept2, 0);
        Ok(())
    }

    fn make_async_observable<T, E>(
        f: impl FnOnce(SharedSubject<T, E>) -> BoxFuture<'static, ()>,
    ) -> SharedSubject<T, E> {
        let subject = SharedSubject::default();
        tokio::spawn((f)(subject.clone()));
        subject
    }

    #[tokio::test]
    async fn test_map() -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let subject = make_async_observable(|mut sub| {
            Box::pin(async move {
                while let Some(v) = rx.recv().await {
                    sub.next(v);
                }
                sub.complete();
            })
        });

        tx.send(1);
        tx.send(2);
        let m = subject.clone().map(|v| {
            println!("1: is {:?}", v);
            v
        });
        m.into_shared().subscribe(|v| {
            println!("in subscribe, got {:?}", v);
        });
        tx.send(3);
        tx.send(4);
        drop(tx);

        // ref to ref can fork
        let m = of(&1).map(|v| v);
        m.map(|v| v).into_shared().subscribe(|_| {
            println!("sub");
        });
        let mut subject = from_iter(vec![1, 2, 3]);
        // subject.next(1);
        // subject.next(2);
        // subject.next(3);
        subject.map(|v| {
            println!("first: {}", v);
            v * 2
        });

        // subject.clone().map(|v| {
        //     println!("second: {}", v);
        //     v * 2
        // });
        tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        println!("wtf");
        // subject.next(10);
        // subject.next(20);
        // subject.next(30);
        // subject.error(());

        Ok(())
    }
}

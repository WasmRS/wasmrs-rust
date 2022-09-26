use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};
use wasmrs_rsocket::Payload;

use bytes::Bytes;
use parking_lot::Mutex;
use rxrust::ops::box_it::BoxOp;
use rxrust::ops::ref_count::RefCount;
use rxrust::prelude::*;
use rxrust::rc::MutArc;
use rxrust::shared::{Shared, SharedObservable};
use rxrust::subject::SharedSubject;
use rxrust::subscription::{SingleSubscription, SubscriptionLike, SubscriptionWrapper};
use tracing::instrument::WithSubscriber;
use wasmrs_rsocket::{Counter, Frame};

use crate::errors::Error;
use crate::{AsyncHostCallback, HostCallback, Invocation};

pub struct StreamState(Handler, ());
// pub type OutgoingStream = LocalSubject<'static, Vec<u8>, ()>;

pub type OptionalResult = Result<Option<Payload>, wasmrs_rsocket::PayloadError>;
pub type StreamResult = Result<Payload, wasmrs_rsocket::PayloadError>;

#[allow(missing_debug_implementations)]
pub enum Handler {
    ReqRR(oneshot::Sender<OptionalResult>),
    ResRRn(Counter),
    ReqRS(mpsc::UnboundedSender<StreamResult>),
    ReqRC(mpsc::UnboundedSender<StreamResult>),
}

#[allow(missing_debug_implementations)]
#[derive(Default)]
/// Module state is essentially a 'handle' that is passed to a runtime engine to allow it
/// to read and write relevant data as different low-level functions are executed during
/// a wasmRS conversation
pub struct ModuleState {
    pub(super) streams: Arc<DashMap<u32, Handler>>,
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
            .field("# pending streams", &self.streams.len())
            .field("stream_index", &self.stream_index)
            .field("id", &self.id)
            .finish()
    }
}

impl ModuleState {
    pub(crate) fn new(id: u64) -> ModuleState {
        ModuleState {
            streams: Arc::new(DashMap::new()),
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

    pub(crate) fn new_stream(&self, handler: Handler) -> (u32) {
        let id = self.stream_index.fetch_add(2, Ordering::SeqCst);
        trace!(module_id = self.id, id, "initializing new stream");
        self.streams.insert(id, handler);
        id
    }

    pub(crate) fn take_stream(&self, stream_id: u32) -> Option<Handler> {
        trace!(module_id = self.id, stream_id, "getting stream");
        self.streams.remove(&stream_id).map(|v| v.1)
    }

    pub(crate) fn kick_handler(&self, stream_id: u32, result: OptionalResult) -> Result<(), Error> {
        trace!(module_id = self.id, stream_id, "kicking handler");

        match self.streams.remove(&stream_id) {
            Some((id, handler)) => match handler {
                Handler::ReqRR(h) => h.send(result).map_err(|_| Error::StreamSend)?,
                Handler::ResRRn(h) => todo!(),
                Handler::ReqRS(h) => {
                    h.send(result.map(|v| v.unwrap()))
                        .map_err(|_| Error::StreamSend)?;
                    self.streams.insert(id, Handler::ReqRS(h));
                }
                Handler::ReqRC(h) => {
                    h.send(result.map(|v| v.unwrap()))
                        .map_err(|_| Error::StreamSend)?;
                    self.streams.insert(id, Handler::ReqRS(h));
                }
            },
            None => return Err(Error::StreamNotFound(stream_id)),
        }
        Ok(())
    }

    pub fn insert_handler(&self, stream_id: u32, handler: Handler) -> Option<Handler> {
        trace!(module_id = self.id, stream_id, "inserting handler");
        todo!()
        // self.streams.get(&stream_id).map(|v| v.0)
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
    pub fn do_host_send(&self, frame_bytes: Bytes) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.stream_index.fetch_add(1, Ordering::SeqCst);
        trace!(module_id = self.id, id, ?frame_bytes, "do_host_send");
        match Frame::decode(frame_bytes) {
            Ok(frame) => self.kick_handler(frame.stream_id(), frame.into())?,
            Err((stream_id, err)) => {
                let stream = self.take_stream(stream_id);
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

    // #[tokio::test]
    // async fn test_stream() -> Result<()> {
    //     let state = ModuleState::new(0);
    //     let (stream_id, out_stream) = state.new_stream();
    //     let mut stream = state.take_stream(stream_id).unwrap();
    //     stream.next(vec![0, 1, 2]);
    //     stream.complete();

    //     let i = Arc::new(AtomicU32::new(0));
    //     let i2 = i.clone();

    //     let sub = out_stream
    //         .map(|v| v.iter().map(|v| v * 2).collect::<Vec<_>>())
    //         .into_shared()
    //         .subscribe(move |v| {
    //             for v in v {
    //                 i2.fetch_add(v as u32, Ordering::Relaxed);
    //             }
    //         });
    //     sub.await;
    //     assert_eq!(i.load(Ordering::Relaxed), 4);

    //     Ok(())
    // }
}

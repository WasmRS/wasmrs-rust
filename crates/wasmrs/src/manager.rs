#![allow(missing_debug_implementations)]
use crate::runtime::SafeMap;
use crate::{fragmentation::Joiner, frames::RSocketFlags, Frame};

use std::collections::HashMap;
use std::sync::atomic::{AtomicI32, AtomicU32, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot};

use bytes::Bytes;
use parking_lot::Mutex;

use crate::{Counter, Error, Payload};

pub struct StreamState(Handler, ());
// pub type OutgoingStream = LocalSubject<'static, Vec<u8>, ()>;

pub type OptionalResult = Result<Option<Payload>, crate::PayloadError>;
pub type StreamResult = Result<Payload, crate::PayloadError>;

pub enum Handler {
    ReqRR(oneshot::Sender<OptionalResult>),
    ResRRn(Counter),
    ReqRS(mpsc::UnboundedSender<StreamResult>),
    ReqRC(mpsc::UnboundedSender<StreamResult>),
}

pub struct BufferState {
    size: u32,
    start: AtomicU32,
    read_pos: AtomicU32,
}

impl Default for BufferState {
    fn default() -> Self {
        Self {
            size: 1024,
            start: Default::default(),
            read_pos: Default::default(),
        }
    }
}

impl BufferState {
    pub fn get_size(&self) -> u32 {
        self.size
    }

    pub fn get_start(&self) -> u32 {
        self.start.load(Ordering::Relaxed)
    }

    pub fn update_start(&self, position: u32) {
        self.start.store(position, Ordering::SeqCst);
    }

    pub fn get_pos(&self) -> u32 {
        self.read_pos.load(Ordering::Relaxed)
    }

    pub fn update_post(&self, position: u32) {
        self.read_pos.store(position, Ordering::Relaxed);
    }
}

#[derive(Default)]
#[must_use]
pub struct SocketManager {
    pub(super) streams: Arc<SafeMap<u32, Handler>>,
    host_buffer: BufferState,
    guest_buffer: BufferState,
    pub(super) stream_index: AtomicU32,
}

impl std::fmt::Debug for SocketManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleState")
            .field("# pending streams", &self.streams.len())
            .field("stream_index", &self.stream_index)
            .finish()
    }
}

impl SocketManager {
    pub fn new() -> SocketManager {
        SocketManager {
            stream_index: AtomicU32::new(1),
            ..Default::default()
        }
    }
    pub fn host_buffer(&self) -> &BufferState {
        &self.host_buffer
    }
    pub fn guest_buffer(&self) -> &BufferState {
        &self.guest_buffer
    }

    pub fn new_stream(&self, handler: Handler) -> (u32) {
        let id = self.stream_index.fetch_add(2, Ordering::SeqCst);
        self.streams.insert(id, handler);
        id
    }

    pub fn take_stream(&self, stream_id: u32) -> Option<Handler> {
        self.streams.remove(&stream_id)
    }

    pub fn kick_handler(&self, stream_id: u32, result: OptionalResult) -> Result<(), Error> {
        match self.streams.remove(&stream_id) {
            Some(handler) => match handler {
                Handler::ReqRR(h) => h.send(result).map_err(|_| Error::StreamSend)?,
                Handler::ResRRn(h) => todo!(),
                Handler::ReqRS(h) => {
                    h.send(result.map(|v| v.unwrap()))
                        .map_err(|_| Error::StreamSend)?;
                    self.streams.insert(stream_id, Handler::ReqRS(h));
                }
                Handler::ReqRC(h) => {
                    h.send(result.map(|v| v.unwrap()))
                        .map_err(|_| Error::StreamSend)?;
                    self.streams.insert(stream_id, Handler::ReqRS(h));
                }
            },
            None => return Err(Error::StreamNotFound(stream_id)),
        }
        Ok(())
    }

    /// Invoked after a guest has completed its initialization.
    pub fn do_host_init(
        &self,
        guest_buff_ptr: u32,
        host_buff_ptr: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.stream_index.fetch_add(1, Ordering::SeqCst);

        self.host_buffer().update_start(host_buff_ptr);
        self.guest_buffer().update_start(guest_buff_ptr);
        Ok(())
    }

    /// Invoked when the guest module wishes to send a stream frame to the host.
    pub fn do_host_send(&self, frame_bytes: Bytes) -> Result<(), Box<dyn std::error::Error>> {
        let id = self.stream_index.fetch_add(1, Ordering::SeqCst);
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
        println!("{}", msg);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;
    use futures_core::future::BoxFuture;
}

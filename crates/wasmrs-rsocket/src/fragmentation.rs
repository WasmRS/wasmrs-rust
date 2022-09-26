use std::collections::LinkedList;

use bytes::{Bytes, BytesMut};

use crate::{frames::FRAME_FLAG_FOLLOWS, generated::Payload, Frame};

pub const MIN_MTU: usize = 64;

#[allow(missing_debug_implementations)]
#[must_use]
pub struct Joiner {
    inner: LinkedList<Frame>,
}

#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Splitter {
    mtu: usize,
}

impl Splitter {
    pub fn new(mtu: usize) -> Splitter {
        assert!(mtu > Frame::LEN_HEADER, "mtu is too small!");
        Splitter { mtu }
    }

    pub fn cut(&self, input: Payload, skip: usize) -> impl Iterator<Item = Payload> {
        SplitterIter {
            mtu: self.mtu,
            skip,
            data: input.data,
            meta: input.metadata,
        }
    }
}

struct SplitterIter {
    mtu: usize,
    skip: usize,
    data: Option<Bytes>,
    meta: Option<Bytes>,
}

impl Iterator for SplitterIter {
    type Item = Payload;

    fn next(&mut self) -> Option<Payload> {
        if self.meta.is_none() && self.data.is_none() {
            return None;
        }
        let mut m: Option<Bytes> = None;
        let mut d: Option<Bytes> = None;
        let mut left = self.mtu - Frame::LEN_HEADER - self.skip;
        if let Some(it) = &mut self.meta {
            let msize = it.len();
            if left < msize {
                m = Some(it.split_to(left));
                left = 0;
            } else {
                m = self.meta.take();
                left -= msize;
            }
        }

        if left > 0 {
            if let Some(it) = &mut self.data {
                let dsize = it.len();
                if left < dsize {
                    d = Some(it.split_to(left));
                } else {
                    d = self.data.take();
                }
            }
        }
        self.skip = 0;
        Some(Payload::new_optional(m, d))
    }
}

impl Into<Payload> for Joiner {
    fn into(self) -> Payload {
        let mut data_buff = BytesMut::new();
        let mut md_buff = BytesMut::new();
        self.inner.into_iter().for_each(|frame: Frame| {
            let (d, m) = match frame {
                Frame::RequestResponse(frame) => (Some(frame.0.data), Some(frame.0.metadata)),
                Frame::RequestStream(frame) => (Some(frame.0.data), Some(frame.0.metadata)),
                Frame::RequestChannel(frame) => (Some(frame.0.data), Some(frame.0.metadata)),
                Frame::RequestFnF(frame) => (Some(frame.0.data), Some(frame.0.metadata)),
                Frame::PayloadFrame(frame) => (Some(frame.data), Some(frame.metadata)),
                _ => (None, None),
            };
            if let Some(raw) = d {
                data_buff.extend(raw);
            }
            if let Some(raw) = m {
                md_buff.extend(raw);
            }
        });

        let data = if data_buff.is_empty() {
            None
        } else {
            Some(data_buff.freeze())
        };
        let metadata = if md_buff.is_empty() {
            None
        } else {
            Some(md_buff.freeze())
        };
        Payload::new_optional(data, metadata)
    }
}

impl Joiner {
    pub fn new() -> Joiner {
        Joiner {
            inner: LinkedList::new(),
        }
    }

    #[must_use]
    pub fn get_stream_id(&self) -> u32 {
        self.first().stream_id()
    }

    #[must_use]
    pub fn get_flag(&self) -> u16 {
        self.first().get_flag() & !FRAME_FLAG_FOLLOWS
    }

    pub fn first(&self) -> &Frame {
        #[allow(clippy::expect_used)]
        self.inner.front().expect("No frames pushed!")
    }

    pub fn push(&mut self, next: Frame) {
        self.inner.push_back(next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        fragmentation::{Joiner, Splitter},
        generated::Payload,
        Frame,
    };

    #[test]
    fn test_joiner() {
        let first = Frame::new_payload(
            1,
            Payload::new(Bytes::from("(ROOT)"), Bytes::from("(ROOT)")),
            Frame::FLAG_FOLLOW,
        );
        let mut joiner = Joiner::new();
        joiner.push(first);

        for i in 0..10 {
            let flag = if i == 9 { 0u16 } else { Frame::FLAG_FOLLOW };
            let next = Frame::new_payload(
                1,
                Payload::new(
                    Bytes::from(format!("(data{:04})", i)),
                    Bytes::from(format!("(data{:04})", i)),
                ),
                Frame::FLAG_FOLLOW,
            );
            joiner.push(next);
        }
        let pa: Payload = joiner.into();
        println!("payload: {:?}", pa);
    }

    #[test]
    fn test_splitter() {
        let input =
            Payload::new_optional(Some(Bytes::from("foobar")), Some(Bytes::from("helloworld")));
        let mut sp = Splitter::new(13);
        for (i, it) in sp.cut(input.clone(), 0).enumerate() {
            println!("{}: {:?}", i, it);
        }
        println!("MODE 100");
        sp = Splitter::new(100);
        for (i, it) in sp.cut(input, 0).enumerate() {
            println!("{}: {:?}", i, it);
        }
    }
}

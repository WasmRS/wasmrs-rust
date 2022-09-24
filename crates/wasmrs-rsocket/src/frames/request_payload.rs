use crate::{
    generated::{FrameFlags, FrameHeader, FrameType},
    util::{from_u24_bytes, from_u32_bytes, to_u24_bytes},
    Frame,
};

use super::{Error, FRAME_FLAG_COMPLETE, FRAME_FLAG_METADATA};
use bytes::{BufMut, Bytes, BytesMut};

pub use crate::generated::RequestPayload;

impl RequestPayload {
    pub fn new(stream_id: u32, frame_type: FrameType, data: Bytes, metadata: Bytes) -> Self {
        Self {
            frame_type,
            stream_id,
            metadata,
            data,
            follows: false,
            complete: false,
            initial_n: 0,
        }
    }

    pub(super) fn get_flags(&self) -> FrameFlags {
        let mut flags = 0;
        if !self.metadata.is_empty() {
            flags |= FRAME_FLAG_METADATA;
        }
        if self.complete {
            flags |= FRAME_FLAG_COMPLETE;
        }
        if self.frame_type == FrameType::RequestChannel {
            flags |= FRAME_FLAG_COMPLETE;
        }
        flags
    }

    pub fn decode(header: FrameHeader, mut buffer: Bytes) -> Result<RequestPayload, Error> {
        let frame_type = header.frame_type();

        let initial_n = if Self::is_multi(frame_type) {
            from_u32_bytes(&buffer.split_to(4))
        } else {
            0
        };

        let metadata_len = if header.has_metadata() {
            from_u24_bytes(&buffer.split_to(3)) as usize
        } else {
            0
        };

        let metadata: Bytes = buffer.split_to(metadata_len);
        let payload: Bytes = buffer;

        Ok(RequestPayload {
            frame_type,
            stream_id: header.stream_id(),
            metadata,
            data: payload,
            follows: header.has_follows(),
            complete: header.has_complete(),
            initial_n,
        })
    }

    fn is_multi(frame_type: FrameType) -> bool {
        matches!(
            frame_type,
            FrameType::RequestChannel | FrameType::RequestStream
        )
    }

    pub(crate) fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, self.frame_type, self.get_flags())
    }

    #[must_use]
    pub fn encode(self) -> Bytes {
        let header = self.gen_header().encode();
        let n_bytes = if Self::is_multi(self.frame_type) {
            self.initial_n.to_be_bytes().to_vec()
        } else {
            Vec::new()
        };
        let mlen = to_u24_bytes(self.metadata.len() as u32);
        let md = self.metadata;
        let data = self.data;
        let frame_len = Frame::LEN_HEADER + n_bytes.len() + mlen.len() + md.len() + data.len();
        println!("frame len : {}", frame_len);
        let mut bytes = BytesMut::with_capacity(frame_len);
        bytes.put(header);
        bytes.put(n_bytes.as_slice());
        bytes.put(mlen);
        bytes.put(md);
        bytes.put(data);
        bytes.freeze()
    }
}

#[cfg(test)]
mod test {
    // Tested in the request* frames
}

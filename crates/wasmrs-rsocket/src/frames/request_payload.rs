use crate::{
    generated::{FrameFlags, FrameHeader, FrameType},
    util::{from_u24_bytes, from_u32_bytes, to_u24_bytes},
};

use super::{Error, FRAME_FLAG_COMPLETE, FRAME_FLAG_METADATA};

pub use crate::generated::RequestPayload;

impl RequestPayload {
    fn get_flags(&self) -> FrameFlags {
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
}

impl RequestPayload {
    pub fn new(stream_id: u32, frame_type: FrameType, data: Vec<u8>, metadata: Vec<u8>) -> Self {
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

    pub fn decode(header: FrameHeader, mut buffer: Vec<u8>) -> Result<RequestPayload, Error> {
        let frame_type = header.frame_type();
        let mut start = 6;

        let initial_n = if Self::is_multi(frame_type) {
            let len = from_u32_bytes(&buffer[start..start + 4]);
            start += 4;
            len
        } else {
            0
        };

        let metadata_len = if header.has_metadata() {
            let bytes = &buffer[start..start + 3];
            start += 3;
            from_u24_bytes(bytes) as usize
        } else {
            0
        };

        let data_start = start + metadata_len;
        let metadata_range = start..(start + metadata_len);

        let payload_range = (data_start)..(buffer.len());

        let payload: Vec<u8> = buffer.drain(payload_range).collect();
        let metadata: Vec<u8> = buffer.drain(metadata_range).collect();

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
    pub fn encode(self) -> Vec<u8> {
        let header = self.gen_header().encode();

        let n_bytes = if Self::is_multi(self.frame_type) {
            (self.initial_n).to_be_bytes().to_vec()
        } else {
            Vec::new()
        };
        [
            header,
            n_bytes,
            to_u24_bytes(self.metadata.len() as u32),
            self.metadata,
            self.data,
        ]
        .concat()
    }
}

#[cfg(test)]
mod test {
    // Tested in the request* frames
}

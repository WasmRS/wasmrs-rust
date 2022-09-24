use super::{
    Error, FrameCodec, FRAME_FLAG_COMPLETE, FRAME_FLAG_FOLLOWS, FRAME_FLAG_METADATA,
    FRAME_FLAG_NEXT,
};

pub use crate::generated::Payload;
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    generated::{FrameFlags, FrameHeader, FrameType},
    util::{from_u24_bytes, to_u24_bytes},
    Frame,
};

impl Payload {}

impl FrameCodec<Payload> for Payload {
    const FRAME_TYPE: FrameType = FrameType::Payload;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode(mut buffer: Bytes) -> Result<Payload, Error> {
        let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
        Self::check_type(&header)?;
        let mut start = Frame::LEN_HEADER;

        let metadata_len = if header.has_metadata() {
            from_u24_bytes(&buffer.split_to(3)) as usize
        } else {
            0
        };

        let data_start = start + metadata_len;
        let metadata_range = start..(start + metadata_len);

        let payload_range = (data_start)..(buffer.len());

        let metadata: Bytes = buffer.split_to(metadata_len);
        let payload: Bytes = buffer;

        Ok(Payload {
            stream_id: header.stream_id(),
            metadata,
            data: payload,
            follows: header.has_follows(),
            complete: header.has_complete(),
            next: header.has_next(),
        })
    }

    fn encode(self) -> Bytes {
        let header = self.gen_header().encode();
        let mlen = to_u24_bytes(self.metadata.len() as u32);
        let md = self.metadata;
        let data = self.data;
        let mut bytes =
            BytesMut::with_capacity(Frame::LEN_HEADER + mlen.len() + md.len() + data.len());
        bytes.put(header);
        bytes.put(mlen);
        bytes.put(md);
        bytes.put(data);
        bytes.freeze()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Payload, self.get_flags())
    }

    fn get_flags(&self) -> FrameFlags {
        let mut flags = 0;
        if !self.metadata.is_empty() {
            flags |= Frame::FLAG_METADATA;
        }
        if self.complete {
            flags |= Frame::FLAG_COMPLETE;
        }
        if self.next {
            flags |= Frame::FLAG_NEXT;
        }
        if self.follows {
            flags |= Frame::FLAG_FOLLOW;
        }
        flags
    }
}

#[cfg(test)]
mod test {
    use crate::frames::FrameCodec;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.payload.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW: {:?}", BYTES);
        let p = Payload::decode(BYTES.into())?;
        assert_eq!(p.stream_id, 1234);
        assert_eq!(p.data, Bytes::from("hello"));
        assert_eq!(p.metadata, Bytes::from("hello"));
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = Payload {
            stream_id: 1234,
            metadata: Bytes::from("hello"),
            data: Bytes::from("hello"),
            follows: true,
            complete: true,
            next: true,
        };
        let encoded = payload.encode();
        assert_eq!(encoded, Bytes::from(BYTES));
        Ok(())
    }
}

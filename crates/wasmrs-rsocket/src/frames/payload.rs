use super::{
    Error, FrameCodec, FRAME_FLAG_COMPLETE, FRAME_FLAG_FOLLOWS, FRAME_FLAG_METADATA,
    FRAME_FLAG_NEXT,
};

pub use crate::generated::Payload;

use crate::{
    generated::{FrameFlags, FrameHeader, FrameType},
    util::{from_u24_bytes, to_u24_bytes},
};

impl Payload {
    fn get_flags(&self) -> FrameFlags {
        (if self.metadata.is_empty() {
            0
        } else {
            FRAME_FLAG_METADATA
        }) | if self.complete {
            FRAME_FLAG_COMPLETE
        } else {
            0
        } | if self.next { FRAME_FLAG_NEXT } else { 0 }
            | if self.follows { FRAME_FLAG_FOLLOWS } else { 0 }
    }
}

impl FrameCodec<Payload> for Payload {
    const FRAME_TYPE: FrameType = FrameType::Payload;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode(mut buffer: Vec<u8>) -> Result<Payload, Error> {
        let header = FrameHeader::from_reader(&*buffer)?;
        Self::check_type(&header)?;
        let mut start = 6;

        let metadata_len = if header.has_metadata() {
            let len = from_u24_bytes(&buffer[start..start + 3]);
            start += 3;
            len
        } else {
            0
        };

        let data_start = start + metadata_len as usize;
        let metadata_range = start..(start + metadata_len as usize);
        let payload_range = (data_start)..(buffer.len());

        let payload: Vec<u8> = buffer.drain(payload_range).collect();
        let metadata: Vec<u8> = buffer.drain(metadata_range).collect();
        Ok(Payload {
            stream_id: header.stream_id(),
            metadata,
            data: payload,
            follows: header.has_follows(),
            complete: header.has_complete(),
            next: header.has_next(),
        })
    }

    fn encode(self) -> Vec<u8> {
        let header = self.gen_header().encode();
        [
            header,
            to_u24_bytes(self.metadata.len() as u32),
            self.metadata,
            self.data,
        ]
        .concat()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Payload, self.get_flags())
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
        let p = Payload::decode(BYTES.to_vec())?;
        assert_eq!(p.stream_id, 1234);
        assert_eq!(p.data, b"hello");
        assert_eq!(p.metadata, b"hello");
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = Payload {
            stream_id: 1234,
            metadata: b"hello".to_vec(),
            data: b"hello".to_vec(),
            follows: true,
            complete: true,
            next: true,
        };
        let encoded = payload.encode();
        assert_eq!(BYTES, encoded);
        Ok(())
    }
}

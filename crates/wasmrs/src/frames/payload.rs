use super::{
    Error, Frame, FRAME_FLAG_COMPLETE, FRAME_FLAG_FOLLOWS, FRAME_FLAG_METADATA, FRAME_FLAG_NEXT,
};

pub(crate) use crate::generated::Payload;

use crate::{
    generated::{FrameHeader, FrameType},
    util::{from_u24_bytes, to_u24_bytes},
};

impl Payload {
    fn get_flags(&self) -> u16 {
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

impl Frame<Payload> for Payload {
    fn kind(&self) -> FrameType {
        FrameType::Payload
    }

    fn decode(header: FrameHeader, mut buffer: Vec<u8>) -> Result<Payload, Error> {
        let (start, metadata_len) = if header.has_metadata() {
            (3, from_u24_bytes(&buffer[0..3]))
        } else {
            (0, 0)
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

    fn encode(mut self) -> Vec<u8> {
        let len =
            self.data.len() + self.metadata.len() + if self.metadata.is_empty() { 0 } else { 3 };
        let mut bytes = Vec::with_capacity(len);
        bytes.append(&mut to_u24_bytes(self.metadata.len() as u32));
        bytes.append(&mut self.metadata);
        bytes.append(&mut self.data);
        bytes
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Payload, self.get_flags())
    }
}

#[cfg(test)]
mod test {
    use crate::frames::Frame;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.payload.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("{:?}", BYTES);
        let header = FrameHeader::from_bytes(BYTES[0..6].to_vec());
        assert!(header.has_metadata());
        let p = Payload::decode(header, BYTES[6..].to_vec())?;
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
        let mut header = payload.gen_header().into_bytes();
        let mut encoded = payload.encode();
        header.append(&mut encoded);
        assert_eq!(BYTES, header);
        Ok(())
    }
}

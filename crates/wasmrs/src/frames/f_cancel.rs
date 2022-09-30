use bytes::Bytes;

use crate::{
    generated::{FrameHeader, FrameType},
    Frame,
};

use super::{Error, RSocketFrame};

use crate::generated::Cancel;

impl RSocketFrame<Cancel> for Cancel {
    const FRAME_TYPE: FrameType = FrameType::Cancel;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode_all(mut buffer: Bytes) -> Result<Self, Error> {
        let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
        Self::decode_frame(&header, buffer)
    }

    fn decode_frame(header: &FrameHeader, _buffer: Bytes) -> Result<Self, Error> {
        Self::check_type(header)?;
        Ok(Cancel {
            stream_id: header.stream_id(),
        })
    }

    fn encode(self) -> Bytes {
        self.gen_header().encode()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Cancel, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::frames::RSocketFrame;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.cancel.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW: {:?}", BYTES);
        let p = Cancel::decode_all(BYTES.into())?;
        // println!("{:?}", p);
        assert_eq!(p.stream_id, 1234);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = Cancel { stream_id: 1234 };
        let encoded = payload.encode();
        assert_eq!(encoded, Bytes::from(BYTES));
        Ok(())
    }
}

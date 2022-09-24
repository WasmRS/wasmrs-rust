use crate::{
    generated::{FrameHeader, FrameType},
    util::from_u32_bytes,
};

use super::{Error, FrameCodec};

pub use crate::generated::RequestN;

impl RequestN {}

impl FrameCodec<RequestN> for RequestN {
    const FRAME_TYPE: FrameType = FrameType::RequestN;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode(buffer: Vec<u8>) -> Result<RequestN, Error> {
        let header = FrameHeader::from_reader(&*buffer)?;
        Self::check_type(&header)?;
        Ok(RequestN {
            stream_id: header.stream_id(),
            n: from_u32_bytes(&buffer[6..6 + 4]),
        })
    }

    fn encode(self) -> Vec<u8> {
        [self.gen_header().encode(), self.n.to_be_bytes().to_vec()].concat()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::RequestN, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::frames::FrameCodec;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_n.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW {:?}", BYTES);
        let p = RequestN::decode(BYTES.to_vec())?;
        assert_eq!(p.stream_id, 1234);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = RequestN {
            stream_id: 1234,
            n: 15,
        };
        let encoded = payload.encode();
        assert_eq!(encoded, BYTES);
        Ok(())
    }
}

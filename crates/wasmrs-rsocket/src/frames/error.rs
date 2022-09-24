use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    generated::{FrameHeader, FrameType},
    util::from_u32_bytes,
    Frame,
};

use super::{Error, FrameCodec};

pub use crate::generated::ErrorFrame;

impl ErrorFrame {}

impl FrameCodec<ErrorFrame> for ErrorFrame {
    const FRAME_TYPE: FrameType = FrameType::Err;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode(mut buffer: Bytes) -> Result<ErrorFrame, Error> {
        let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
        Self::check_type(&header)?;
        let start = Frame::LEN_HEADER;

        Ok(ErrorFrame {
            stream_id: header.stream_id(),
            code: from_u32_bytes(&buffer.split_to(4)),
            data: String::from_utf8(buffer.to_vec()).map_err(|_| super::Error::StringConversion)?,
        })
    }

    fn encode(self) -> Bytes {
        let header = self.gen_header().encode();
        let code = self.code.to_be_bytes();
        let data = self.data.into_bytes();
        let mut bytes = BytesMut::with_capacity(Frame::LEN_HEADER + code.len() + data.len());
        bytes.put(header);
        bytes.put(code.as_slice());
        bytes.put(data.as_slice());
        bytes.freeze()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Err, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::frames::FrameCodec;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.error.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("{:?}", BYTES);
        let p = ErrorFrame::decode(BYTES.into())?;
        assert_eq!(p.stream_id, 1234);
        assert_eq!(&p.data, "errstr");
        assert_eq!(p.code, 11);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = ErrorFrame {
            stream_id: 1234,
            data: "errstr".to_owned(),
            code: 11,
        };
        let encoded = payload.encode();
        assert_eq!(encoded, Bytes::from(BYTES));
        Ok(())
    }
}

use crate::{
    generated::{FrameHeader, FrameType},
    util::from_u32_bytes,
};

use super::{Error, FrameCodec};

pub use crate::generated::ErrorFrame;

impl ErrorFrame {}

impl FrameCodec<ErrorFrame> for ErrorFrame {
    const FRAME_TYPE: FrameType = FrameType::Err;

    fn stream_id(&self) -> u32 {
        self.stream_id
    }

    fn decode(mut buffer: Vec<u8>) -> Result<ErrorFrame, Error> {
        let header = FrameHeader::from_reader(&*buffer)?;
        Self::check_type(&header)?;
        let start = 6;

        Ok(ErrorFrame {
            stream_id: header.stream_id(),
            code: from_u32_bytes(&buffer[start..start + 4]),
            data: String::from_utf8(buffer.drain(start + 4..).collect())
                .map_err(|_| super::Error::StringConversion)?,
        })
    }

    fn encode(self) -> Vec<u8> {
        let header = self.gen_header().encode();
        [
            header,
            self.code.to_be_bytes().to_vec(),
            self.data.into_bytes(),
        ]
        .concat()
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
        let p = ErrorFrame::decode(BYTES.to_vec())?;
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
        assert_eq!(encoded, BYTES);
        Ok(())
    }
}

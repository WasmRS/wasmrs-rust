use crate::{
    generated::{ErrorFrame, FrameHeader, FrameType},
    util::from_u32_bytes,
};

use super::{Error, Frame};

impl ErrorFrame {}

impl Frame<ErrorFrame> for ErrorFrame {
    fn kind(&self) -> FrameType {
        FrameType::Payload
    }

    fn decode(header: FrameHeader, mut buffer: Vec<u8>) -> Result<ErrorFrame, Error> {
        Ok(ErrorFrame {
            stream_id: header.stream_id(),
            code: from_u32_bytes(&buffer[0..4]),
            data: String::from_utf8(buffer.drain(4..).collect())
                .map_err(|_| super::Error::StringConversion)?,
        })
    }

    fn encode(self) -> Vec<u8> {
        let len = self.data.len() + 4;
        let mut bytes = Vec::with_capacity(len);
        bytes.append(&mut self.code.to_be_bytes().to_vec());
        bytes.append(&mut self.data.into_bytes());
        bytes
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Err, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::frames::Frame;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.error.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("{:?}", BYTES);
        let header = FrameHeader::from_bytes(BYTES[0..6].to_vec());
        let p = ErrorFrame::decode(header, BYTES[6..].to_vec())?;
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
        let mut header = payload.gen_header().into_bytes();
        let mut encoded = payload.encode();
        header.append(&mut encoded);
        assert_eq!(header, BYTES);
        Ok(())
    }
}

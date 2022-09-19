use crate::generated::{Cancel, FrameHeader, FrameType};

use super::{Error, Frame};

impl Frame<Cancel> for Cancel {
    fn kind(&self) -> FrameType {
        FrameType::Payload
    }

    fn decode(header: FrameHeader, _: Vec<u8>) -> Result<Cancel, Error> {
        Ok(Cancel {
            stream_id: header.stream_id(),
        })
    }

    fn encode(self) -> Vec<u8> {
        Vec::new()
    }

    fn gen_header(&self) -> FrameHeader {
        FrameHeader::new(self.stream_id, FrameType::Cancel, 0)
    }
}

#[cfg(test)]
mod test {
    use crate::frames::Frame;

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.cancel.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("{:?}", BYTES);
        let header = FrameHeader::from_bytes(BYTES[0..6].to_vec());
        let p = Cancel::decode(header, BYTES[6..].to_vec())?;
        assert_eq!(p.stream_id, 1234);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = Cancel { stream_id: 1234 };
        let mut header = payload.gen_header().into_bytes();
        let mut encoded = payload.encode();
        header.append(&mut encoded);
        assert_eq!(header, BYTES);
        Ok(())
    }
}

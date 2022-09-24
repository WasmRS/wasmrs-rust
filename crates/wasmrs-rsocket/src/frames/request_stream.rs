use super::{Error, FrameCodec, RequestStream};
use crate::generated::{FrameHeader, FrameType};

impl RequestStream {}

impl FrameCodec<RequestStream> for RequestStream {
    const FRAME_TYPE: FrameType = FrameType::RequestStream;

    fn stream_id(&self) -> u32 {
        self.0.stream_id
    }

    fn decode(buffer: Vec<u8>) -> Result<RequestStream, Error> {
        let header = FrameHeader::from_reader(&*buffer)?;
        Self::check_type(&header)?;
        Ok(Self(crate::generated::RequestPayload::decode(
            header, buffer,
        )?))
    }

    fn encode(self) -> Vec<u8> {
        self.0.encode()
    }

    fn gen_header(&self) -> FrameHeader {
        self.0.gen_header()
    }
}

#[cfg(test)]
mod test {
    use crate::{frames::FrameCodec, generated::RequestPayload};

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_stream.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW: {:?}", BYTES);
        let p = RequestStream::decode(BYTES.to_vec())?;
        assert_eq!(p.0.stream_id, 1234);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = RequestPayload {
            frame_type: FrameType::RequestStream,
            stream_id: 1234,
            metadata: b"hello".to_vec(),
            data: b"hello".to_vec(),
            follows: true,
            complete: true,
            initial_n: 0,
        };
        let this = RequestStream(payload);
        let encoded = this.encode();
        assert_eq!(encoded, BYTES);
        Ok(())
    }
}

use crate::generated::{FrameHeader, FrameType};

use super::{Error, FrameCodec, RequestResponse, FRAME_FLAG_METADATA};

impl RequestResponse {}

impl FrameCodec<RequestResponse> for RequestResponse {
    const FRAME_TYPE: FrameType = FrameType::RequestResponse;

    fn stream_id(&self) -> u32 {
        self.0.stream_id
    }

    fn decode(buffer: Vec<u8>) -> Result<RequestResponse, Error> {
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
        FrameHeader::new(
            self.0.stream_id,
            FrameType::RequestResponse,
            if self.0.metadata.is_empty() {
                0
            } else {
                FRAME_FLAG_METADATA
            },
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{frames::FrameCodec, generated::RequestPayload};

    use super::*;
    use anyhow::Result;

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_response.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW: {:?}", BYTES);
        let p = RequestResponse::decode(BYTES.to_vec())?;
        assert_eq!(p.0.stream_id, 1234);
        assert_eq!(&p.0.data, b"hello");
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = RequestPayload {
            frame_type: FrameType::RequestResponse,
            stream_id: 1234,
            metadata: b"hello".to_vec(),
            data: b"hello".to_vec(),
            follows: true,
            complete: true,
            initial_n: 1,
        };
        let this = RequestResponse(payload);
        let encoded = this.encode();
        assert_eq!(encoded, BYTES);
        Ok(())
    }
}

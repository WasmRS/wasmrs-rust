use crate::{
    generated::{FrameHeader, FrameType},
    Frame,
};

use super::{Error, FrameCodec, RequestResponse, FRAME_FLAG_METADATA};
use bytes::Bytes;

impl RequestResponse {}

impl FrameCodec<RequestResponse> for RequestResponse {
    const FRAME_TYPE: FrameType = FrameType::RequestResponse;

    fn stream_id(&self) -> u32 {
        self.0.stream_id
    }

    fn decode(mut buffer: Bytes) -> Result<RequestResponse, Error> {
        let header = FrameHeader::from_bytes(buffer.split_to(Frame::LEN_HEADER));
        Self::check_type(&header)?;
        Ok(Self(crate::generated::RequestPayload::decode(
            header, buffer,
        )?))
    }

    fn encode(self) -> Bytes {
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

    fn get_flags(&self) -> crate::generated::FrameFlags {
        self.0.get_flags()
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
        let p = RequestResponse::decode(BYTES.into())?;
        assert_eq!(p.0.stream_id, 1234);
        assert_eq!(p.0.data, Bytes::from(b"hello".as_slice()));
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = RequestPayload {
            frame_type: FrameType::RequestResponse,
            stream_id: 1234,
            metadata: Bytes::from("hello"),
            data: Bytes::from("hello"),
            follows: true,
            complete: false, // TODO THIS MAY BE A BUG IN GO VS RUST. GO BINARIES SHOULD HAVE COMPLETE SET BUT IT'S NOT.
            initial_n: 1,
        };
        let this = RequestResponse(payload);
        let encoded = this.encode();
        assert_eq!(encoded, Bytes::from(BYTES));
        Ok(())
    }
}

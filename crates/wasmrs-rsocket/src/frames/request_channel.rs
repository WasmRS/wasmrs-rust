use crate::{
    generated::{FrameHeader, FrameType},
    Frame,
};

use super::{Error, FrameCodec, RequestChannel};
use bytes::Bytes;

impl RequestChannel {}

impl FrameCodec<RequestChannel> for RequestChannel {
    const FRAME_TYPE: FrameType = FrameType::RequestChannel;

    fn stream_id(&self) -> u32 {
        self.0.stream_id
    }

    fn decode(mut buffer: Bytes) -> Result<RequestChannel, Error> {
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
        self.0.gen_header()
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

    static BYTES: &[u8] = include_bytes!("../../testdata/frame.request_channel.bin");

    #[test]
    fn test_decode() -> Result<()> {
        println!("RAW: {:?}", BYTES);
        let p = RequestChannel::decode(BYTES.into())?;
        assert_eq!(p.0.stream_id, 1234);
        Ok(())
    }

    #[test]
    fn test_encode() -> Result<()> {
        let payload = RequestPayload {
            frame_type: FrameType::RequestChannel,
            stream_id: 1234,
            metadata: Bytes::from("hello"),
            data: Bytes::from("hello"),
            follows: true,
            complete: false, // TODO THIS MAY BE A BUG IN GO VS RUST. GO BINARIES SHOULD HAVE COMPLETE SET BUT IT'S NOT.
            initial_n: 0,
        };
        let this = RequestChannel(payload);
        let encoded = this.encode();
        assert_eq!(encoded, Bytes::from(BYTES));
        Ok(())
    }
}

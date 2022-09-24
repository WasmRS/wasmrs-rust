pub(crate) mod cancel;
pub(crate) mod error;
pub(crate) mod payload;
pub(crate) mod request_channel;
pub(crate) mod request_fnf;
pub(crate) mod request_n;
pub(crate) mod request_payload;
pub(crate) mod request_response;
pub(crate) mod request_stream;

pub use crate::generated::{
    Cancel, ErrorFrame, Payload, RequestChannel, RequestN, RequestPayload, RequestResponse,
    RequestStream,
};
use bytes::{BufMut, Bytes, BytesMut};

use crate::{
    generated::{self as frames, BasePayload, FrameFlags, FrameHeader, FrameType},
    util::from_u16_bytes,
    Frame, Metadata,
};

#[derive(Debug)]
#[allow(missing_copy_implementations)]
pub enum Error {
    WrongType(FrameType, FrameType),
    ReadBuffer,
    StringConversion,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Error::WrongType(_, _) => "Tried to decode wrong type.",
            Error::ReadBuffer => "Could not read frame buffer.",
            Error::StringConversion => "Could not read string from bytes.",
        })
    }
}

pub const FRAME_FLAG_METADATA: FrameFlags = 1 << 8;
pub const FRAME_FLAG_FOLLOWS: FrameFlags = 1 << 7;
pub const FRAME_FLAG_COMPLETE: FrameFlags = 1 << 6;
pub const FRAME_FLAG_NEXT: FrameFlags = 1 << 5;
pub const FRAME_FLAG_IGNORE: FrameFlags = 1 << 4;

impl crate::generated::FrameFlag {}

impl BasePayload {
    pub fn new(metadata: Option<Bytes>, data: Option<Bytes>) -> Self {
        Self { metadata, data }
    }
}

impl Metadata {
    pub fn new(namespace: impl AsRef<str>, operation: impl AsRef<str>) -> Metadata {
        Metadata {
            namespace: namespace.as_ref().to_owned(),
            operation: operation.as_ref().to_owned(),
            instance: Bytes::new(),
        }
    }
    #[must_use]
    pub fn encode(self) -> Bytes {
        let len = self.namespace.len()
            + self.operation.len()
            + self.instance.len()
            + Frame::LEN_HEADER
            + 2
            + 2
            + 2;
        let mut bytes = BytesMut::with_capacity(len);
        bytes.put((self.namespace.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.namespace.into_bytes().as_slice());
        bytes.put((self.operation.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.operation.into_bytes().as_slice());
        bytes.put((self.instance.len() as u16).to_be_bytes().as_slice());
        bytes.put(self.instance);

        debug_assert_eq!(
            bytes.len(),
            len,
            "encoded metadata is not the correct length."
        );
        bytes.freeze()
    }
}

impl Frame {
    pub const LEN_HEADER: usize = 6;
    pub const FLAG_FOLLOW: FrameFlags = FRAME_FLAG_FOLLOWS;
    pub const FLAG_NEXT: FrameFlags = FRAME_FLAG_NEXT;
    pub const FLAG_COMPLETE: FrameFlags = FRAME_FLAG_COMPLETE;
    pub const FLAG_IGNORE: FrameFlags = FRAME_FLAG_IGNORE;
    pub const FLAG_METADATA: FrameFlags = FRAME_FLAG_METADATA;

    #[must_use]
    pub fn stream_id(&self) -> u32 {
        match self {
            Frame::Payload(frame) => frame.stream_id,
            Frame::Cancel(frame) => frame.stream_id,
            Frame::ErrorFrame(frame) => frame.stream_id,
            Frame::RequestN(frame) => frame.stream_id,
            Frame::RequestResponse(frame) => frame.0.stream_id,
            Frame::RequestFnF(frame) => frame.0.stream_id,
            Frame::RequestStream(frame) => frame.0.stream_id,
            Frame::RequestChannel(frame) => frame.0.stream_id,
        }
    }

    #[must_use]
    pub fn get_flag(&self) -> FrameFlags {
        match self {
            Frame::Payload(frame) => frame.get_flags(),
            Frame::Cancel(frame) => frame.get_flags(),
            Frame::ErrorFrame(frame) => frame.get_flags(),
            Frame::RequestN(frame) => frame.get_flags(),
            Frame::RequestResponse(frame) => frame.get_flags(),
            Frame::RequestFnF(frame) => frame.get_flags(),
            Frame::RequestStream(frame) => frame.get_flags(),
            Frame::RequestChannel(frame) => frame.get_flags(),
        }
    }

    #[must_use]
    pub fn frame_type(&self) -> FrameType {
        match self {
            Frame::Payload(_) => FrameType::Payload,
            Frame::Cancel(_) => FrameType::Cancel,
            Frame::ErrorFrame(_) => FrameType::Err,
            Frame::RequestN(_) => FrameType::RequestN,
            Frame::RequestResponse(_) => FrameType::RequestResponse,
            Frame::RequestFnF(_) => FrameType::RequestFnf,
            Frame::RequestStream(_) => FrameType::RequestStream,
            Frame::RequestChannel(_) => FrameType::RequestChannel,
        }
    }

    pub fn decode(mut bytes: Bytes) -> Result<Frame, (u32, Error)> {
        let header = FrameHeader::from_bytes(bytes.split_to(Frame::LEN_HEADER));
        let stream_id = header.stream_id();
        Self::_decode(header, bytes).map_err(|e| (stream_id, e))
    }

    pub fn _decode(header: FrameHeader, buffer: Bytes) -> Result<Frame, Error> {
        let frame = match header.frame_type() {
            FrameType::Reserved => todo!(),
            FrameType::Setup => todo!(),
            FrameType::RequestResponse => {
                frames::Frame::RequestResponse(frames::RequestResponse::decode(buffer)?)
            }
            FrameType::RequestFnf => frames::Frame::RequestFnF(frames::RequestFnF::decode(buffer)?),
            FrameType::RequestStream => {
                frames::Frame::RequestStream(frames::RequestStream::decode(buffer)?)
            }
            FrameType::RequestChannel => {
                frames::Frame::RequestChannel(frames::RequestChannel::decode(buffer)?)
            }
            FrameType::RequestN => todo!(),
            FrameType::Cancel => frames::Frame::Cancel(Box::new(Cancel {
                stream_id: header.stream_id(),
            })),
            FrameType::Payload => {
                frames::Frame::Payload(Box::new(frames::Payload::decode(buffer)?))
            }
            FrameType::Err => {
                frames::Frame::ErrorFrame(Box::new(frames::ErrorFrame::decode(buffer)?))
            }
            FrameType::Ext => todo!(),
            _ => todo!(), // Maybe not todo?,
        };
        Ok(frame)
    }

    #[must_use]
    pub fn encode(self) -> Bytes {
        match self {
            Frame::Payload(f) => f.encode(),
            Frame::Cancel(f) => f.encode(),
            Frame::ErrorFrame(f) => f.encode(),
            Frame::RequestN(f) => f.encode(),
            Frame::RequestResponse(f) => f.encode(),
            Frame::RequestFnF(f) => f.encode(),
            Frame::RequestStream(f) => f.encode(),
            Frame::RequestChannel(f) => f.encode(),
        }
    }

    pub fn new_error(stream_id: u32, code: u32, data: impl AsRef<str>) -> Frame {
        Frame::ErrorFrame(Box::new(ErrorFrame {
            stream_id,
            code,
            data: data.as_ref().to_owned(),
        }))
    }

    pub fn new_payload(stream_id: u32, metadata: Bytes, data: Bytes, flags: FrameFlags) -> Frame {
        Frame::Payload(Box::new(Payload {
            stream_id,
            metadata,
            data,
            follows: flags & FRAME_FLAG_FOLLOWS == FRAME_FLAG_FOLLOWS,
            complete: flags & FRAME_FLAG_COMPLETE == FRAME_FLAG_COMPLETE,
            next: flags & FRAME_FLAG_NEXT == FRAME_FLAG_NEXT,
        }))
    }
}

pub trait FrameCodec<T> {
    const FRAME_TYPE: FrameType;
    fn check_type(header: &FrameHeader) -> Result<(), Error> {
        if header.frame_type() == Self::FRAME_TYPE {
            Ok(())
        } else {
            Err(Error::WrongType(header.frame_type(), Self::FRAME_TYPE))
        }
    }
    fn kind(&self) -> FrameType {
        Self::FRAME_TYPE
    }
    fn stream_id(&self) -> u32;
    fn decode(buffer: Bytes) -> Result<T, Error>;
    fn encode(self) -> Bytes;
    fn gen_header(&self) -> FrameHeader;
    fn get_flags(&self) -> FrameFlags {
        0
    }
}

impl FrameHeader {
    pub(crate) fn new(stream_id: u32, frame_type: FrameType, frame_flags: u16) -> Self {
        let mut header = BytesMut::with_capacity(Frame::LEN_HEADER);
        let frame_type: u32 = frame_type.into();
        let frame_type: u16 = frame_type.try_into().unwrap();
        let frame_type = (frame_type << 10) | frame_flags;

        header.put(stream_id.to_be_bytes().as_slice());
        header.put(frame_type.to_be_bytes().as_slice());

        Self {
            header: header.freeze(),
        }
    }

    pub(crate) fn from_bytes(header: Bytes) -> Self {
        Self { header }
    }

    #[cfg(test)]
    fn as_bytes(&self) -> &[u8] {
        &self.header
    }

    fn encode(self) -> Bytes {
        self.header
    }

    pub(crate) fn stream_id(&self) -> u32 {
        let bytes: [u8; 4] = [
            self.header[0] & 0x7f,
            self.header[1],
            self.header[2],
            self.header[3],
        ];
        u32::from_be_bytes(bytes)
    }

    fn n(&self) -> u16 {
        // let bytes: [u8; 2] = [self.header[4], self.header[5]];
        from_u16_bytes(&self.header.slice(4..Frame::LEN_HEADER))
    }

    pub(crate) fn frame_type(&self) -> FrameType {
        let id: u8 = (self.header[4] >> 2);
        id.try_into().unwrap()
    }

    pub fn check(&self, flag: FrameFlags) -> bool {
        self.n() & flag == flag
    }

    pub(crate) fn has_metadata(&self) -> bool {
        self.check(FRAME_FLAG_METADATA)
    }

    pub(crate) fn has_follows(&self) -> bool {
        self.check(FRAME_FLAG_FOLLOWS)
    }

    pub(crate) fn has_next(&self) -> bool {
        self.check(FRAME_FLAG_NEXT)
    }

    pub(crate) fn has_complete(&self) -> bool {
        self.check(FRAME_FLAG_COMPLETE)
    }

    pub(crate) fn has_ignore(&self) -> bool {
        self.check(FRAME_FLAG_IGNORE)
    }
}

impl std::fmt::Display for FrameHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut flags = Vec::new();
        if self.has_next() {
            flags.push("N");
        }
        if self.has_complete() {
            flags.push("CL");
        }
        if self.has_follows() {
            flags.push("FRS");
        }
        if self.has_metadata() {
            flags.push("M");
        }
        if self.has_ignore() {
            flags.push("I");
        }

        let t = self.frame_type();

        write!(
            f,
            "FrameHeader{{id={},type={},flag={}}}",
            self.stream_id(),
            t,
            flags.join("|")
        )
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    fn print_binary(v: &[u8]) {
        let mut bytes = Vec::new();
        for byte in v {
            bytes.push(format!("{:08b}", byte));
        }
        println!("[{}]", bytes.join(" "));
    }
    use crate::{
        frames::{FRAME_FLAG_FOLLOWS, FRAME_FLAG_IGNORE, FRAME_FLAG_METADATA, FRAME_FLAG_NEXT},
        generated::{FrameHeader, FrameType},
        Frame,
    };

    use super::FRAME_FLAG_COMPLETE;

    #[test]
    fn test_new_header() -> Result<()> {
        let header = FrameHeader::new(2147483647, FrameType::Payload, FRAME_FLAG_COMPLETE);
        println!("Bytes: {:?}", header.as_bytes());
        println!("Frame type: {}", header.frame_type());
        print_binary(header.as_bytes());
        println!("Header: {}", header);
        assert_eq!(header.stream_id(), 2147483647);
        assert_eq!(header.frame_type() as u32, FrameType::Payload.into());
        assert!(header.has_complete());
        assert!(!header.has_next());
        assert!(!header.has_metadata());
        assert!(!header.has_follows());
        assert!(!header.has_ignore());

        Ok(())
    }

    #[test]
    fn test_payload_header() -> Result<()> {
        let frame = include_bytes!("../../testdata/frame.payload.bin");
        let header = FrameHeader::from_bytes(frame[0..Frame::LEN_HEADER].into());
        print_binary(header.as_bytes());
        assert!(header.has_metadata());
        Ok(())
    }

    #[test]
    fn test_header() -> Result<()> {
        let header = FrameHeader::from_bytes(vec![0u8, 0, 4, 210, 25, 0].into());
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(header.has_metadata());
        Ok(())
    }

    #[test]
    fn test_header_no_flags() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, 0);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(!header.has_metadata());
        assert!(!header.has_next());
        assert!(!header.has_complete());
        assert!(!header.has_metadata());
        assert!(!header.has_ignore());
        Ok(())
    }

    #[test]
    fn test_header_metadata() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_METADATA);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(header.has_metadata());
        assert!(!header.has_next());
        assert!(!header.has_complete());
        assert!(!header.has_follows());
        assert!(!header.has_ignore());
        Ok(())
    }

    #[test]
    fn test_header_next() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_NEXT);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(!header.has_metadata());
        assert!(header.has_next());
        assert!(!header.has_complete());
        assert!(!header.has_follows());
        assert!(!header.has_ignore());
        Ok(())
    }

    #[test]
    fn test_header_complete() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_COMPLETE);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(!header.has_metadata());
        assert!(!header.has_next());
        assert!(header.has_complete());
        assert!(!header.has_follows());
        assert!(!header.has_ignore());
        Ok(())
    }

    #[test]
    fn test_header_ignore() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_IGNORE);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(!header.has_metadata());
        assert!(!header.has_next());
        assert!(!header.has_complete());
        assert!(!header.has_follows());
        assert!(header.has_ignore());
        Ok(())
    }

    #[test]
    fn test_header_follows() -> Result<()> {
        let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_FOLLOWS);
        print_binary(header.as_bytes());
        println!("{}", header);
        println!("{:?}", header.as_bytes());
        assert!(!header.has_metadata());
        assert!(!header.has_next());
        assert!(!header.has_complete());
        assert!(header.has_follows());
        assert!(!header.has_ignore());
        Ok(())
    }

    // #[test]
    // fn test_flags() -> Result<()> {
    //     let header = FrameHeader::new(0, FrameType::RequestStream, FRAME_FLAG_IGNORE);
    //     print_binary(&FRAME_FLAG_IGNORE.to_be_bytes());
    //     print_binary(&FRAME_FLAG_NEXT.to_be_bytes());
    //     print_binary(&FRAME_FLAG_COMPLETE.to_be_bytes());
    //     print_binary(&FRAME_FLAG_FOLLOWS.to_be_bytes());
    //     print_binary(&FRAME_FLAG_METADATA.to_be_bytes());
    //     panic!();
    //     Ok(())
    // }
}

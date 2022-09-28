pub(crate) mod cancel;
pub(crate) mod error;
pub(crate) mod payload;
pub(crate) mod request_channel;
pub(crate) mod request_fnf;
pub(crate) mod request_n;
pub(crate) mod request_payload;
pub(crate) mod request_response;
pub(crate) mod request_stream;

use crate::{
    generated::{Cancel, ErrorFrame, PayloadFrame, RequestChannel, RequestResponse, RequestStream},
    read_data, read_string, Error, RequestFnF,
};
pub use bytes::{BufMut, Bytes, BytesMut};
pub use payload::FragmentedPayload;

use crate::{
    generated::{self as frames, FrameFlags, FrameHeader, FrameType, Payload},
    util::from_u16_bytes,
    Frame, Metadata,
};

pub const FRAME_FLAG_METADATA: FrameFlags = 1 << 8;
pub const FRAME_FLAG_FOLLOWS: FrameFlags = 1 << 7;
pub const FRAME_FLAG_COMPLETE: FrameFlags = 1 << 6;
pub const FRAME_FLAG_NEXT: FrameFlags = 1 << 5;
pub const FRAME_FLAG_IGNORE: FrameFlags = 1 << 4;

impl crate::generated::FrameFlag {}

impl Payload {
    pub fn new(metadata: Bytes, data: Bytes) -> Self {
        Self {
            metadata: Some(metadata),
            data: Some(data),
        }
    }
    pub fn new_optional(metadata: Option<Bytes>, data: Option<Bytes>) -> Self {
        Self { metadata, data }
    }
    pub fn empty() -> Self {
        Self {
            metadata: None,
            data: None,
        }
    }
    pub fn parse_metadata(&self) -> Result<Metadata, Error> {
        if self.metadata.is_none() {
            return Err(Error::MetadataNotFound);
        }
        let bytes = self.metadata.as_ref().unwrap();
        let (namespace, nslen) = read_string(0, bytes)?;
        let (operation, oplen) = read_string(nslen, bytes)?;
        let (instance, _) = read_data(nslen + oplen, bytes)?;
        Ok(Metadata {
            namespace,
            operation,
            instance: instance.into(),
        })
    }
}

impl From<Frame> for Result<Option<Payload>, crate::PayloadError> {
    fn from(frame: Frame) -> Self {
        match frame {
            Frame::PayloadFrame(frame) => Ok(Some(Payload::new(frame.metadata, frame.data))),
            Frame::Cancel(_frame) => todo!(),
            Frame::ErrorFrame(frame) => Err(crate::PayloadError::new(frame.code, frame.data)),
            Frame::RequestN(_frame) => todo!(),
            Frame::RequestResponse(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
            Frame::RequestFnF(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
            Frame::RequestStream(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
            Frame::RequestChannel(frame) => Ok(Some(Payload::new(frame.0.metadata, frame.0.data))),
        }
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
        let len = self.namespace.len() + self.operation.len() + self.instance.len() + 2 + 2 + 2;
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

    pub fn is_followable_or_payload(&self) -> (bool, bool) {
        match &self {
            Frame::RequestFnF(_) => (true, false),
            Frame::RequestResponse(_) => (true, false),
            Frame::RequestStream(_) => (true, false),
            Frame::RequestChannel(_) => (true, false),
            Frame::PayloadFrame(_) => (true, true),
            _ => (false, false),
        }
    }

    #[must_use]
    pub fn stream_id(&self) -> u32 {
        match self {
            Frame::PayloadFrame(frame) => frame.stream_id,
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
            Frame::PayloadFrame(frame) => frame.get_flag(),
            Frame::Cancel(frame) => frame.get_flag(),
            Frame::ErrorFrame(frame) => frame.get_flag(),
            Frame::RequestN(frame) => frame.get_flag(),
            Frame::RequestResponse(frame) => frame.get_flag(),
            Frame::RequestFnF(frame) => frame.get_flag(),
            Frame::RequestStream(frame) => frame.get_flag(),
            Frame::RequestChannel(frame) => frame.get_flag(),
        }
    }

    #[must_use]
    pub fn frame_type(&self) -> FrameType {
        match self {
            Frame::PayloadFrame(_) => FrameType::Payload,
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
            FrameType::RequestResponse => frames::Frame::RequestResponse(
                frames::RequestResponse::decode_frame(&header, buffer)?,
            ),
            FrameType::RequestFnf => {
                frames::Frame::RequestFnF(frames::RequestFnF::decode_frame(&header, buffer)?)
            }
            FrameType::RequestStream => {
                frames::Frame::RequestStream(frames::RequestStream::decode_frame(&header, buffer)?)
            }
            FrameType::RequestChannel => frames::Frame::RequestChannel(
                frames::RequestChannel::decode_frame(&header, buffer)?,
            ),
            FrameType::RequestN => todo!(),
            FrameType::Cancel => frames::Frame::Cancel(Cancel {
                stream_id: header.stream_id(),
            }),
            FrameType::Payload => {
                frames::Frame::PayloadFrame(frames::PayloadFrame::decode_frame(&header, buffer)?)
            }
            FrameType::Err => {
                frames::Frame::ErrorFrame(frames::ErrorFrame::decode_frame(&header, buffer)?)
            }
            FrameType::Ext => todo!(),
            _ => unreachable!(), // Maybe not todo?,
        };
        Ok(frame)
    }

    #[must_use]
    pub fn encode(self) -> Bytes {
        match self {
            Frame::PayloadFrame(f) => f.encode(),
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
        Frame::ErrorFrame(ErrorFrame {
            stream_id,
            code,
            data: data.as_ref().to_owned(),
        })
    }

    pub fn new_cancel(stream_id: u32) -> Frame {
        Frame::Cancel(Cancel { stream_id })
    }

    pub fn new_request_response(
        stream_id: u32,
        payload: Payload,
        flags: FrameFlags,
        initial_n: u32,
    ) -> Frame {
        Frame::RequestResponse(RequestResponse::from_payload(
            stream_id, payload, flags, initial_n,
        ))
    }

    pub fn new_request_stream(
        stream_id: u32,
        payload: Payload,
        flags: FrameFlags,
        initial_n: u32,
    ) -> Frame {
        Frame::RequestStream(RequestStream::from_payload(
            stream_id, payload, flags, initial_n,
        ))
    }

    pub fn new_request_channel(
        stream_id: u32,
        payload: Payload,
        flags: FrameFlags,
        initial_n: u32,
    ) -> Frame {
        Frame::RequestChannel(RequestChannel::from_payload(
            stream_id, payload, flags, initial_n,
        ))
    }

    pub fn new_request_fnf(
        stream_id: u32,
        payload: Payload,
        flags: FrameFlags,
        initial_n: u32,
    ) -> Frame {
        Frame::RequestFnF(RequestFnF::from_payload(
            stream_id, payload, flags, initial_n,
        ))
    }

    pub fn new_payload(stream_id: u32, payload: Payload, flags: FrameFlags) -> Frame {
        Frame::PayloadFrame(PayloadFrame::from_payload(stream_id, payload, flags))
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
    fn decode_all(buffer: Bytes) -> Result<T, Error>;
    fn decode_frame(header: &FrameHeader, buffer: Bytes) -> Result<T, Error>;
    fn encode(self) -> Bytes;
    fn gen_header(&self) -> FrameHeader;
    fn get_flag(&self) -> FrameFlags {
        0
    }
}

pub trait RSocketFlags {
    fn flag_next(&self) -> bool;
    fn flag_metadata(&self) -> bool;
    fn flag_complete(&self) -> bool;
    fn flag_follows(&self) -> bool;
    fn flag_ignore(&self) -> bool;
}

impl RSocketFlags for FrameFlags {
    fn flag_next(&self) -> bool {
        self & Frame::FLAG_NEXT == Frame::FLAG_NEXT
    }

    fn flag_metadata(&self) -> bool {
        self & Frame::FLAG_METADATA == Frame::FLAG_METADATA
    }

    fn flag_complete(&self) -> bool {
        self & Frame::FLAG_COMPLETE == Frame::FLAG_COMPLETE
    }

    fn flag_follows(&self) -> bool {
        self & Frame::FLAG_FOLLOW == Frame::FLAG_FOLLOW
    }

    fn flag_ignore(&self) -> bool {
        self & Frame::FLAG_IGNORE == Frame::FLAG_IGNORE
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
        let id: u8 = self.header[4] >> 2;
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

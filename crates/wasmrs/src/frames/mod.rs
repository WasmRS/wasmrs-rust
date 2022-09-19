pub(crate) mod cancel;
pub(crate) mod error;
pub(crate) mod payload;

use crate::generated::{FrameHeader, FrameType};

#[derive(Debug)]
pub enum Error {
    // DecodeError,
    StringConversion,
}

impl std::error::Error for Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Error")
    }
}

static FRAME_FLAG_METADATA: u16 = 1 << 5;
static FRAME_FLAG_FOLLOWS: u16 = 1 << 6;
static FRAME_FLAG_COMPLETE: u16 = 1 << 7;
static FRAME_FLAG_NEXT: u16 = 1 << 8;
static FRAME_FLAG_IGNORE: u16 = 1 << 9;

impl crate::generated::FrameFlag {}

pub(crate) trait Frame<T> {
    fn kind(&self) -> FrameType;
    fn decode(header: FrameHeader, buffer: Vec<u8>) -> Result<T, Error>;
    fn encode(self) -> Vec<u8>;
    fn gen_header(&self) -> FrameHeader;
}

impl crate::generated::FrameHeader {
    pub(crate) fn new(stream_id: u32, frame_type: FrameType, frame_flags: u16) -> Self {
        let frame_type: u32 = frame_type.into();
        let frame_type: u16 = frame_type.try_into().unwrap();
        let frame_type = (frame_type << 10) | frame_flags;

        let header = [
            stream_id.to_be_bytes().to_vec(),
            frame_type.to_be_bytes().to_vec(),
        ]
        .concat();
        println!("header: {:?} ", header);

        Self { header }
    }

    pub(crate) fn from_bytes(header: Vec<u8>) -> Self {
        println!("{:?}", header);
        Self { header }
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.header
    }

    pub(crate) fn into_bytes(self) -> Vec<u8> {
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
        let bytes: [u8; 2] = [self.header[4], self.header[5]];
        u16::from_be_bytes(bytes)
    }
    pub(crate) fn flag(&self) -> u16 {
        let bytes: [u8; 2] = [self.header[4], self.header[5]];
        u16::from_be_bytes(bytes)
    }

    pub(crate) fn frame_type(&self) -> u8 {
        self.header[4] >> 2
    }

    fn check(&self, flag: u16) -> bool {
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

        let t = FrameType::try_from(self.frame_type() as u32).unwrap();

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

    use crate::generated::{FrameHeader, FrameType};

    use super::FRAME_FLAG_COMPLETE;

    fn print_binary(v: &[u8]) -> () {
        let mut bytes = Vec::new();
        for byte in v {
            bytes.push(format!("{:08b}", byte));
        }
        println!("[{}]", bytes.join(" "));
    }

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
        let header = FrameHeader::from_bytes(frame[0..6].to_vec());
        print_binary(header.as_bytes());
        assert!(header.has_metadata());
        Ok(())
    }
}

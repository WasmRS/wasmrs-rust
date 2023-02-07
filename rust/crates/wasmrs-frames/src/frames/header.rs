use bytes::{BufMut, Bytes, BytesMut};

use super::{FrameFlags, FrameHeader, FrameType};
use crate::util::from_u16_bytes;
use crate::Frame;

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

  pub(crate) fn encode(self) -> Bytes {
    self.header
  }

  pub(crate) fn stream_id(&self) -> u32 {
    let bytes: [u8; 4] = [self.header[0] & 0x7f, self.header[1], self.header[2], self.header[3]];
    u32::from_be_bytes(bytes)
  }

  fn n(&self) -> u16 {
    from_u16_bytes(&self.header.slice(4..Frame::LEN_HEADER))
  }

  pub(crate) fn frame_type(&self) -> FrameType {
    let id: u8 = self.header[4] >> 2;
    id.try_into().unwrap()
  }

  pub(crate) fn check(&self, flag: FrameFlags) -> bool {
    self.n() & flag == flag
  }

  pub(crate) fn has_metadata(&self) -> bool {
    self.check(Frame::FLAG_METADATA)
  }

  pub(crate) fn has_follows(&self) -> bool {
    self.check(Frame::FLAG_FOLLOW)
  }

  pub(crate) fn has_next(&self) -> bool {
    self.check(Frame::FLAG_NEXT)
  }

  pub(crate) fn has_complete(&self) -> bool {
    self.check(Frame::FLAG_COMPLETE)
  }

  pub(crate) fn has_ignore(&self) -> bool {
    self.check(Frame::FLAG_IGNORE)
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
  use super::*;
  use anyhow::Result;

  fn print_binary(v: &[u8]) {
    let mut bytes = Vec::new();
    for byte in v {
      bytes.push(format!("{:08b}", byte));
    }
    println!("[{}]", bytes.join(" "));
  }
  use crate::Frame;

  #[test]
  fn test_new_header() -> Result<()> {
    let header = FrameHeader::new(2147483647, FrameType::Payload, Frame::FLAG_COMPLETE);
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
    let header = FrameHeader::new(0, FrameType::RequestStream, Frame::FLAG_METADATA);
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
    let header = FrameHeader::new(0, FrameType::RequestStream, Frame::FLAG_NEXT);
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
    let header = FrameHeader::new(0, FrameType::RequestStream, Frame::FLAG_COMPLETE);
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
    let header = FrameHeader::new(0, FrameType::RequestStream, Frame::FLAG_IGNORE);
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
    let header = FrameHeader::new(0, FrameType::RequestStream, Frame::FLAG_FOLLOW);
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

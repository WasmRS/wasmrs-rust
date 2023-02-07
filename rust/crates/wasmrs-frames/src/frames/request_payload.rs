use bytes::{BufMut, Bytes, BytesMut};

use super::{Error, FrameFlags, FrameHeader, FrameType, RSocketFlags};
use crate::util::{from_u24_bytes, from_u32_bytes, to_u24_bytes};
use crate::{Frame, Payload};

#[derive()]
#[cfg_attr(not(target = "wasm32-unknown-unknown"), derive(Debug))]
#[cfg_attr(feature = "serde", derive(Clone))]
#[must_use]
pub struct RequestPayload {
  /// The type of Request this payload creates.
  pub frame_type: FrameType,
  /// The stream ID this frame belongs to.
  pub stream_id: u32,
  /// Any metadata associated with the Payload as raw bytes.
  pub metadata: Bytes,
  /// The actual payload data as raw bytes.
  pub data: Bytes,
  /// Whether this payload is broken up into multiple frames.
  pub follows: bool,
  /// Whether or not this frame is the last frame in a stream.
  pub complete: bool,
  pub initial_n: u32,
}

impl RequestPayload {
  pub(super) fn from_payload(
    stream_id: u32,
    payload: Payload,
    frame_type: FrameType,
    flags: FrameFlags,
    initial_n: u32,
  ) -> Self {
    Self {
      stream_id,
      metadata: payload.metadata.unwrap_or_default(),
      data: payload.data.unwrap_or_default(),
      follows: flags.flag_follows(),
      complete: flags.flag_complete(),
      frame_type,
      initial_n,
    }
  }

  pub(super) fn get_flags(&self) -> FrameFlags {
    let mut flags = 0;
    if !self.metadata.is_empty() {
      flags |= Frame::FLAG_METADATA;
    }
    if self.complete && self.frame_type == FrameType::RequestChannel {
      flags |= Frame::FLAG_COMPLETE;
    }
    flags
  }

  pub(crate) fn decode(header: &FrameHeader, mut buffer: Bytes) -> Result<RequestPayload, Error> {
    let frame_type = header.frame_type();

    let initial_n = if Self::is_multi(frame_type) {
      from_u32_bytes(&buffer.split_to(4))
    } else {
      0
    };

    let metadata = if header.has_metadata() {
      let metadata_len = from_u24_bytes(&buffer.split_to(3)) as usize;
      buffer.split_to(metadata_len)
    } else {
      Bytes::new()
    };

    let payload: Bytes = buffer;

    Ok(RequestPayload {
      frame_type,
      stream_id: header.stream_id(),
      metadata,
      data: payload,
      follows: header.has_follows(),
      complete: header.has_complete(),
      initial_n,
    })
  }

  fn is_multi(frame_type: FrameType) -> bool {
    matches!(frame_type, FrameType::RequestChannel | FrameType::RequestStream)
  }

  pub(crate) fn gen_header(&self) -> FrameHeader {
    FrameHeader::new(self.stream_id, self.frame_type, self.get_flags())
  }

  #[must_use]
  pub(crate) fn encode(self) -> Bytes {
    let header = self.gen_header().encode();
    let n_bytes = if Self::is_multi(self.frame_type) {
      self.initial_n.to_be_bytes().to_vec()
    } else {
      Vec::new()
    };

    let (mlen, md) = if self.metadata.is_empty() {
      (Bytes::new(), Bytes::new())
    } else {
      (to_u24_bytes(self.metadata.len() as u32), self.metadata)
    };
    let data = self.data;
    let frame_len = Frame::LEN_HEADER + n_bytes.len() + mlen.len() + md.len() + data.len();
    let mut bytes = BytesMut::with_capacity(frame_len);
    bytes.put(header);
    bytes.put(n_bytes.as_slice());
    bytes.put(mlen);
    bytes.put(md);
    bytes.put(data);
    bytes.freeze()
  }
}

impl From<RequestPayload> for Payload {
  fn from(req: RequestPayload) -> Self {
    Payload {
      metadata: Some(req.metadata),
      data: Some(req.data),
    }
  }
}

#[cfg(test)]
mod test {
  // Tested in the request* frames
}

#![allow(unused)]
use std::path::Path;

use wasmrs_frames::Frame;

use crate::SocketSide;

#[derive(Debug, Default)]
/// A struct used to record frames sent & received by a [WasmSocket].
pub struct FrameRecords {
  pub frames: Vec<FrameRecord>,
}

impl FrameRecords {
  fn push(&mut self, record: FrameRecord) {
    #[cfg(feature = "print-frames")]
    print_record(record.clone(), self.frames.len());
    self.frames.push(record.clone());
    #[cfg(feature = "dump-frames")]
    dump_record(record, self.frames.len(), Path::new("frames")).unwrap();
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "dir"))]
/// A serialized frame sent or received by a [WasmSocket].
pub enum FrameRecord {
  /// An incoming frame.
  Incoming {
    /// Whether the frame was received by the host or the guest.
    side: SocketSide,
    /// The stream ID the frame belongs to.
    stream_id: u32,
    /// The frame encoded as base64.
    frame: String,
  },
  /// An outgoing frame.
  Outgoing {
    /// Whether the frame was sent by the host or the guest.
    side: SocketSide,
    /// The stream ID the frame belongs to.
    stream_id: u32,
    /// The frame encoded as base64.
    frame: String,
  },
}

impl FrameRecord {
  /// True if the frame was sent out of the socket.
  #[must_use]
  pub fn is_outgoing(&self) -> bool {
    matches!(self, FrameRecord::Outgoing { .. })
  }

  /// True if the frame was received from of the socket.
  #[must_use]
  pub fn is_incoming(&self) -> bool {
    matches!(self, FrameRecord::Outgoing { .. })
  }

  fn dir(&self) -> &str {
    match self {
      FrameRecord::Incoming { .. } => "in",
      FrameRecord::Outgoing { .. } => "out",
    }
  }

  fn stream_id(&self) -> u32 {
    match self {
      FrameRecord::Incoming { stream_id, .. } => *stream_id,
      FrameRecord::Outgoing { stream_id, .. } => *stream_id,
    }
  }

  /// Decode the frame from the base64-encoded string.
  pub fn frame(&self) -> Result<Frame, crate::Error> {
    let frame = match self {
      FrameRecord::Incoming { frame, .. } => frame,
      FrameRecord::Outgoing { frame, .. } => frame,
    };
    use base64::Engine;
    wasmrs_frames::Frame::decode(
      base64::engine::general_purpose::STANDARD
        .decode(frame)
        .map_err(|e| crate::Error::Record(e.to_string()))?
        .into(),
    )
    .map_err(|(id, e)| crate::Error::Record(e.to_string()))
  }

  /// The base64 representation of the frame.
  #[must_use]
  pub fn encoded(&self) -> &str {
    match self {
      FrameRecord::Incoming { frame, .. } => frame,
      FrameRecord::Outgoing { frame, .. } => frame,
    }
  }

  fn side(&self) -> String {
    match self {
      FrameRecord::Incoming { side, .. } => side.to_string(),
      FrameRecord::Outgoing { side, .. } => side.to_string(),
    }
  }
}

impl std::fmt::Display for FrameRecord {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FrameRecord::Incoming { side, stream_id, frame } => {
        write!(f, "s{}-{}-in", stream_id, side)
      }
      FrameRecord::Outgoing { side, stream_id, frame } => {
        write!(f, "s{}-{}-out", stream_id, side)
      }
    }
  }
}

/// A record of all frames sent & received by a [crate::WasmSocket].
pub static FRAME_RECORDS: once_cell::sync::Lazy<parking_lot::Mutex<FrameRecords>> =
  once_cell::sync::Lazy::new(|| parking_lot::Mutex::new(FrameRecords::default()));

pub(crate) fn write_outgoing_record(side: SocketSide, frame: &Frame) {
  use base64::Engine;
  FRAME_RECORDS.lock().push(FrameRecord::Outgoing {
    side,
    stream_id: frame.stream_id(),
    frame: base64::engine::general_purpose::STANDARD.encode(frame.clone().encode()),
  });
}

pub(crate) fn write_incoming_record(side: SocketSide, frame: &Frame) {
  use base64::Engine;
  FRAME_RECORDS.lock().push(FrameRecord::Incoming {
    side,
    stream_id: frame.stream_id(),
    frame: base64::engine::general_purpose::STANDARD.encode(frame.clone().encode()),
  });
}

fn dump_record(record: FrameRecord, i: usize, dir: &Path) -> Result<(), crate::Error> {
  #[cfg(feature = "dump-frames")]
  {
    use std::fs::File;
    use std::io::Write;

    if std::fs::read_dir(dir).is_err() {
      std::fs::create_dir(dir).map_err(|e| crate::Error::Record(e.to_string()))?;
    }

    let mut file = File::create(dir.join(format!(
      "s{}-n{:03}-{}-{}.frame",
      record.stream_id(),
      i,
      record.side(),
      record.dir()
    )))
    .map_err(|e| crate::Error::Record(e.to_string()))?;
    let json = serde_json::to_string(&record).unwrap();

    file
      .write_all(json.as_bytes())
      .map_err(|e| crate::Error::Record(e.to_string()))?;
  }
  Ok(())
}

/// Get the recorded frames.
pub fn get_records() -> Vec<FrameRecord> {
  FRAME_RECORDS.lock().frames.drain(..).collect()
}

fn print_record(record: FrameRecord, i: usize) {
  #[cfg(feature = "print-frames")]
  {
    let json = serde_json::to_string(&record).unwrap();

    println!("{}", json);
  }
}

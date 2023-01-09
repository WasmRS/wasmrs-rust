use std::sync::atomic::{AtomicU32, Ordering};

/// The implementation of a WasmRS buffer where frames are written to and read from.
pub struct BufferState {
  size: AtomicU32,
  start: AtomicU32,
}

impl Default for BufferState {
  fn default() -> Self {
    Self {
      size: AtomicU32::new(4092),
      start: Default::default(),
    }
  }
}

impl BufferState {
  /// Get the size of the buffer.
  pub fn get_size(&self) -> u32 {
    self.size.load(Ordering::SeqCst)
  }

  /// Change the size of the buffer.
  pub fn update_size(&self, size: u32) {
    self.size.store(size, Ordering::SeqCst);
  }

  /// Get the start location of the buffer.
  pub fn get_start(&self) -> u32 {
    self.start.load(Ordering::SeqCst)
  }

  /// Update the start position of thebuffer.
  pub fn update_start(&self, position: u32) {
    self.start.store(position, Ordering::SeqCst);
  }
}

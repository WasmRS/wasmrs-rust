use std::sync::atomic::{AtomicU32, Ordering};

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
    pub fn get_size(&self) -> u32 {
        self.size.load(Ordering::SeqCst)
    }

    pub fn update_size(&self, size: u32) {
        self.size.store(size, Ordering::SeqCst);
    }

    pub fn get_start(&self) -> u32 {
        self.start.load(Ordering::SeqCst)
    }

    pub fn update_start(&self, position: u32) {
        self.start.store(position, Ordering::SeqCst);
    }
}

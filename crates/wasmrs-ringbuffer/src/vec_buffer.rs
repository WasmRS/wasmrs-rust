/// The capacity of a `RingBuffer` created by new or default (`1024`).
// must be a power of 2
pub(crate) const RINGBUFFER_DEFAULT_CAPACITY: usize = 1024;

use crate::mask;
use crate::ring_buffer::{ReadOnlyRingBuffer, RingBuffer};

#[derive(Debug)]
#[must_use]
pub struct VecRingBuffer<T>
where
    T: Copy,
{
    buf: Vec<T>,
    capacity: usize,
    readptr: usize,
    writeptr: usize,
}

impl std::io::Read for VecRingBuffer<u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = buf.len();
        for el in buf {
            *el = self.next();
        }
        Ok(size)
    }
}

impl<T> VecRingBuffer<T>
where
    T: Copy,
{
    pub fn resize<F>(&mut self, cap: usize, val: F)
    where
        F: FnMut() -> T,
    {
        assert_ne!(cap, 0, "Capacity must be greater than 0");
        assert!(cap.is_power_of_two(), "Capacity must be a power of two");

        self.buf.resize_with(cap, val);
    }

    #[inline]
    /// Creates a `VecRingBuffer` with a certain capacity. The capacity must be a power of two.
    /// # Panics
    /// Panics when capacity is zero or not a power of two.
    pub fn with_capacity(cap: usize) -> Self {
        assert_ne!(cap, 0, "Capacity must be greater than 0");
        assert!(cap.is_power_of_two(), "Capacity must be a power of two");

        Self {
            buf: Vec::with_capacity(cap),
            capacity: cap,
            readptr: 0,
            writeptr: 0,
        }
    }

    /// Creates an `VecRingBuffer` with a capacity of [`RINGBUFFER_DEFAULT_CAPACITY`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the internal buffer.
    #[inline]
    #[must_use]
    pub fn buffer(&self) -> &[T] {
        &self.buf
    }
}

impl<T> Extend<T> for VecRingBuffer<T>
where
    T: Copy,
{
    fn extend<A: IntoIterator<Item = T>>(&mut self, iter: A) {
        let iter = iter.into_iter();

        for i in iter {
            self.push(i);
        }
    }
}

impl<T> Default for VecRingBuffer<T>
where
    T: Copy,
{
    /// Creates a buffer with a capacity of [`crate::RINGBUFFER_DEFAULT_CAPACITY`].
    #[inline]
    fn default() -> Self {
        Self {
            buf: Vec::with_capacity(RINGBUFFER_DEFAULT_CAPACITY),
            capacity: RINGBUFFER_DEFAULT_CAPACITY,
            readptr: 0,
            writeptr: 0,
        }
    }
}

impl<T> ReadOnlyRingBuffer<T> for VecRingBuffer<T>
where
    T: Copy,
{
    fn update_read_pos(&mut self, position: usize) {
        self.readptr = position;
    }

    fn get_read_pos(&mut self) -> usize {
        self.readptr
    }

    fn len(&self) -> usize {
        self.writeptr - self.readptr
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.capacity
    }

    fn as_ptr(&self) -> usize {
        self.buf.as_ptr() as _
    }

    fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    fn next(&mut self) -> T {
        let index = mask(self.capacity, self.readptr);
        let res = self.buf[index];
        self.readptr += 1;

        res
    }
}

impl<T> RingBuffer<T> for VecRingBuffer<T>
where
    T: Copy,
{
    #[inline]
    fn push(&mut self, value: T) {
        let index = mask(self.capacity, self.writeptr);

        if index >= self.buf.len() {
            // Use push() if we're at the vec length.
            self.buf.push(value);
        } else {
            // Otherwise we can overwrite at an index.
            self.buf[index] = value;
        }

        self.writeptr += 1;
    }

    fn get_write_pos(&self) -> usize {
        self.writeptr
    }

    fn update_write_pos(&mut self, pos: usize) {
        self.writeptr = pos;
    }

    fn write<A: IntoIterator<Item = T>>(&mut self, iter: A) {
        let iter = iter.into_iter();

        for i in iter {
            self.push(i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ring_buffer::ReadOnlyRingBuffer;

    use super::{VecRingBuffer, RINGBUFFER_DEFAULT_CAPACITY};

    #[test]
    fn test_default() {
        let b: VecRingBuffer<u32> = VecRingBuffer::default();
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.capacity());
        assert_eq!(RINGBUFFER_DEFAULT_CAPACITY, b.buf.capacity());
        assert_eq!(b.capacity, b.capacity());
        assert_eq!(b.buf.len(), b.len());
        assert_eq!(0, b.writeptr);
        assert_eq!(0, b.readptr);
        assert!(b.buf.is_empty());
    }

    #[test]
    fn test_read() {
        let mut buffer: VecRingBuffer<u8> = VecRingBuffer::with_capacity(16);
        buffer.extend(vec![9, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
        let len: Vec<u8> = buffer.read(1).collect();
        assert_eq!(len, vec![9]);
        let payload: Vec<u8> = buffer.read(len[0] as _).collect();
        assert_eq!(payload, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        buffer.extend(vec![15, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
        let len: Vec<_> = buffer.read(1).collect();
        assert_eq!(len, &[15]);
        let payload: Vec<_> = buffer.read(len[0] as _).collect();
        assert_eq!(
            payload,
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
    }
}

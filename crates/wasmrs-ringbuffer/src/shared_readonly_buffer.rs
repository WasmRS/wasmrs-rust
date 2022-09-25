use crate::ring_buffer::ReadOnlyRingBuffer;

use crate::mask;

#[derive(Debug)]
pub struct SharedReadOnlyRingBuffer<'a, T>
where
    T: Copy,
{
    buf: &'a [T],
    ring_start: usize,
    ring_len: usize,
    readptr: usize,
}

impl<'a, T> ReadOnlyRingBuffer<T> for SharedReadOnlyRingBuffer<'a, T>
where
    T: Copy,
{
    fn update_read_pos(&mut self, position: usize) {
        self.readptr = position;
    }

    fn get_read_pos(&self) -> usize {
        self.readptr
    }

    fn len(&self) -> usize {
        self.capacity()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.ring_len
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn is_full(&self) -> bool {
        true
    }

    fn as_ptr(&self) -> usize {
        self.buf.as_ptr() as _
    }

    fn next(&mut self) -> T {
        let index = self.ring_start + mask(self.ring_len, self.readptr);
        let res = self.buf[index];
        self.readptr += 1;

        res
    }
}

impl<'a> std::io::Read for SharedReadOnlyRingBuffer<'a, u8> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let size = buf.len();
        for el in buf {
            *el = self.next();
        }
        Ok(size)
    }
}

impl<'a, T> SharedReadOnlyRingBuffer<'a, T>
where
    T: Copy,
{
    /// Creates an `SharedRingBuffer` with an existing reference.
    #[inline]
    pub fn new(buf: &'a [T], ring_start: usize, ring_len: usize, readptr: usize) -> Self {
        assert_ne!(ring_len, 0, "Capacity must be greater than 0");
        assert!(
            ring_len.is_power_of_two(),
            "Capacity must be a power of two"
        );

        Self {
            buf,
            ring_start,
            ring_len,
            readptr,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ring_buffer::ReadOnlyRingBuffer;

    use super::SharedReadOnlyRingBuffer;

    #[test]
    fn test_default() {
        let mem: &mut [u8] = &mut [];
        let buff_len = 4;
        let buff_start = 2;
        let readptr = 2;
        let b = SharedReadOnlyRingBuffer::new(mem, buff_start, buff_len, readptr);
        assert_eq!(b.capacity(), buff_len);
        assert_eq!(b.readptr, readptr);
    }

    #[test]
    fn test_read() {
        let mem = &mut [1, 2, 3, 4, 5, 6, 7, 8];
        let buff_len = 4;
        let buff_start = 2;
        let readptr = 2;
        let mut b = SharedReadOnlyRingBuffer::new(mem, buff_start, buff_len, readptr);
        let bytes: Vec<_> = b.read(4).collect();
        assert_eq!(bytes, vec![5, 6, 3, 4]);
        let bytes: Vec<_> = b.read(2).collect();
        assert_eq!(bytes, vec![5, 6]);
        b.update_read_pos(0);
        let bytes: Vec<_> = b.read(4).collect();
        assert_eq!(bytes, vec![3, 4, 5, 6]);
    }
}

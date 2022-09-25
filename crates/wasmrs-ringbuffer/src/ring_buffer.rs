use crate::iter::RingBufferIterator;

pub trait RingBuffer<T>: Extend<T>
where
    Self: Sized,
{
    fn push(&mut self, value: T);

    fn get_write_pos(&self) -> usize;

    fn update_write_pos(&mut self, pos: usize);

    fn write_at<A: IntoIterator<Item = T>>(&mut self, position: usize, iter: A) {
        self.update_write_pos(position);
        self.write(iter);
    }

    fn write<A: IntoIterator<Item = T>>(&mut self, iter: A);
}

pub trait ReadOnlyRingBuffer<T>
where
    Self: Sized,
{
    fn next(&mut self) -> T;

    fn read(&mut self, len: usize) -> RingBufferIterator<'_, Self, T> {
        RingBufferIterator::new(self, len)
    }

    fn read_at(&mut self, index: usize, len: usize) -> RingBufferIterator<'_, Self, T> {
        self.update_read_pos(index);
        RingBufferIterator::new(self, len)
    }

    fn len(&self) -> usize;

    #[must_use]
    fn as_ptr(&self) -> usize;

    fn get_read_pos(&self) -> usize;

    fn update_read_pos(&mut self, position: usize);

    fn is_empty(&self) -> bool;

    // Capacity is how much you can store, vs length which is how much is stored.
    fn capacity(&self) -> usize;

    /// Returns true when the length of the ringbuffer equals the capacity.
    #[inline]
    fn is_full(&self) -> bool {
        self.len() == self.capacity()
    }
}

use crate::ring_buffer::ReadOnlyRingBuffer;

use core::marker::PhantomData;

#[allow(missing_debug_implementations)]
pub struct RingBufferIterator<'rb, RB, I>
where
    RB: ReadOnlyRingBuffer<I>,
{
    obj: &'rb mut RB,
    num: usize,
    done: usize,
    phantom: PhantomData<I>,
}

impl<'rb, RB, I> RingBufferIterator<'rb, RB, I>
where
    RB: ReadOnlyRingBuffer<I>,
{
    #[inline]
    pub(crate) fn new(obj: &'rb mut RB, num: usize) -> Self {
        Self {
            obj,
            num,
            done: 0,
            phantom: PhantomData::default(),
        }
    }
}

impl<'rb, RB, I> Iterator for RingBufferIterator<'rb, RB, I>
where
    RB: ReadOnlyRingBuffer<I>,
{
    type Item = I;

    fn next(&mut self) -> Option<I> {
        self.done += 1;
        if self.done > self.num {
            return None;
        }
        Some(self.obj.next())
    }
}

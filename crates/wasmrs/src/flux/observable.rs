use futures::Stream;

use crate::runtime::ConditionallySafe;

use super::{Flux, FluxPipe};

pub trait Observable<Item, Err>: Stream<Item = Result<Item, Err>> + ConditionallySafe
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
    Self: Sized,
{
    fn pipe(self, into: Flux<Item, Err>) -> FluxPipe<Item, Err, Self> {
        FluxPipe::new(self, into)
    }
}

use futures::Stream;

use super::{Flux, FluxPipe};
use crate::runtime::ConditionallySafe;

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

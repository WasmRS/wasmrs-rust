use futures::Stream;

use super::{FluxChannel, FluxPipe};
use wasmrs_runtime::ConditionallySafe;

/// The wasmrs-rx implementation of an Rx Observable trait
pub trait Observable<Item, Err>: Stream<Item = Result<Item, Err>> + ConditionallySafe
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
  Self: Sized,
{
  /// Pipe one [Flux] into another.
  fn pipe(self, into: FluxChannel<Item, Err>) -> FluxPipe<Item, Err, Self> {
    FluxPipe::new(self, into)
  }
}

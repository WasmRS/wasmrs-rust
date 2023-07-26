use futures::Stream;

use super::{FluxChannel, FluxPipe};
use wasmrs_runtime::ConditionallySendSync;

/// The wasmrs-rx implementation of an Rx Observable trait
pub trait Observable<Item, Err>: Stream<Item = Result<Item, Err>> + ConditionallySendSync
where
  Item: ConditionallySendSync,
  Err: ConditionallySendSync,
  Self: Sized,
{
  /// Pipe one [Flux] into another.
  fn pipe(self, into: FluxChannel<Item, Err>) -> FluxPipe<Item, Err, Self> {
    FluxPipe::new(self, into)
  }
}

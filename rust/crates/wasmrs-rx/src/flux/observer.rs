use super::Signal;
use crate::Error;
use wasmrs_runtime::ConditionallySafe;

/// The wasmrs-rx implementation of an Rx Observer trait
pub trait Observer<Item, Err>
where
  Item: ConditionallySafe,
  Err: ConditionallySafe,
{
  /// Send a complete [Signal]
  fn send_signal(&self, signal: Signal<Item, Err>) -> Result<(), Error>;

  /// Send a [Result] and have it map to an appropriate [Signal] variant.
  fn send_result(&self, result: Result<Item, Err>) -> Result<(), Error> {
    self.send_signal(match result {
      Ok(ok) => Signal::Ok(ok),
      Err(err) => Signal::Err(err),
    })
  }

  /// Send a successful value.
  fn send(&self, item: Item) -> Result<(), Error> {
    self.send_signal(Signal::Ok(item))
  }

  /// Send an error value.
  fn error(&self, err: Err) -> Result<(), Error> {
    self.send_signal(Signal::Err(err))
  }

  /// Mark the [Observer] as complete.
  fn complete(&self) {
    let _ = self.send_signal(Signal::Complete);
  }

  /// Returns true if the observer has been closed.
  fn is_complete(&self) -> bool;
}

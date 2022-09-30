use crate::{runtime::ConditionallySafe, Error};

use super::Signal;

pub trait Observer<Item, Err>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    fn is_complete(&self) -> bool;

    fn send_signal(&self, signal: Signal<Item, Err>) -> Result<(), Error>;
    fn send_result(&self, result: Result<Item, Err>) -> Result<(), Error> {
        self.send_signal(match result {
            Ok(ok) => Signal::Ok(ok),
            Err(err) => Signal::Err(err),
        })
    }

    fn send(&self, item: Item) -> Result<(), Error> {
        self.send_signal(Signal::Ok(item))
    }

    fn error(&self, err: Err) -> Result<(), Error> {
        self.send_signal(Signal::Err(err))
    }

    fn complete(&self) {
        let _ = self.send_signal(Signal::Complete);
    }
}

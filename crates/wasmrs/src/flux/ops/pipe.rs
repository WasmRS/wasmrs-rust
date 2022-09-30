use std::{pin::Pin, task::Poll};

use futures::{Stream, TryStreamExt};
use pin_project_lite::pin_project;

use crate::{flux::Flux, runtime::ConditionallySafe};

#[allow(missing_debug_implementations)]
pin_project! {
pub struct FluxPipe<Item, Err, From>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    #[pin]
    from: From,
    to: Flux<Item, Err>,
}
}

impl<Item, Err, From> FluxPipe<Item, Err, From>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
{
    pub fn new(from: From, to: Flux<Item, Err>) -> Self {
        Self { from, to }
    }
}

impl<Item, Err, From> FluxPipe<Item, Err, From>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
    From: Stream<Item = Result<Item, Err>>,
{
}

impl<Item, Err, From> Stream for FluxPipe<Item, Err, From>
where
    Item: ConditionallySafe,
    Err: ConditionallySafe,
    From: Stream<Item = Result<Item, Err>> + Unpin,
{
    type Item = Result<Item, Err>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        self.get_mut().from.try_poll_next_unpin(cx)
    }
}
#[cfg(all(test, not(target_family = "wasm")))]
mod test {

    use crate::{flux::Observer, Observable};

    use super::*;
    use anyhow::Result;
    use futures::StreamExt;

    #[tokio::test]
    async fn test_pipes() -> Result<()> {
        let flux = Flux::<String, String>::new();

        let observer = flux.split_receiver()?;

        flux.send("First".to_owned())?;

        let mut second_flux = Flux::<String, String>::new();

        let mut pipe = observer.pipe(second_flux);

        let value = pipe.next().await;
        assert_eq!(value, Some(Ok("First".to_owned())));
        Ok(())
    }
}

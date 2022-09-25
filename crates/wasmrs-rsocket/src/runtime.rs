use std::future::Future;

pub fn spawn<F>(task: F)
where
    F: Send + Future<Output = ()> + 'static,
{
    #[cfg(not(target_arch = "wasm32"))]
    tokio::spawn(task);
    // #[cfg(target_arch = "wasm32")]
    // yielding_executor::single_threaded::spawn(task);
}

//! WebAssembly implementations of wasmrs-runtime functions and structs.
#![allow(missing_docs)]

use std::cell::{RefCell, UnsafeCell};
use std::sync::Arc;

use futures_util::task::LocalSpawnExt;
use futures_util::Future;
pub type TaskHandle = ();

pub type BoxFuture<Output> = std::pin::Pin<Box<dyn Future<Output = Output> + 'static>>;

thread_local! {
  static LOCAL_POOL: UnsafeCell<futures_executor::LocalPool> = UnsafeCell::new(futures_executor::LocalPool::new());
  static SPAWNER: UnsafeCell<Option<futures_executor::LocalSpawner>> = UnsafeCell::new(None);
  static IS_RUNNING: AtomicBool = AtomicBool::new(false);
}

pub fn spawn<Fut>(future: Fut)
where
  Fut: Future<Output = ()> + ConditionallySafe + 'static,
{
  SPAWNER.with(|spawner| {
    #[allow(unsafe_code)]
    let spawner = unsafe { &mut *spawner.get() };
    match spawner {
      Some(spawner) => spawner
        .spawn_local(future)
        .expect("Could not spawn process in WASM runtime."),
      None => {
        LOCAL_POOL.with(|pool| {
          #[allow(unsafe_code)]
          let pool = unsafe { &mut *pool.get() };
          let s = pool.spawner();
          s.spawn_local(future).expect("Could not spawn process in WASM runtime.");
          spawner.replace(s)
        });
      }
    }
  });
}

#[allow(missing_copy_implementations, missing_debug_implementations)]
pub struct PendingOnce {
  is_ready: bool,
}

impl Future for PendingOnce {
  type Output = ();
  fn poll(mut self: std::pin::Pin<&mut Self>, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
    ctx.waker().wake_by_ref();
    if self.is_ready {
      std::task::Poll::Ready(())
    } else {
      self.is_ready = true;
      std::task::Poll::Pending
    }
  }
}

pub async fn yield_now() {
  PendingOnce { is_ready: false }.await
}

fn is_running() -> bool {
  IS_RUNNING.with(|cell| cell.load(std::sync::atomic::Ordering::SeqCst))
}

fn running_state(state: bool) {
  IS_RUNNING.with(|cell| cell.store(state, std::sync::atomic::Ordering::SeqCst));
}

pub fn exhaust_pool() {
  if !is_running() {
    running_state(true);
    LOCAL_POOL.with(|cell| {
      #[allow(unsafe_code)]
      let pool = unsafe { &mut *cell.get() };
      pool.run_until_stalled();
    });
    running_state(false);
  }
}

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;

#[allow(missing_debug_implementations)]
pub struct SafeMap<K, V>(UnsafeCell<HashMap<K, V>>)
where
  K: std::hash::Hash,
  K: Eq;

impl<K, V> SafeMap<K, V>
where
  K: std::hash::Hash,
  K: Eq,
{
  pub fn remove(&self, key: &K) -> Option<V> {
    #[allow(unsafe_code)]
    unsafe { &mut *self.0.get() }.remove(key)
  }
  pub fn insert(&self, key: K, value: V) {
    #[allow(unsafe_code)]
    unsafe { &mut *self.0.get() }.insert(key, value);
  }
  #[must_use]
  pub fn len(&self) -> usize {
    #[allow(unsafe_code)]
    unsafe { &mut *self.0.get() }.len()
  }
  #[must_use]
  pub fn is_empty(&self) -> bool {
    #[allow(unsafe_code)]
    unsafe { &mut *self.0.get() }.is_empty()
  }

  pub fn cloned(&self, key: &K) -> Option<V>
  where
    V: Clone,
  {
    #[allow(unsafe_code)]
    unsafe { &mut *self.0.get() }.get(key).map(|v| v.clone())
  }

  pub fn entry<'a>(&'a self, key: K) -> Entry<'a, K, V> {
    #[allow(unsafe_code)]
    let map = unsafe { &mut *self.0.get() };
    let entry = map.entry(key);
    let val = match entry {
      std::collections::hash_map::Entry::Occupied(v) => Entry::Occupied(OccupiedEntry(v)),
      std::collections::hash_map::Entry::Vacant(v) => Entry::Vacant(VacantEntry(v)),
    };
    val
  }
}

#[must_use]
#[allow(missing_debug_implementations)]
pub enum Entry<'a, K, V> {
  Occupied(OccupiedEntry<'a, K, V>),
  Vacant(VacantEntry<'a, K, V>),
}

#[allow(missing_debug_implementations)]
pub struct OccupiedEntry<'a, K, V>(std::collections::hash_map::OccupiedEntry<'a, K, V>);

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
  K: Eq,
  K: std::hash::Hash,
{
  pub fn get(&self) -> &V {
    self.0.get()
  }
  pub fn remove(self) -> V {
    self.0.remove()
  }
}

#[allow(missing_debug_implementations)]
pub struct VacantEntry<'a, K, V>(std::collections::hash_map::VacantEntry<'a, K, V>);

impl<K, V> Default for SafeMap<K, V>
where
  K: std::hash::Hash,
  K: Eq,
{
  fn default() -> Self {
    Self(Default::default())
  }
}

#[allow(missing_debug_implementations)]
pub struct OptionalMut<T>(Arc<RefCell<Option<T>>>);

impl<T> OptionalMut<T>
where
  T: ConditionallySafe,
{
  pub fn new(item: T) -> Self {
    Self(Arc::new(RefCell::new(Some(item))))
  }

  pub fn none() -> Self {
    Self(Arc::new(RefCell::new(None)))
  }

  pub fn take(&self) -> Option<T> {
    self.0.borrow_mut().take()
  }

  pub fn insert(&self, item: T) {
    let _ = self.0.borrow_mut().insert(item);
  }
}
impl<T> Clone for OptionalMut<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub trait ConditionallySafe: 'static {}

impl<S> ConditionallySafe for S where S: 'static {}

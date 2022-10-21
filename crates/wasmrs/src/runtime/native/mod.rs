use std::future::Future;
use std::sync::Arc;

use dashmap::DashMap;
use parking_lot::Mutex;
use tokio::task::JoinHandle;

pub type TaskHandle = JoinHandle<()>;

pub type BoxFuture<Output> = std::pin::Pin<Box<dyn Future<Output = Output> + Send + Sync + 'static>>;

pub fn spawn<F>(task: F) -> TaskHandle
where F: Future<Output = ()> + ConditionallySafe + 'static {
  tokio::spawn(task)
}

pub fn exhaust_pool() {
  unimplemented!("Not implemented in non-wasm hosts")
}

#[allow(missing_debug_implementations)]
pub struct SafeMap<K, V>(DashMap<K, V>)
where
  K: std::hash::Hash,
  K: Eq;

impl<K, V> SafeMap<K, V>
where
  K: std::hash::Hash,
  K: Eq,
{
  pub fn remove(&self, key: &K) -> Option<V> {
    self.0.remove(key).map(|v| v.1)
  }

  pub fn insert(&self, key: K, value: V) {
    self.0.insert(key, value);
  }

  #[must_use]
  pub fn len(&self) -> usize {
    self.0.len()
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }

  pub fn cloned(&self, key: &K) -> Option<V>
  where V: Clone {
    self.0.get(key).map(|v| v.clone())
  }

  pub fn entry(&self, key: K) -> Entry<'_, K, V> {
    match self.0.entry(key) {
      dashmap::mapref::entry::Entry::Occupied(v) => Entry::Occupied::<K, V>(OccupiedEntry(v)),
      dashmap::mapref::entry::Entry::Vacant(v) => Entry::Vacant::<K, V>(VacantEntry(v)),
    }
  }
}

#[must_use]
#[allow(missing_debug_implementations)]
pub enum Entry<'a, K, V> {
  Occupied(OccupiedEntry<'a, K, V>),
  Vacant(VacantEntry<'a, K, V>),
}

#[allow(missing_debug_implementations)]
pub struct OccupiedEntry<'a, K, V>(dashmap::mapref::entry::OccupiedEntry<'a, K, V>);

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
pub struct VacantEntry<'a, K, V>(dashmap::mapref::entry::VacantEntry<'a, K, V>);

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
pub(crate) struct OptionalMut<T>(Arc<Mutex<Option<T>>>);

impl<T> OptionalMut<T>
where T: Send
{
  pub(crate) fn new(item: T) -> Self {
    Self(Arc::new(Mutex::new(Some(item))))
  }

  pub(crate) fn none() -> Self {
    Self(Arc::new(Mutex::new(None)))
  }

  pub(crate) fn take(&self) -> Option<T> {
    self.0.lock().take()
  }

  pub(crate) fn insert(&self, item: T) {
    let _ = self.0.lock().insert(item);
  }
}

impl<T> Clone for OptionalMut<T> {
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

pub trait ConditionallySafe: Send + Sync + 'static {}

impl<S> ConditionallySafe for S where S: Send + Sync + 'static {}

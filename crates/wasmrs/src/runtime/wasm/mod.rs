use futures_util::task::LocalSpawnExt;
use futures_util::Future;
use std::cell::{RefCell, RefMut, UnsafeCell};
use std::sync::Arc;
pub type TaskHandle = ();

pub type BoxFuture<Output> = std::pin::Pin<Box<dyn Future<Output = Output> + 'static>>;

thread_local! {
  static SPAWNER: UnsafeCell<futures_executor::LocalPool> = UnsafeCell::new(futures_executor::LocalPool::new());
}

pub fn spawn<Fut>(future: Fut)
where
    Fut: Future<Output = ()> + ConditionallySafe + 'static,
{
    SPAWNER.with(|cell| {
        #[allow(unsafe_code)]
        let pool = unsafe { &mut *cell.get() };
        let spawner = pool.spawner();
        spawner
            .spawn_local(future)
            .expect("Could not spawn process in WASM runtime.");
    });
}

pub fn exhaust_pool() {
    SPAWNER.with(|cell| {
        #[allow(unsafe_code)]
        let pool = unsafe { &mut *cell.get() };
        pool.run_until_stalled();
    });
}

use std::collections::HashMap;

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
        unsafe { (&mut *self.0.get()) }.remove(key)
    }
    pub fn insert(&self, key: K, value: V) {
        #[allow(unsafe_code)]
        unsafe { (&mut *self.0.get()) }.insert(key, value);
    }
    #[must_use]
    pub fn len(&self) -> usize {
        #[allow(unsafe_code)]
        unsafe { (&mut *self.0.get()) }.len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        #[allow(unsafe_code)]
        unsafe { (&mut *self.0.get()) }.is_empty()
    }

    pub fn entry<'a>(&'a self, key: K) -> (Entry<'a, K, V>) {
        #[allow(unsafe_code)]
        let map = unsafe { (&mut *self.0.get()) };
        let entry = map.entry(key);
        let val = match entry {
            std::collections::hash_map::Entry::Occupied(v) => Entry::Occupied(OccupiedEntry(v)),
            std::collections::hash_map::Entry::Vacant(v) => Entry::Vacant(VacantEntry(v)),
        };
        (val)
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
    pub fn remove(mut self) -> V {
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
pub(crate) struct OptionalMut<T>(Arc<RefCell<Option<T>>>);

impl<T> OptionalMut<T>
where
    T: ConditionallySafe,
{
    pub(crate) fn new(item: T) -> Self {
        Self(Arc::new(RefCell::new(Some(item))))
    }

    pub(crate) fn take(&self) -> Option<T> {
        self.0.borrow_mut().take()
    }

    pub(crate) fn insert(&self, item: T) {
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

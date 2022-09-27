use std::cell::RefCell;
use std::future::Future;
use std::sync::Arc;

pub fn spawn<F>(_task: F)
where
    F: Send + Future<Output = ()> + 'static,
{
    todo!();
}

use std::collections::HashMap;

#[allow(missing_debug_implementations)]
pub struct SafeMap<K, V>(RefCell<HashMap<K, V>>)
where
    K: std::hash::Hash,
    K: Eq;

impl<K, V> SafeMap<K, V>
where
    K: std::hash::Hash,
    K: Eq,
{
    pub fn remove(&self, key: &K) -> Option<V> {
        self.0.borrow_mut().remove(key)
    }
    pub fn insert(&self, key: K, value: V) {
        self.0.borrow_mut().insert(key, value);
    }
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.borrow().len()
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.borrow().is_empty()
    }
}

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

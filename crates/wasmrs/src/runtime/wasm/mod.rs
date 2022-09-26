use std::cell::RefCell;
use std::future::Future;

pub fn spawn<F>(task: F)
where
    F: Send + Future<Output = ()> + 'static,
{
    todo!();
}

use std::{collections::HashMap, sync::Arc};

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

use std::{collections::HashMap, future::Future, sync::Arc};

use dashmap::{mapref::one::Ref, DashMap};
use parking_lot::Mutex;

pub fn spawn<F>(task: F)
where
    F: Send + Future<Output = ()> + 'static,
{
    tokio::spawn(task);
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

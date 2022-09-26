use std::{borrow::Borrow, collections::HashMap, sync::Arc};

use parking_lot::Mutex;

use crate::fragmentation::Joiner;

#[derive(Default)]
pub struct SafeMap<K, V>(Arc<Mutex<HashMap<K, V>>>)
where
    K: std::hash::Hash,
    K: Eq;

impl<K, V> SafeMap<K, V>
where
    K: std::hash::Hash,
    K: Eq,
{
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: std::hash::Hash + Eq,
    {
        self.0.lock().get(k)
    }

    pub fn get_mut<Q: ?Sized>(&self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: std::hash::Hash + Eq,
    {
        self.0.lock().get_mut(k)
    }
}

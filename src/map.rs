use core::hash::{BuildHasher, Hash};

pub trait Entry<'a, K, V> {
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V;
}

pub trait Map<K, V> {
    type Entry<'a>: Entry<'a, K, V>
    where
        Self: 'a;

    fn insert(&mut self, key: K, value: V) -> Option<V>;

    fn remove(&mut self, key: &K) -> Option<V>;

    fn entry(&mut self, key: K) -> Self::Entry<'_>;
}

impl<'a, K, V> Entry<'a, K, V> for std::collections::hash_map::Entry<'a, K, V> {
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        self.or_insert_with(default)
    }
}

impl<K, V, S> Map<K, V> for std::collections::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Entry<'a>
        = std::collections::hash_map::Entry<'a, K, V>
    where
        Self: 'a;

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn entry(&mut self, key: K) -> Self::Entry<'_> {
        self.entry(key)
    }
}

#[cfg(feature = "hashbrown")]
impl<'a, K, V, S> Entry<'a, K, V> for hashbrown::hash_map::Entry<'a, K, V, S>
where
    K: Hash,
    S: BuildHasher,
{
    fn or_insert_with<F>(self, default: F) -> &'a mut V
    where
        F: FnOnce() -> V,
    {
        self.or_insert_with(default)
    }
}

#[cfg(feature = "hashbrown")]
impl<K, V, S> Map<K, V> for hashbrown::HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Entry<'a>
        = hashbrown::hash_map::Entry<'a, K, V, S>
    where
        Self: 'a;

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        self.remove(key)
    }

    fn entry(&mut self, key: K) -> Self::Entry<'_> {
        self.entry(key)
    }
}

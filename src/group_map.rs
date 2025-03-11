#![cfg(feature = "use_std")]

use crate::map::{Entry, Map};
use std::iter::Iterator;

/// Return a `HashMap` of keys mapped to a list of their corresponding values.
///
/// See [`.into_group_map()`](crate::Itertools::into_group_map)
/// for more information.
pub fn into_group_map<I, K, V, M>(iter: I) -> M
where
    I: Iterator<Item = (K, V)>,
    M: Map<K, Vec<V>> + Default,
{
    let mut lookup = M::default();

    iter.for_each(|(key, val)| {
        lookup.entry(key).or_insert_with(Vec::new).push(val);
    });

    lookup
}

pub fn into_group_map_by<I, K, V, F, M>(iter: I, mut f: F) -> M
where
    I: Iterator<Item = V>,
    F: FnMut(&V) -> K,
    M: Map<K, Vec<V>> + Default,
{
    into_group_map(iter.map(|v| (f(&v), v)))
}

#![forbid(unsafe_code)]

use std::borrow::Borrow;
use std::mem::replace;
use std::ops::Index;
use std::vec::IntoIter;

////////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug, PartialEq, Eq)]
pub struct FlatMap<K, V>(Vec<(K, V)>);

impl<K: Ord, V> FlatMap<K, V> {
    pub fn new() -> Self {
        FlatMap(vec![])
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn as_slice(&self) -> &[(K, V)] {
        self.0.as_slice()
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match self.0.binary_search_by(|(k, _)| k.cmp(&key)) {
            Ok(ind) => Some(replace(&mut self.0[ind].1, value)),
            Err(ind) => {
                self.0.insert(ind, (key, value));
                None
            }
        }
    }

    pub fn get<B: Ord + ?Sized>(&self, key: &B) -> Option<&V>
    where
        K: Borrow<B>,
    {
        if let Ok(ind) = self.0.binary_search_by(|(k, _)| k.borrow().cmp(key)) {
            return Some(&self.0.get(ind).unwrap().1);
        }
        None
    }

    pub fn remove<B: Ord + ?Sized>(&mut self, key: &B) -> Option<V>
    where
        K: Borrow<B>,
    {
        if let Ok(ind) = self.0.binary_search_by(|(k, _)| k.borrow().cmp(key)) {
            return Some(self.0.remove(ind).1);
        }
        None
    }

    pub fn remove_entry<B: Ord + ?Sized>(&mut self, key: &B) -> Option<(K, V)>
    where
        K: Borrow<B>,
    {
        if let Ok(ind) = self.0.binary_search_by(|(k, _)| k.borrow().cmp(key)) {
            return Some(self.0.remove(ind));
        }
        None
    }
}

////////////////////////////////////////////////////////////////////////////////

impl<B: ?Sized + Ord, K: Ord + Borrow<B>, V> Index<&B> for FlatMap<K, V> {
    type Output = V;

    fn index(&self, index: &B) -> &Self::Output {
        self.get(index).unwrap()
    }
}

impl<K: Ord, V> Extend<(K, V)> for FlatMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K: Ord, V> From<Vec<(K, V)>> for FlatMap<K, V> {
    fn from(mut value: Vec<(K, V)>) -> Self {
        value.sort_by(|(a, _), (b, _)| a.cmp(b));
        value.reverse();
        value.dedup_by(|(a, _), (b, _)| a == b);
        value.reverse();
        FlatMap::<K, V>(value)
    }
}

impl<K: Ord, V> From<FlatMap<K, V>> for Vec<(K, V)> {
    fn from(value: FlatMap<K, V>) -> Self {
        value.0
    }
}

impl<K: Ord, V> FromIterator<(K, V)> for FlatMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        Self::from(Vec::from_iter(iter))
    }
}

impl<K: Ord, V> IntoIterator for FlatMap<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

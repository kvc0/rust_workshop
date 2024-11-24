use std::hash::Hash;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

use cache::ShareableCache;
use dashmap::DashMap;

struct Entry<K, V> {
    data: Mutex<Option<(K, V)>>,
    visited: AtomicBool,
}

pub struct ConcurrentSieveCache<K, V> {
    data: Vec<Entry<K, V>>,
    map: dashmap::DashMap<K, usize>,
    hand: Mutex<usize>,
}

impl<K, V> ConcurrentSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash + Clone,
    V: Send + Sync + Clone,
{
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(cache::MAX_SIZE);
        for _ in 0..cache::MAX_SIZE {
            data.push(Entry {
                data: Mutex::new(None),
                visited: AtomicBool::new(false),
            });
        }

        Self {
            data,
            map: DashMap::new(),
            hand: Mutex::new(0),
        }
    }
}

impl<K, V> ConcurrentSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash + Clone,
    V: Send + Sync + Clone,
{
    fn evict(&self) -> usize {
        let mut hand = self.hand.lock().unwrap();

        loop {
            let current = *hand;
            *hand += 1;
            if *hand >= self.data.len() {
                *hand = 0;
            }

            let entry = &self.data[current];
            let visited = entry.visited.swap(false, Ordering::Relaxed);

            if visited {
                continue;
            }

            return current;
        }
    }
}

impl<K, V> ShareableCache<K, V> for ConcurrentSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash + Clone,
    V: Send + Sync + Clone,
{
    fn get(&self, key: &K) -> Option<V> {
        let index = *self.map.get(key)?;
        let entry = &self.data[index];

        entry.visited.store(true, Ordering::Relaxed);
        let lock = entry.data.lock().expect("mutex was poisoned");
        let (_, value) = lock.as_ref()?;

        Some(value.clone())
    }

    fn set(&self, key: K, value: V) {
        let index = self.evict();
        let entry = &self.data[index];

        let mut data = entry.data.lock().unwrap();

        if let Some((key, _)) = data.take() {
            self.map.remove(&key);
        }

        let prev = self.map.insert(key.clone(), index);
        *data = Some((key, value));

        drop(data);

        if let Some(prev) = prev {
            *self.data[prev].data.lock().unwrap() = None;
        }
    }
}

impl<K, V> Default for ConcurrentSieveCache<K, V>
where
    K: Send + Sync + Eq + Hash + Clone,
    V: Send + Sync + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}

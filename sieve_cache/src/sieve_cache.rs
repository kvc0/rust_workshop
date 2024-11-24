use std::collections::HashMap;
use std::hash::Hash;

use cache::SizeLimitedCache;

struct Entry<Key, Value> {
    data: Option<(Key, Value)>,
    visited: bool,
}

pub struct SieveCache<Key, Value> {
    map: HashMap<Key, usize>,
    data: Vec<Entry<Key, Value>>,
    hand: usize,
}

impl<Key, Value> SieveCache<Key, Value> {
    pub fn new() -> Self {
        let mut data = Vec::with_capacity(cache::MAX_SIZE);
        for _ in 0..cache::MAX_SIZE {
            data.push(Entry {
                data: None,
                visited: false,
            });
        }

        Self {
            data,
            map: HashMap::default(),
            hand: 0,
        }
    }
}

impl<Key, Value> SieveCache<Key, Value>
where
    Key: Eq + Hash,
{
    fn evict(&mut self) -> usize {
        loop {
            let current = self.hand;
            self.hand += 1;
            if self.hand >= self.data.len() {
                self.hand = 0;
            }

            let entry = &mut self.data[current];
            if std::mem::replace(&mut entry.visited, false) {
                continue;
            }

            if let Some((key, _)) = entry.data.take() {
                self.map.remove(&key);
            }

            return current;
        }
    }
}

impl<Key, Value> SizeLimitedCache<Key, Value> for SieveCache<Key, Value>
// See the comment on SieveCache for commentary on "where clauses" in rust.
where
    Key: Clone + Eq + std::hash::Hash,
    Value: Clone,
{
    fn get(&mut self, key: &Key) -> Option<Value> {
        let index = *self.map.get(key)?;
        let entry = &mut self.data[index];

        entry.visited = true;
        let (_, value) = entry.data.as_ref()?;

        Some(value.clone())
    }

    fn set(&mut self, key: Key, value: Value) {
        if let Some(index) = self.map.get(&key) {
            let entry = &mut self.data[*index];
            entry.data = Some((key, value));
            return;
        }

        let index = self.evict();
        let entry = &mut self.data[index];
        entry.data = Some((key.clone(), value));
        self.map.insert(key, index);
    }
}

impl<Key, Value> Default for SieveCache<Key, Value> {
    fn default() -> Self {
        Self::new()
    }
}

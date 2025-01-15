use std::collections::HashMap;

#[derive(Debug)]
/// I'll need to revisit the way nested scopes & free list are handled
pub struct Varmap<'run, K, V> {
    storage: Vec<Option<(K, V)>>, // Underlying storage for map entries
    free_list: Vec<usize>,        // Stack of indices for free slots
    index_map: HashMap<K, usize>, // Maps keys to indices in the storage
    parent: Option<&'run mut Varmap<'run, K, V>>, // Parent map for nested scopes
}

impl<K: std::cmp::Eq + std::hash::Hash + Clone, V> Default for Varmap<'_, K, V> {
    fn default() -> Self {
        Self::new(None)
    }
}

impl<'run, K: std::cmp::Eq + std::hash::Hash + Clone, V> Varmap<'run, K, V> {
    /// Create a new Varmap.
    pub fn new(parent: Option<&'run mut Varmap<'run, K, V>>) -> Self {
        Self {
            storage: Vec::new(),
            free_list: Vec::new(),
            index_map: HashMap::new(),
            parent,
        }
    }

    /// Insert a new key-value pair into the map.
    pub fn insert(&mut self, key: K, value: V) {
        if let Some(&index) = self.index_map.get(&key) {
            // Update the value if the key already exists.
            self.storage[index] = Some((key.clone(), value));
        } else {
            // Use a free slot if available, otherwise push to the storage.
            let index = if let Some(free_index) = self.free_list.pop() {
                free_index
            } else {
                self.storage.push(None);
                self.storage.len() - 1
            };

            self.storage[index] = Some((key.clone(), value));
            self.index_map.insert(key, index);
        }
    }

    /// Remove a key-value pair from the map.
    pub fn remove(&mut self, key: K) -> Option<V> {
        if let Some(&index) = self.index_map.get(&key) {
            if let Some((_, value)) = self.storage[index].take() {
                // Add the slot to the free list and remove the key from the index map.
                self.free_list.push(index);
                self.index_map.remove(&key);
                return Some(value);
            }
        }
        None
    }

    /// Retrieve a value by key.
    pub fn get(&self, key: K) -> Option<&V> {
        self.index_map
            .get(&key)
            .and_then(|&index| self.storage[index].as_ref().map(|(_, v)| v))
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.get(key)))
    }

    /// Retrieve a mutable reference to a value by key.
    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if let Some(&index) = self.index_map.get(&key) {
            self.storage[index].as_mut().map(|(_, v)| v)
        } else {
            self.parent.as_mut().and_then(|parent| parent.get_mut(key))
        }
    }

    /// Check if the map contains a key.
    pub fn contains_key(&self, key: K) -> bool {
        self.index_map.contains_key(&key)
    }
}

use std::borrow::Borrow;
use std::fmt;

pub struct Table<T> {
    pub items: Vec<T>,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T> Iterator for Table<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.pop()
    }
}

impl<T> Table<T> {
    pub fn insert(&mut self, item: T)
    where
        T: PartialEq,
    {
        if self.has(&item) {
            return;
        }
        self.items.push(item)
    }
    pub fn get_index<K>(&self, item: &K) -> Option<usize>
    where
        T: Borrow<K>,
        K: PartialEq + ?Sized,
    {
        self.items.iter().position(|x| x.borrow() == item.borrow())
    }
    pub fn has<K>(&self, item: &K) -> bool
    where
        T: Borrow<K>,
        K: PartialEq,
    {
        self.items.iter().any(|x| x.borrow() == item)
    }
    pub fn retrieve(&self, idx: usize) -> Option<&T> {
        self.items.get(idx)
    }
    pub fn len(&self) -> usize {
        self.items.len()
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn clear(&mut self) {
        self.items.clear()
    }
    pub fn extend(&mut self, other: Table<T>) {
        self.items.extend(other.items);
    }
    pub fn remove<K>(&mut self, item: &K)
    where
        T: Borrow<K>,
        K: PartialEq,
    {
        self.items.remove(self.get_index(item).unwrap());
    }
}

impl<T: fmt::Debug> fmt::Debug for Table<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Table").field("Items", &self.items).finish()
    }
}

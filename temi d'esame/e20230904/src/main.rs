use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::{Mutex, RwLock};
use std::time::{Duration, Instant};

struct Cache<K: Eq + Hash, V> {
    vet: RwLock<HashMap<K, (V, Instant)>>,
}

impl<K: Eq + Hash, V> Cache<K, V> {
    pub fn new() -> Self {
        Cache {
            vet: RwLock::new(HashMap::new()),
        }
    }

    pub fn size(&self) -> usize {
        self.vet.read().unwrap().len()
    }

    pub fn put(&mut self, k: K, v: V, d: Duration) {
        let mut lock = self.vet.write().unwrap();
        //cleaning
        lock.retain(|_, v| v.1 > Instant::now());
        lock.insert(k, (v, Instant::now() + d));
        drop(lock);
    }

    pub fn renew(&mut self, k: K, d: Duration) -> bool {
        let mut lock = self.vet.write().unwrap();
        if let Some((v, _)) = lock.get_mut(&k) {
            *v = (v.clone(), Instant::now() + d);
            true
        } else {
            false
        }
    }

    pub fn get(&self, k: &K) -> Option<V> {}
}

fn main() {
    println!("Hello, world!");
}

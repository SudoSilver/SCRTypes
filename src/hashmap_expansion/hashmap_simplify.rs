use std::collections::HashMap;
use std::hash::Hash;

pub trait SimpleHashMap<K, V> {
    // F is a generic type for closures (|something| something!())
    // f is a variable holding F
    // K is a key of a HashMap
    // V is a value of a HashMap
    // FnOnce is a trait that defines how the closure can be called (at least once)
    fn if_get<F>(&self, key: &K, f: F) where F: FnOnce(&V);
    fn if_get_mut<F>(&mut self, key: &K, f: F) where F: FnOnce(&mut V);
}

impl<K,V> SimpleHashMap<K,V> for HashMap<K,V>
where K: Eq + Hash {
    fn if_get<F>(&self, key: &K, f: F) where F: FnOnce(&V) {
        if let Some(val) = self.get(key) {
            return f(val);
        }
    }

    fn if_get_mut<F>(&mut self, key: &K, f: F) where F: FnOnce(&mut V) {
        if let Some(val) = self.get_mut(key) {
            return f(val);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_hashmap_if_get() {
        let mut map = HashMap::new();
        map.insert("silver", 999);

        let mut called = false;
        map.if_get(&"silver", |val| {
            assert_eq!(*val, 999);
            called = true;
        });
        assert!(called);

        // Verify it safely skips on missing keys
        map.if_get(&"ghost", |_| {
            panic!("Should not execute for missing key");
        });
    }

    #[test]
    fn test_simple_hashmap_if_get_mut() {
        let mut map = HashMap::new();
        map.insert("ammo", 10);

        map.if_get_mut(&"ammo", |count| {
            *count += 5;
        });

        assert_eq!(map.get(&"ammo"), Some(&15));
    }
}

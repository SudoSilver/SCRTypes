use std::collections::HashMap;
use std::hash::Hash; // types that can be keys in a hashmap
use std::ops::{AddAssign, SubAssign}; // AddAssign is things you can add to, SubAssign is things you can subtract from

pub trait HashMapExpansion<K,T> {
    fn increment(&mut self, key: K, amount: T);
    fn decrement(&mut self, key: K, amount: T);
}

impl<K,V,T> HashMapExpansion<K,T> for HashMap<K, V>
where K: Eq + Clone + Hash,
V: AddAssign<T> + SubAssign<T> + Default {
    fn increment(&mut self, key: K, amount: T) {
        *self.entry(key).or_default() += amount;
    }

    fn decrement(&mut self, key: K, amount: T) {
        *self.entry(key).or_default() -= amount;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_increment_and_decrement_integers() {
        let mut scores: HashMap<String, i32> = HashMap::new();

        // Testing incrementing values that don't exist yet (initializes to 0 first!)
        scores.increment("Silver".to_string(), 10);
        scores.increment("Silver".to_string(), 5);

        // Testing decrementing back down
        scores.decrement("Silver".to_string(), 3);

        assert_eq!(scores.get("Silver"), Some(&12));
    }

    #[test]
    fn test_map_increment_and_decrement_floats() {
        let mut wallet: HashMap<&str, f64> = HashMap::new();

        // Checking that your generic trait bounds actually support floats natively
        wallet.increment("SOL", 2.5);
        wallet.decrement("SOL", 1.2);

        assert_eq!(wallet.get("SOL"), Some(&1.3));
    }
}

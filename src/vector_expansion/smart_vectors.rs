use std::collections::HashMap;
use std::hash::Hash;
use crate::boolean_expansion::boolean_extension::BoolExpansion;
use crate::hashmap_expansion::hashmap_extension::HashMapExpansion;

pub trait SmartVectors<T> {
    fn frequencies(&self) -> HashMap<T, usize>;
    fn dedup_clone(&self) -> Vec<T>;
    fn not_contains(&self, item: &T) -> bool;
}

impl<T> SmartVectors<T> for Vec<T>
where T: Clone + Eq + Hash {
    fn frequencies(&self) -> HashMap<T, usize> {
        let mut fq_map = HashMap::new();

        for item in self {
            fq_map.increment(item.clone(), 1);
        }
        return fq_map;
    }

    fn dedup_clone(&self) -> Vec<T> {
        let mut uniques = Vec::new();

        for item in self {
            if uniques.not_contains(item) {
                uniques.push(item.clone());
            }
        }
        return uniques;
    }
    fn not_contains(&self, item: &T) -> bool {
        return self.contains(item).flip();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_frequencies() {
        // Testing numbers
        let numbers = vec![1, 2, 2, 3, 3, 3];
        let fq_numbers = numbers.frequencies();

        let expected_numbers = HashMap::from([
            (1, 1),
                                             (2, 2),
                                             (3, 3),
        ]);
        assert_eq!(fq_numbers, expected_numbers);

        // Testing string slices
        let words = vec!["apple", "banana", "apple"];
        let fq_words = words.frequencies();

        let expected_words = HashMap::from([
            ("apple", 2),
                                           ("banana", 1),
        ]);
        assert_eq!(fq_words, expected_words);
    }

    #[test]
    fn test_dedup_clone() {
        let duplicates = vec![1, 1, 2, 3, 2, 4, 1];
        let result = duplicates.dedup_clone();

        let expected = vec![1, 2, 3, 4];
        assert_eq!(result, expected, "[TEST]: Vector deduplication failed to maintain unique values or sequence order");
    }

    #[test]
    fn test_not_contains() {
        let list = vec!["Silver", "Rust", "Ergonomics"];

        // This leverages your custom .flip() wrapper natively!
        assert!(list.not_contains(&"Python"));
        assert_eq!(list.not_contains(&"Rust"), false);
    }
}

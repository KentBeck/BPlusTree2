pub struct BPlusTreeMap<K, V> {
    root: Option<Node<K, V>>,
}

struct Node<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K, V> std::iter::FromIterator<(K, V)> for BPlusTreeMap<K, V>
where
    K: Ord,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = BPlusTreeMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

impl<K, V> Extend<(K, V)> for BPlusTreeMap<K, V>
where
    K: Ord,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        BPlusTreeMap { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.root {
            None => {
                let node = Node {
                    keys: vec![key],
                    values: vec![value],
                };
                self.root = Some(node);
                None
            }
            Some(node) => {
                // Find the position to insert
                let pos = match node.keys.binary_search(&key) {
                    Ok(pos) => {
                        // Key already exists, replace the value
                        let old_value = std::mem::replace(&mut node.values[pos], value);
                        return Some(old_value);
                    }
                    Err(pos) => pos,
                };

                // Insert the key and value at the found position
                node.keys.insert(pos, key);
                node.values.insert(pos, value);
                None
            }
        }
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &self.root {
            None => None,
            Some(node) => match node.keys.binary_search_by(|k| k.borrow().cmp(key)) {
                Ok(pos) => Some(&node.values[pos]),
                Err(_) => None,
            },
        }
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &mut self.root {
            None => None,
            Some(node) => {
                match node.keys.binary_search_by(|k| k.borrow().cmp(key)) {
                    Ok(pos) => {
                        // Remove the key and value at the found position
                        let _removed_key = node.keys.remove(pos);
                        let removed_value = node.values.remove(pos);

                        // If the node is now empty, set root to None
                        if node.keys.is_empty() {
                            self.root = None;
                        }

                        Some(removed_value)
                    }
                    Err(_) => None,
                }
            }
        }
    }

    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &self.root {
            None => false,
            Some(node) => match node.keys.binary_search_by(|k| k.borrow().cmp(key)) {
                Ok(_) => true,
                Err(_) => false,
            },
        }
    }

    pub fn len(&self) -> usize {
        match &self.root {
            None => 0,
            Some(node) => node.keys.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_create_empty_bplus_tree_map() {
        let _map: BPlusTreeMap<i32, String> = BPlusTreeMap::new();
        // Just testing that we can create an empty map without errors
    }

    #[test]
    fn test_insert_single_key_value_pair() {
        let mut map = BPlusTreeMap::new();
        let old_value = map.insert(1, "one".to_string());
        assert_eq!(old_value, None);
    }

    #[test]
    fn test_retrieve_value_by_key() {
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), None);
    }

    #[test]
    fn test_overwrite_existing_key_value() {
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());

        // Overwrite the existing key
        let old_value = map.insert(1, "new one".to_string());

        assert_eq!(old_value, Some("one".to_string()));
        assert_eq!(map.get(&1), Some(&"new one".to_string()));
    }

    #[test]
    fn test_remove_key_value_pair() {
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Remove an existing key
        let removed = map.remove(&1);
        assert_eq!(removed, Some("one".to_string()));
        assert_eq!(map.get(&1), None);
        assert_eq!(map.get(&2), Some(&"two".to_string()));

        // Try to remove a non-existent key
        let removed = map.remove(&3);
        assert_eq!(removed, None);

        // Remove the last key
        let removed = map.remove(&2);
        assert_eq!(removed, Some("two".to_string()));
        assert_eq!(map.get(&2), None);
    }

    #[test]
    fn test_checking_if_key_exists() {
        let mut map = BPlusTreeMap::new();

        // Empty map
        assert_eq!(map.contains_key(&1), false);

        // Add some keys
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Check existing and non-existing keys
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), false);

        // Remove a key and check again
        map.remove(&1);
        assert_eq!(map.contains_key(&1), false);
        assert_eq!(map.contains_key(&2), true);
    }

    #[test]
    fn test_getting_number_of_elements() {
        let mut map = BPlusTreeMap::new();

        // Empty map
        assert_eq!(map.len(), 0);

        // Add some elements
        map.insert(1, "one".to_string());
        assert_eq!(map.len(), 1);

        map.insert(2, "two".to_string());
        assert_eq!(map.len(), 2);

        // Overwrite an existing key (shouldn't change length)
        map.insert(1, "new one".to_string());
        assert_eq!(map.len(), 2);

        // Remove elements
        map.remove(&1);
        assert_eq!(map.len(), 1);

        map.remove(&2);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_checking_if_map_is_empty() {
        let mut map = BPlusTreeMap::new();

        // New map should be empty
        assert_eq!(map.is_empty(), true);

        // Add an element
        map.insert(1, "one".to_string());
        assert_eq!(map.is_empty(), false);

        // Remove the element
        map.remove(&1);
        assert_eq!(map.is_empty(), true);
    }

    #[test]
    fn test_creating_map_from_iterator() {
        // Create a vector of key-value pairs
        let pairs = vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (3, "three".to_string()),
        ];

        // Create a BPlusTreeMap from the iterator
        let map = BPlusTreeMap::from_iter(pairs);

        // Check that all elements were inserted correctly
        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), None);

        // Test with duplicate keys (later entries should overwrite earlier ones)
        let pairs_with_duplicates = vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (1, "new one".to_string()),
        ];

        let map = BPlusTreeMap::from_iter(pairs_with_duplicates);
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&"new one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_extending_map_with_iterator() {
        // Create a map with some initial elements
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Create a vector of additional key-value pairs
        let additional_pairs = vec![(3, "three".to_string()), (4, "four".to_string())];

        // Extend the map with the additional pairs
        map.extend(additional_pairs);

        // Check that all elements are in the map
        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));

        // Test extending with pairs that include existing keys
        let pairs_with_duplicates = vec![(2, "new two".to_string()), (5, "five".to_string())];

        map.extend(pairs_with_duplicates);

        // Check that existing keys were updated and new keys were added
        assert_eq!(map.len(), 5);
        assert_eq!(map.get(&2), Some(&"new two".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));
    }
}

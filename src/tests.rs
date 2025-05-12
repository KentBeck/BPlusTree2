// Tests for BPlusTreeMap

#[cfg(test)]
mod tests {
    use super::super::bplus_tree_map::{BPlusTreeMap, LeafNode};

    #[test]
    fn test_create_empty_bplus_tree_map() {
        let _map: BPlusTreeMap<i32, String> = BPlusTreeMap::new();
        // Just testing that we can create an empty map without errors
    }

    #[test]
    fn test_create_with_custom_branching_factor() {
        let _map = BPlusTreeMap::<i32, String>::with_branching_factor(8);
        // Just testing that we can create a map with a custom branching factor
    }

    #[test]
    #[should_panic(expected = "Branching factor must be at least 2")]
    fn test_invalid_branching_factor() {
        let _map = BPlusTreeMap::<i32, String>::with_branching_factor(1);
        // This should panic because branching factor must be at least 2
    }

    #[test]
    fn test_leaf_node_splitting() {
        // Create a map with a small branching factor
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(3);

        // Insert keys until we trigger a leaf split
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string()); // This should trigger a leaf split

        // Verify all keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
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

        // Insert some key-value pairs
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Test retrieving existing keys
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));

        // Test retrieving non-existent keys
        assert_eq!(map.get(&0), None);
        assert_eq!(map.get(&4), None);

        // Test with a different type that can be borrowed from the key type
        let key_ref: &i32 = &2;
        assert_eq!(map.get(key_ref), Some(&"two".to_string()));
    }

    #[test]
    fn test_branch_node_structure() {
        // Create leaf nodes
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        // Create a tree with a branch node as root and a custom branching factor
        let mut map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(4));

        // Insert a value that should go to the left leaf
        let old_value = map.insert(2, "new two".to_string());
        assert_eq!(old_value, Some("two".to_string())); // Should replace existing value

        // Insert a value that should go to the right leaf
        let old_value = map.insert(6, "six".to_string());
        assert_eq!(old_value, None); // Should be a new insertion
    }

    #[test]
    fn test_overwriting_existing_key_value() {
        let mut map = BPlusTreeMap::new();

        // Insert a key-value pair
        let old_value = map.insert(1, "one".to_string());
        assert_eq!(old_value, None);

        // Overwrite the existing key with a new value
        let old_value = map.insert(1, "new one".to_string());
        assert_eq!(old_value, Some("one".to_string()));

        // Verify the new value is accessible
        assert_eq!(map.get(&1), Some(&"new one".to_string()));
    }

    #[test]
    fn test_removing_key_value_pair() {
        let mut map = BPlusTreeMap::new();

        // Insert some key-value pairs
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Remove a key-value pair
        let removed_value = map.remove(&2);
        assert_eq!(removed_value, Some("two".to_string()));

        // Verify the key is no longer in the map
        assert_eq!(map.get(&2), None);

        // Verify other keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));

        // Try to remove a non-existent key
        let removed_value = map.remove(&4);
        assert_eq!(removed_value, None);
    }

    #[test]
    fn test_checking_if_key_exists() {
        let mut map = BPlusTreeMap::new();

        // Check if keys exist in an empty map
        assert_eq!(map.contains_key(&1), false);

        // Insert some key-value pairs
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Check if existing keys exist
        assert_eq!(map.contains_key(&1), true);
        assert_eq!(map.contains_key(&2), true);
        assert_eq!(map.contains_key(&3), true);

        // Check if non-existent keys exist
        assert_eq!(map.contains_key(&0), false);
        assert_eq!(map.contains_key(&4), false);

        // Remove a key and check if it still exists
        map.remove(&2);
        assert_eq!(map.contains_key(&2), false);
    }

    #[test]
    fn test_getting_number_of_elements() {
        let mut map = BPlusTreeMap::new();

        // Check len of an empty map
        assert_eq!(map.len(), 0);

        // Insert some key-value pairs and check len
        map.insert(1, "one".to_string());
        assert_eq!(map.len(), 1);

        map.insert(2, "two".to_string());
        assert_eq!(map.len(), 2);

        map.insert(3, "three".to_string());
        assert_eq!(map.len(), 3);

        // Overwrite an existing key and check len
        map.insert(2, "new two".to_string());
        assert_eq!(map.len(), 3); // Length should not change

        // Remove a key and check len
        map.remove(&1);
        assert_eq!(map.len(), 2);

        // Remove another key and check len
        map.remove(&3);
        assert_eq!(map.len(), 1);

        // Remove the last key and check len
        map.remove(&2);
        assert_eq!(map.len(), 0);
    }

    #[test]
    fn test_checking_if_map_is_empty() {
        let mut map = BPlusTreeMap::new();

        // Check if a new map is empty
        assert_eq!(map.is_empty(), true);

        // Insert a key-value pair and check if the map is empty
        map.insert(1, "one".to_string());
        assert_eq!(map.is_empty(), false);

        // Insert more key-value pairs and check if the map is empty
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());
        assert_eq!(map.is_empty(), false);

        // Remove keys and check if the map is empty
        map.remove(&1);
        assert_eq!(map.is_empty(), false);

        map.remove(&2);
        assert_eq!(map.is_empty(), false);

        // Remove the last key and check if the map is empty
        map.remove(&3);
        assert_eq!(map.is_empty(), true);
    }
}

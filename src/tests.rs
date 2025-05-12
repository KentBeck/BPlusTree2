// Tests for BPlusTreeMap

#[cfg(test)]
mod tests {
    use super::super::bplus_tree_map::{BPlusTreeMap, LeafNode};
    use std::iter::FromIterator;

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

    #[test]
    fn test_creating_bplus_tree_map_from_iterator() {
        // Create a vector of key-value pairs
        let pairs = vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (3, "three".to_string()),
        ];

        // Create a BPlusTreeMap from the iterator
        let map = BPlusTreeMap::from_iter(pairs);

        // Check that the map has the correct size
        assert_eq!(map.len(), 3);

        // Check that all key-value pairs are in the map
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));

        // Check that a non-existent key returns None
        assert_eq!(map.get(&4), None);

        // Test with an empty iterator
        let empty_pairs: Vec<(i32, String)> = Vec::new();
        let empty_map = BPlusTreeMap::from_iter(empty_pairs);
        assert_eq!(empty_map.len(), 0);
        assert_eq!(empty_map.is_empty(), true);

        // Test with duplicate keys (later entries should overwrite earlier ones)
        let duplicate_pairs = vec![
            (1, "one".to_string()),
            (2, "two".to_string()),
            (1, "new one".to_string()),
        ];
        let duplicate_map = BPlusTreeMap::from_iter(duplicate_pairs);
        assert_eq!(duplicate_map.len(), 2);
        assert_eq!(duplicate_map.get(&1), Some(&"new one".to_string()));
        assert_eq!(duplicate_map.get(&2), Some(&"two".to_string()));
    }

    #[test]
    fn test_extending_bplus_tree_map_with_iterator() {
        // Create an initial map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Check initial state
        assert_eq!(map.len(), 2);
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));

        // Create a vector of additional key-value pairs
        let additional_pairs = vec![(3, "three".to_string()), (4, "four".to_string())];

        // Extend the map with the iterator
        map.extend(additional_pairs);

        // Check that the map has been extended correctly
        assert_eq!(map.len(), 4);
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));

        // Test extending with an empty iterator
        let empty_pairs: Vec<(i32, String)> = Vec::new();
        map.extend(empty_pairs);

        // Check that the map remains unchanged
        assert_eq!(map.len(), 4);

        // Test extending with pairs that have duplicate keys (should overwrite existing values)
        let duplicate_pairs = vec![(2, "new two".to_string()), (5, "five".to_string())];
        map.extend(duplicate_pairs);

        // Check that the map has been updated correctly
        assert_eq!(map.len(), 5);
        assert_eq!(map.get(&2), Some(&"new two".to_string())); // Value should be updated
        assert_eq!(map.get(&5), Some(&"five".to_string())); // New key-value pair should be added
    }

    #[test]
    fn test_converting_bplus_tree_map_into_iterator() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Convert the map into an iterator
        let iter = map.into_iter();

        // Collect the iterator into a vector for easier testing
        let entries: Vec<(i32, String)> = iter.collect();

        // Check that all entries are present (order may vary)
        assert_eq!(entries.len(), 3);

        // Sort the entries by key for consistent testing
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Check each entry
        assert_eq!(sorted_entries[0], (1, "one".to_string()));
        assert_eq!(sorted_entries[1], (2, "two".to_string()));
        assert_eq!(sorted_entries[2], (3, "three".to_string()));

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_iter = empty_map.into_iter();
        let empty_entries: Vec<(i32, String)> = empty_iter.collect();
        assert_eq!(empty_entries.len(), 0);

        // Test with a map that has a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));
        let branch_iter = branch_map.into_iter();
        let branch_entries: Vec<(i32, String)> = branch_iter.collect();

        // Check that all entries are present
        assert_eq!(branch_entries.len(), 4);

        // Sort the entries by key for consistent testing
        let mut sorted_branch_entries = branch_entries.clone();
        sorted_branch_entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Check each entry
        assert_eq!(sorted_branch_entries[0], (1, "one".to_string()));
        assert_eq!(sorted_branch_entries[1], (2, "two".to_string()));
        assert_eq!(sorted_branch_entries[2], (4, "four".to_string()));
        assert_eq!(sorted_branch_entries[3], (5, "five".to_string()));
    }

    #[test]
    fn test_debug_formatting() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Format the map using Debug
        let debug_str = format!("{:?}", map);

        // Check that the debug string contains all key-value pairs
        // The exact format may vary, but it should contain all keys and values
        assert!(debug_str.contains("1"));
        assert!(debug_str.contains("one"));
        assert!(debug_str.contains("2"));
        assert!(debug_str.contains("two"));
        assert!(debug_str.contains("3"));
        assert!(debug_str.contains("three"));

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_debug_str = format!("{:?}", empty_map);

        // Empty map should be formatted as "{}"
        assert_eq!(empty_debug_str, "{}");

        // Test with a map that has a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));
        let branch_debug_str = format!("{:?}", branch_map);

        // Check that the debug string contains all key-value pairs
        assert!(branch_debug_str.contains("1"));
        assert!(branch_debug_str.contains("one"));
        assert!(branch_debug_str.contains("2"));
        assert!(branch_debug_str.contains("two"));
        assert!(branch_debug_str.contains("4"));
        assert!(branch_debug_str.contains("four"));
        assert!(branch_debug_str.contains("5"));
        assert!(branch_debug_str.contains("five"));
    }

    #[test]
    fn test_cloning_bplus_tree_map() {
        // Create a map with some key-value pairs
        let mut original_map = BPlusTreeMap::new();
        original_map.insert(1, "one".to_string());
        original_map.insert(2, "two".to_string());
        original_map.insert(3, "three".to_string());

        // Clone the map
        let cloned_map = original_map.clone();

        // Check that the cloned map has the same size
        assert_eq!(cloned_map.len(), original_map.len());

        // Check that all key-value pairs are in the cloned map
        assert_eq!(cloned_map.get(&1), Some(&"one".to_string()));
        assert_eq!(cloned_map.get(&2), Some(&"two".to_string()));
        assert_eq!(cloned_map.get(&3), Some(&"three".to_string()));

        // Modify the original map and check that the clone is not affected
        let old_value = original_map.insert(2, "new two".to_string());
        assert_eq!(old_value, Some("two".to_string()));
        assert_eq!(original_map.get(&2), Some(&"new two".to_string()));
        assert_eq!(cloned_map.get(&2), Some(&"two".to_string())); // Clone should still have the old value

        // Add a new key to the original map and check that the clone is not affected
        original_map.insert(4, "four".to_string());
        assert_eq!(original_map.len(), 4);
        assert_eq!(cloned_map.len(), 3);
        assert_eq!(original_map.get(&4), Some(&"four".to_string()));
        assert_eq!(cloned_map.get(&4), None);

        // Remove a key from the original map and check that the clone is not affected
        let removed_value = original_map.remove(&1);
        assert_eq!(removed_value, Some("one".to_string()));
        assert_eq!(original_map.get(&1), None);
        assert_eq!(cloned_map.get(&1), Some(&"one".to_string())); // Clone should still have the key

        // Test cloning an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let cloned_empty_map = empty_map.clone();
        assert_eq!(cloned_empty_map.len(), 0);
        assert_eq!(cloned_empty_map.is_empty(), true);

        // Test cloning a map with a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));
        let cloned_branch_map = branch_map.clone();

        // Check that the cloned map has the same size
        assert_eq!(cloned_branch_map.len(), branch_map.len());

        // Check that all key-value pairs are in the cloned map
        assert_eq!(cloned_branch_map.get(&1), Some(&"one".to_string()));
        assert_eq!(cloned_branch_map.get(&2), Some(&"two".to_string()));
        assert_eq!(cloned_branch_map.get(&4), Some(&"four".to_string()));
        assert_eq!(cloned_branch_map.get(&5), Some(&"five".to_string()));
    }

    #[test]
    fn test_creating_empty_bplus_tree_map_with_default() {
        // Create a map using Default trait
        let map: BPlusTreeMap<i32, String> = Default::default();

        // Check that the map is empty
        assert_eq!(map.len(), 0);
        assert_eq!(map.is_empty(), true);

        // Check that the map has the default branching factor (4)
        // We can't directly access the branching_factor field, so we'll test it indirectly
        // by inserting elements and checking that the map behaves as expected
        let mut map: BPlusTreeMap<i32, String> = Default::default();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());
        map.insert(4, "four".to_string());
        map.insert(5, "five".to_string());

        // Check that all elements are accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));

        // Check that the map has the correct size
        assert_eq!(map.len(), 5);

        // Test that Default::default() is equivalent to BPlusTreeMap::new()
        let default_map: BPlusTreeMap<i32, String> = Default::default();
        let new_map: BPlusTreeMap<i32, String> = BPlusTreeMap::new();

        // Both maps should be empty
        assert_eq!(default_map.len(), new_map.len());
        assert_eq!(default_map.is_empty(), new_map.is_empty());
    }

    #[test]
    fn test_indexing_syntax_with_index() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Test indexing syntax
        assert_eq!(&map[&1], "one");
        assert_eq!(&map[&2], "two");
        assert_eq!(&map[&3], "three");

        // Test with a more complex key type
        let mut string_map = BPlusTreeMap::new();
        string_map.insert("apple".to_string(), 1);
        string_map.insert("banana".to_string(), 2);
        string_map.insert("cherry".to_string(), 3);

        // Test indexing syntax with string keys
        assert_eq!(&string_map[&"apple".to_string()], &1);
        assert_eq!(&string_map[&"banana".to_string()], &2);
        assert_eq!(&string_map[&"cherry".to_string()], &3);

        // Test with string slices (using Borrow)
        assert_eq!(&string_map[&"apple" as &str], &1);
        assert_eq!(&string_map[&"banana" as &str], &2);
        assert_eq!(&string_map[&"cherry" as &str], &3);

        // Test with a map that has a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));

        // Test indexing syntax with branch node
        assert_eq!(&branch_map[&1], "one");
        assert_eq!(&branch_map[&2], "two");
        assert_eq!(&branch_map[&4], "four");
        assert_eq!(&branch_map[&5], "five");
    }

    #[test]
    #[should_panic(expected = "no entry found for key")]
    fn test_indexing_with_nonexistent_key() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // This should panic because the key doesn't exist
        let _ = &map[&3];
    }

    #[test]
    fn test_iterating_over_key_value_pairs() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Collect the iterator into a vector for easier testing
        let entries: Vec<(&i32, &String)> = map.iter().collect();

        // Check that all entries are present
        assert_eq!(entries.len(), 3);

        // Check that entries are in ascending order by key
        assert_eq!(entries[0], (&1, &"one".to_string()));
        assert_eq!(entries[1], (&2, &"two".to_string()));
        assert_eq!(entries[2], (&3, &"three".to_string()));

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_entries: Vec<(&i32, &String)> = empty_map.iter().collect();
        assert_eq!(empty_entries.len(), 0);

        // Test with a map that has a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));
        let branch_entries: Vec<(&i32, &String)> = branch_map.iter().collect();

        // Check that all entries are present
        assert_eq!(branch_entries.len(), 4);

        // Check that entries are in ascending order by key
        assert_eq!(branch_entries[0], (&1, &"one".to_string()));
        assert_eq!(branch_entries[1], (&2, &"two".to_string()));
        assert_eq!(branch_entries[2], (&4, &"four".to_string()));
        assert_eq!(branch_entries[3], (&5, &"five".to_string()));

        // Test that the iterator can be used multiple times
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // First iteration
        let mut count = 0;
        for (k, v) in map.iter() {
            count += 1;
            assert!(k == &1 || k == &2);
            assert!(v == &"one".to_string() || v == &"two".to_string());
        }
        assert_eq!(count, 2);

        // Second iteration
        let mut count = 0;
        for (k, v) in map.iter() {
            count += 1;
            assert!(k == &1 || k == &2);
            assert!(v == &"one".to_string() || v == &"two".to_string());
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_multi_level_tree_creation() {
        // Create a map with a small branching factor to force multiple levels
        let mut map = BPlusTreeMap::with_branching_factor(3);

        // Insert enough elements to create a tree with depth > 2
        // With branching factor 3, we need at least 9 elements to get to depth 3
        for i in 1..=20 {
            map.insert(i, format!("value_{}", i));
        }

        // Verify all elements are accessible
        for i in 1..=20 {
            assert_eq!(map.get(&i), Some(&format!("value_{}", i)));
        }

        // Verify the size is correct
        assert_eq!(map.len(), 20);

        // Verify iteration works correctly
        let entries: Vec<(&i32, &String)> = map.iter().collect();
        assert_eq!(entries.len(), 20);

        // Check that entries are in ascending order by key
        for i in 0..20 {
            assert_eq!(entries[i], (&(i as i32 + 1), &format!("value_{}", i + 1)));
        }

        // Test removing elements from the multi-level tree
        for i in 1..=10 {
            assert_eq!(map.remove(&i), Some(format!("value_{}", i)));
        }

        // Verify size after removal
        assert_eq!(map.len(), 10);

        // Verify remaining elements are still accessible
        for i in 11..=20 {
            assert_eq!(map.get(&i), Some(&format!("value_{}", i)));
        }

        // Verify removed elements are no longer accessible
        for i in 1..=10 {
            assert_eq!(map.get(&i), None);
        }

        // Test inserting new elements after removal
        for i in 1..=5 {
            map.insert(i, format!("new_value_{}", i));
        }

        // Verify size after insertion
        assert_eq!(map.len(), 15);

        // Verify new elements are accessible
        for i in 1..=5 {
            assert_eq!(map.get(&i), Some(&format!("new_value_{}", i)));
        }

        // Verify iteration still works correctly
        let entries: Vec<(&i32, &String)> = map.iter().collect();
        assert_eq!(entries.len(), 15);

        // Check that the first 5 entries are the newly inserted ones
        for i in 0..5 {
            assert_eq!(
                entries[i],
                (&(i as i32 + 1), &format!("new_value_{}", i + 1))
            );
        }

        // Check that the remaining entries are the original ones
        for i in 0..10 {
            assert_eq!(
                entries[i + 5],
                (&(i as i32 + 11), &format!("value_{}", i + 11))
            );
        }
    }
}

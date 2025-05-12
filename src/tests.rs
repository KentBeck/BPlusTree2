// Tests for BPlusTreeMap

mod node_operations_tests;
mod refactor_tests;

#[cfg(test)]
mod tests {
    use super::super::bplus_tree_map::{
        BPlusTreeMap, BranchNode, Entry, Iter, Keys, LeafNode, NodeVisitor, Values,
    };
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

    #[test]
    fn test_iterating_with_mutable_references() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());

        // Modify values using iter_mut
        for (_, value) in map.iter_mut() {
            *value = format!("modified_{}", value);
        }

        // Verify that values were modified
        assert_eq!(map.get(&1), Some(&"modified_one".to_string()));
        assert_eq!(map.get(&2), Some(&"modified_two".to_string()));
        assert_eq!(map.get(&3), Some(&"modified_three".to_string()));

        // Test with an empty map
        let mut empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_entries: Vec<(&i32, &mut String)> = empty_map.iter_mut().collect();
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

        let mut branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));

        // Modify values using iter_mut
        for (key, value) in branch_map.iter_mut() {
            *value = format!("modified_{}_{}", value, key);
        }

        // Verify that values were modified
        assert_eq!(branch_map.get(&1), Some(&"modified_one_1".to_string()));
        assert_eq!(branch_map.get(&2), Some(&"modified_two_2".to_string()));
        assert_eq!(branch_map.get(&4), Some(&"modified_four_4".to_string()));
        assert_eq!(branch_map.get(&5), Some(&"modified_five_5".to_string()));

        // Test with a multi-level tree
        let mut multi_level_map = BPlusTreeMap::with_branching_factor(3);
        for i in 1..=10 {
            multi_level_map.insert(i, format!("value_{}", i));
        }

        // Modify values using iter_mut
        for (key, value) in multi_level_map.iter_mut() {
            *value = format!("modified_{}_{}", value, key);
        }

        // Verify that values were modified
        for i in 1..=10 {
            assert_eq!(
                multi_level_map.get(&i),
                Some(&format!("modified_value_{}_{}", i, i))
            );
        }

        // Test that iter_mut can be used to selectively modify values
        let mut map = BPlusTreeMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        // Double only even values
        for (key, value) in map.iter_mut() {
            if *key % 2 == 0 {
                *value *= 2;
            }
        }

        // Verify that only even values were modified
        assert_eq!(map.get(&1), Some(&10));
        assert_eq!(map.get(&2), Some(&40)); // 20 * 2 = 40
        assert_eq!(map.get(&3), Some(&30));
    }

    #[test]
    fn test_iterating_over_keys_only() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Collect the keys into a vector for easier testing
        let keys: Vec<&i32> = map.keys().collect();

        // Check that all keys are present
        assert_eq!(keys.len(), 3);

        // Check that keys are in ascending order
        assert_eq!(keys[0], &1);
        assert_eq!(keys[1], &2);
        assert_eq!(keys[2], &3);

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_keys: Vec<&i32> = empty_map.keys().collect();
        assert_eq!(empty_keys.len(), 0);

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
        let branch_keys: Vec<&i32> = branch_map.keys().collect();

        // Check that all keys are present
        assert_eq!(branch_keys.len(), 4);

        // Check that keys are in ascending order
        assert_eq!(branch_keys[0], &1);
        assert_eq!(branch_keys[1], &2);
        assert_eq!(branch_keys[2], &4);
        assert_eq!(branch_keys[3], &5);

        // Test with a multi-level tree
        let mut multi_level_map = BPlusTreeMap::with_branching_factor(3);
        for i in 1..=10 {
            multi_level_map.insert(i, format!("value_{}", i));
        }

        let multi_level_keys: Vec<&i32> = multi_level_map.keys().collect();

        // Check that all keys are present
        assert_eq!(multi_level_keys.len(), 10);

        // Check that keys are in ascending order
        for i in 0..10 {
            assert_eq!(multi_level_keys[i], &(i as i32 + 1));
        }

        // Test that the keys iterator can be used multiple times
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // First iteration
        let mut count = 0;
        for k in map.keys() {
            count += 1;
            assert!(k == &1 || k == &2);
        }
        assert_eq!(count, 2);

        // Second iteration
        let mut count = 0;
        for k in map.keys() {
            count += 1;
            assert!(k == &1 || k == &2);
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn test_iterating_over_values_only() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Collect the values into a vector for easier testing
        let values: Vec<&String> = map.values().collect();

        // Check that all values are present
        assert_eq!(values.len(), 3);

        // Check that values are in order corresponding to ascending key order
        assert_eq!(values[0], &"one".to_string());
        assert_eq!(values[1], &"two".to_string());
        assert_eq!(values[2], &"three".to_string());

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_values: Vec<&String> = empty_map.values().collect();
        assert_eq!(empty_values.len(), 0);

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
        let branch_values: Vec<&String> = branch_map.values().collect();

        // Check that all values are present
        assert_eq!(branch_values.len(), 4);

        // Check that values are in order corresponding to ascending key order
        assert_eq!(branch_values[0], &"one".to_string());
        assert_eq!(branch_values[1], &"two".to_string());
        assert_eq!(branch_values[2], &"four".to_string());
        assert_eq!(branch_values[3], &"five".to_string());

        // Test with a multi-level tree
        let mut multi_level_map = BPlusTreeMap::with_branching_factor(3);
        for i in 1..=10 {
            multi_level_map.insert(i, format!("value_{}", i));
        }

        let multi_level_values: Vec<&String> = multi_level_map.values().collect();

        // Check that all values are present
        assert_eq!(multi_level_values.len(), 10);

        // Check that values are in order corresponding to ascending key order
        for i in 0..10 {
            assert_eq!(multi_level_values[i], &format!("value_{}", i + 1));
        }

        // Test that the values iterator can be used multiple times
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // First iteration
        let mut count = 0;
        for v in map.values() {
            count += 1;
            assert!(v == &"one".to_string() || v == &"two".to_string());
        }
        assert_eq!(count, 2);

        // Second iteration
        let mut count = 0;
        for v in map.values() {
            count += 1;
            assert!(v == &"one".to_string() || v == &"two".to_string());
        }
        assert_eq!(count, 2);

        // Test with different value types
        let mut int_map = BPlusTreeMap::new();
        int_map.insert("a", 1);
        int_map.insert("b", 2);
        int_map.insert("c", 3);

        let int_values: Vec<&i32> = int_map.values().collect();
        assert_eq!(int_values.len(), 3);
        assert_eq!(int_values[0], &1);
        assert_eq!(int_values[1], &2);
        assert_eq!(int_values[2], &3);
    }

    #[test]
    fn test_iterating_over_mutable_values() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Modify values using values_mut
        for value in map.values_mut() {
            *value = format!("modified_{}", value);
        }

        // Verify that values were modified
        assert_eq!(map.get(&1), Some(&"modified_one".to_string()));
        assert_eq!(map.get(&2), Some(&"modified_two".to_string()));
        assert_eq!(map.get(&3), Some(&"modified_three".to_string()));

        // Test with an empty map
        let mut empty_map = BPlusTreeMap::<i32, String>::new();
        let empty_values: Vec<&mut String> = empty_map.values_mut().collect();
        assert_eq!(empty_values.len(), 0);

        // Test with a map that has a branch node as root
        let left_leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        let right_leaf = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        let mut branch_map = BPlusTreeMap::with_branch_root(3, left_leaf, right_leaf, Some(3));

        // Modify values using values_mut
        for value in branch_map.values_mut() {
            *value = format!("modified_{}", value);
        }

        // Verify that values were modified
        assert_eq!(branch_map.get(&1), Some(&"modified_one".to_string()));
        assert_eq!(branch_map.get(&2), Some(&"modified_two".to_string()));
        assert_eq!(branch_map.get(&4), Some(&"modified_four".to_string()));
        assert_eq!(branch_map.get(&5), Some(&"modified_five".to_string()));

        // Test with a multi-level tree
        let mut multi_level_map = BPlusTreeMap::with_branching_factor(3);
        for i in 1..=10 {
            multi_level_map.insert(i, format!("value_{}", i));
        }

        // Modify values using values_mut
        for value in multi_level_map.values_mut() {
            *value = format!("modified_{}", value);
        }

        // Verify that values were modified
        for i in 1..=10 {
            assert_eq!(
                multi_level_map.get(&i),
                Some(&format!("modified_value_{}", i))
            );
        }

        // Test that values_mut can be used to selectively modify values
        let mut map = BPlusTreeMap::new();
        map.insert(1, 10);
        map.insert(2, 20);
        map.insert(3, 30);

        // Double all values
        for value in map.values_mut() {
            *value *= 2;
        }

        // Verify that all values were modified
        assert_eq!(map.get(&1), Some(&20)); // 10 * 2 = 20
        assert_eq!(map.get(&2), Some(&40)); // 20 * 2 = 40
        assert_eq!(map.get(&3), Some(&60)); // 30 * 2 = 60

        // Test with different value types
        let mut string_map = BPlusTreeMap::new();
        string_map.insert(1, "one".to_string());
        string_map.insert(2, "two".to_string());
        string_map.insert(3, "three".to_string());

        // Modify string values
        for value in string_map.values_mut() {
            value.push_str("_modified");
        }

        // Verify that values were modified
        assert_eq!(string_map.get(&1), Some(&"one_modified".to_string()));
        assert_eq!(string_map.get(&2), Some(&"two_modified".to_string()));
        assert_eq!(string_map.get(&3), Some(&"three_modified".to_string()));
    }

    #[test]
    fn test_node_visitor_pattern() {
        // Create a custom visitor that counts the number of keys
        struct KeyCounter {
            count: usize,
        }

        impl NodeVisitor<i32, String> for KeyCounter {
            type Result = usize;

            fn visit_leaf(&mut self, leaf: &LeafNode<i32, String>) {
                self.count += leaf.keys.len();
            }

            fn visit_branch(&mut self, _branch: &BranchNode<i32, String>) {
                // No keys to count in branch nodes (we only count keys in leaf nodes)
            }

            fn result(self) -> Self::Result {
                self.count
            }
        }

        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Use the visitor to count keys
        let mut visitor = KeyCounter { count: 0 };
        map.accept(&mut visitor);
        assert_eq!(visitor.result(), 3);

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        let mut visitor = KeyCounter { count: 0 };
        empty_map.accept(&mut visitor);
        assert_eq!(visitor.result(), 0);

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
        let mut visitor = KeyCounter { count: 0 };
        branch_map.accept(&mut visitor);
        assert_eq!(visitor.result(), 4); // 2 keys in left leaf + 2 keys in right leaf

        // Test with a multi-level tree
        let mut multi_level_map = BPlusTreeMap::with_branching_factor(3);
        for i in 1..=10 {
            multi_level_map.insert(i, format!("value_{}", i));
        }

        let mut visitor = KeyCounter { count: 0 };
        multi_level_map.accept(&mut visitor);
        assert_eq!(visitor.result(), 10);

        // Create a custom visitor that transforms values
        struct ValueTransformer<F>
        where
            F: Fn(&String) -> String,
        {
            transform_fn: F,
            transformed_values: Vec<String>,
        }

        impl<F> NodeVisitor<i32, String> for ValueTransformer<F>
        where
            F: Fn(&String) -> String,
        {
            type Result = Vec<String>;

            fn visit_leaf(&mut self, leaf: &LeafNode<i32, String>) {
                for value in &leaf.values {
                    self.transformed_values.push((self.transform_fn)(value));
                }
            }

            fn visit_branch(&mut self, _branch: &BranchNode<i32, String>) {
                // No values to transform in branch nodes
            }

            fn result(self) -> Self::Result {
                self.transformed_values
            }
        }

        // Use the visitor to transform values
        let mut visitor = ValueTransformer {
            transform_fn: |s| format!("transformed_{}", s),
            transformed_values: Vec::new(),
        };
        map.accept(&mut visitor);

        // Check that all values were transformed
        let transformed_values = visitor.result();
        assert_eq!(transformed_values.len(), 3);
        assert!(transformed_values.contains(&"transformed_one".to_string()));
        assert!(transformed_values.contains(&"transformed_two".to_string()));
        assert!(transformed_values.contains(&"transformed_three".to_string()));
    }

    #[test]
    fn test_entry_api() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Test or_insert with an existing key
        let entry = map.entry(1);
        let value = entry.or_insert("default".to_string());
        *value = "modified_one".to_string();
        assert_eq!(map.get(&1), Some(&"modified_one".to_string()));

        // Test or_insert with a new key
        let entry = map.entry(3);
        let _value = entry.or_insert("three".to_string());
        assert_eq!(map.get(&3), Some(&"three".to_string()));

        // Test or_insert_with with an existing key
        let entry = map.entry(2);
        let value = entry.or_insert_with(|| "default".to_string());
        *value = "modified_two".to_string();
        assert_eq!(map.get(&2), Some(&"modified_two".to_string()));

        // Test or_insert_with with a new key
        let entry = map.entry(4);
        let _value = entry.or_insert_with(|| "four".to_string());
        assert_eq!(map.get(&4), Some(&"four".to_string()));

        // Test or_insert_with_key with an existing key
        let entry = map.entry(1);
        let _value = entry.or_insert_with_key(|k| format!("key_{}", k));
        assert_eq!(map.get(&1), Some(&"modified_one".to_string())); // Value should not change

        // Test or_insert_with_key with a new key
        let entry = map.entry(5);
        let _value = entry.or_insert_with_key(|k| format!("key_{}", k));
        assert_eq!(map.get(&5), Some(&"key_5".to_string()));

        // Test and_modify with an existing key
        let entry = map.entry(1);
        let _value = entry
            .and_modify(|v| *v = format!("modified_{}", v))
            .or_insert("default".to_string());
        assert_eq!(map.get(&1), Some(&"modified_modified_one".to_string()));

        // Test and_modify with a new key
        let entry = map.entry(6);
        let _value = entry
            .and_modify(|v| *v = format!("modified_{}", v))
            .or_insert("six".to_string());
        assert_eq!(map.get(&6), Some(&"six".to_string())); // and_modify should not be called

        // Test OccupiedEntry methods
        let mut map = BPlusTreeMap::new();
        map.insert(1, "one".to_string());

        // Test get and get_mut
        match map.entry(1) {
            Entry::Occupied(mut entry) => {
                assert_eq!(entry.get(), &"one".to_string());
                *entry.get_mut() = "modified_one".to_string();
                assert_eq!(entry.get(), &"modified_one".to_string());
            }
            Entry::Vacant(_) => panic!("Expected Occupied entry"),
        }

        // Test insert
        match map.entry(1) {
            Entry::Occupied(mut entry) => {
                let old_value = entry.insert("new_one".to_string());
                assert_eq!(old_value, "modified_one".to_string());
                assert_eq!(entry.get(), &"new_one".to_string());
            }
            Entry::Vacant(_) => panic!("Expected Occupied entry"),
        }

        // Test remove
        match map.entry(1) {
            Entry::Occupied(entry) => {
                let value = entry.remove();
                assert_eq!(value, "new_one".to_string());
                assert_eq!(map.get(&1), None);
            }
            Entry::Vacant(_) => panic!("Expected Occupied entry"),
        }

        // Test VacantEntry methods
        let mut map = BPlusTreeMap::new();

        // Test key
        match map.entry(1) {
            Entry::Occupied(_) => panic!("Expected Vacant entry"),
            Entry::Vacant(entry) => {
                assert_eq!(entry.key(), &1);
            }
        }

        // Test insert
        match map.entry(1) {
            Entry::Occupied(_) => panic!("Expected Vacant entry"),
            Entry::Vacant(entry) => {
                let value = entry.insert("one".to_string());
                *value = "modified_one".to_string();
                assert_eq!(map.get(&1), Some(&"modified_one".to_string()));
            }
        }
    }

    #[test]
    fn test_common_iterator_abstraction() {
        // Create a map with some key-value pairs
        let mut map = BPlusTreeMap::new();
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());

        // Test the iter method with the new Iter type
        let entries: Vec<(&i32, &String)> = map.iter().collect();
        assert_eq!(entries.len(), 3);
        assert_eq!(entries[0], (&1, &"one".to_string()));
        assert_eq!(entries[1], (&2, &"two".to_string()));
        assert_eq!(entries[2], (&3, &"three".to_string()));

        // Test the keys method with the new Keys type
        let keys: Vec<&i32> = map.keys().collect();
        assert_eq!(keys.len(), 3);
        assert_eq!(keys[0], &1);
        assert_eq!(keys[1], &2);
        assert_eq!(keys[2], &3);

        // Test the values method with the new Values type
        let values: Vec<&String> = map.values().collect();
        assert_eq!(values.len(), 3);
        assert_eq!(values[0], &"one".to_string());
        assert_eq!(values[1], &"two".to_string());
        assert_eq!(values[2], &"three".to_string());

        // Test the values_mut method with the new ValuesMut type
        let mut values_mut: Vec<&mut String> = map.values_mut().collect();
        assert_eq!(values_mut.len(), 3);

        // Modify the values through the mutable references
        for value in &mut values_mut {
            **value = format!("modified_{}", value);
        }

        // Verify that the values were modified
        assert_eq!(map.get(&1), Some(&"modified_one".to_string()));
        assert_eq!(map.get(&2), Some(&"modified_two".to_string()));
        assert_eq!(map.get(&3), Some(&"modified_three".to_string()));

        // Test the into_iter method with the new IntoIter type
        let map_clone = map.clone();
        let entries: Vec<(i32, String)> = map_clone.into_iter().collect();
        assert_eq!(entries.len(), 3);

        // Sort the entries by key for consistent testing
        let mut sorted_entries = entries.clone();
        sorted_entries.sort_by(|a, b| a.0.cmp(&b.0));

        assert_eq!(sorted_entries[0], (1, "modified_one".to_string()));
        assert_eq!(sorted_entries[1], (2, "modified_two".to_string()));
        assert_eq!(sorted_entries[2], (3, "modified_three".to_string()));

        // Test with an empty map
        let empty_map = BPlusTreeMap::<i32, String>::new();
        assert_eq!(empty_map.iter().count(), 0);
        assert_eq!(empty_map.keys().count(), 0);
        assert_eq!(empty_map.values().count(), 0);

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

        // Test iter
        let entries: Vec<(&i32, &String)> = branch_map.iter().collect();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0], (&1, &"one".to_string()));
        assert_eq!(entries[1], (&2, &"two".to_string()));
        assert_eq!(entries[2], (&4, &"four".to_string()));
        assert_eq!(entries[3], (&5, &"five".to_string()));

        // Test keys
        let keys: Vec<&i32> = branch_map.keys().collect();
        assert_eq!(keys.len(), 4);
        assert_eq!(keys[0], &1);
        assert_eq!(keys[1], &2);
        assert_eq!(keys[2], &4);
        assert_eq!(keys[3], &5);

        // Test values
        let values: Vec<&String> = branch_map.values().collect();
        assert_eq!(values.len(), 4);
        assert_eq!(values[0], &"one".to_string());
        assert_eq!(values[1], &"two".to_string());
        assert_eq!(values[2], &"four".to_string());
        assert_eq!(values[3], &"five".to_string());
    }
}

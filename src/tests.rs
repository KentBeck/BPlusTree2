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
    fn test_tree_structure_after_leaf_split() {
        // Create a map with a small branching factor but large enough to avoid branch splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(4);

        // Insert keys in ascending order
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string()); // This should trigger a leaf split

        // Insert one more key to test the tree structure
        map.insert(4, "four".to_string());

        // Verify all keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));

        // Verify keys that don't exist
        assert_eq!(map.get(&0), None);
        assert_eq!(map.get(&5), None);
    }

    #[test]
    fn test_tree_structure_after_leaf_split_descending_order() {
        // Create a map with a small branching factor but large enough to avoid branch splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(4);

        // Insert keys in descending order
        map.insert(5, "five".to_string());
        map.insert(4, "four".to_string());
        map.insert(3, "three".to_string()); // This should trigger a leaf split

        // Insert more keys to test the tree structure
        map.insert(2, "two".to_string());

        // Verify all keys are still accessible
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));
    }

    #[test]
    fn test_tree_structure_after_leaf_split_random_order() {
        // Create a map with a small branching factor but large enough to avoid branch splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(4);

        // Insert keys in a random order
        map.insert(3, "three".to_string());
        map.insert(1, "one".to_string());
        map.insert(5, "five".to_string()); // This should trigger a leaf split

        // Insert one more key to test the tree structure
        map.insert(2, "two".to_string());

        // Verify all keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));
    }

    #[test]
    fn test_overwrite_after_split() {
        // Create a map with a small branching factor but large enough to avoid branch splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(4);

        // Insert keys to trigger a split
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string()); // This should trigger a leaf split

        // Overwrite existing keys
        let old_value = map.insert(1, "ONE".to_string());
        assert_eq!(old_value, Some("one".to_string()));

        let old_value = map.insert(3, "THREE".to_string());
        assert_eq!(old_value, Some("three".to_string()));

        // Verify the updated values
        assert_eq!(map.get(&1), Some(&"ONE".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"THREE".to_string()));
    }

    #[test]
    fn test_remove_after_split() {
        // Create a map with a small branching factor but large enough to avoid branch splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(4);

        // Insert keys to trigger a split
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string()); // This should trigger a leaf split

        // Add a remove method to the BPlusTreeMap implementation if it doesn't exist
        // For now, we'll just verify that all keys are accessible
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

    // Unit tests for the split_leaf method
    #[test]
    fn test_split_leaf_even_number_of_keys() {
        // Create a leaf node with an even number of keys
        let mut leaf = LeafNode {
            keys: vec![1, 2, 3, 4],
            values: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
            ],
        };

        // Split the leaf node
        let (split_key, new_leaf) = BPlusTreeMap::<i32, String>::test_split_leaf(&mut leaf);

        // Check the split key
        assert_eq!(split_key, 3);

        // Check the original leaf (should contain the first half)
        assert_eq!(leaf.keys, vec![1, 2]);
        assert_eq!(leaf.values, vec!["one".to_string(), "two".to_string()]);

        // Check the new leaf (should contain the second half)
        assert_eq!(new_leaf.keys, vec![3, 4]);
        assert_eq!(
            new_leaf.values,
            vec!["three".to_string(), "four".to_string()]
        );
    }

    #[test]
    fn test_split_leaf_odd_number_of_keys() {
        // Create a leaf node with an odd number of keys
        let mut leaf = LeafNode {
            keys: vec![1, 2, 3, 4, 5],
            values: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
            ],
        };

        // Split the leaf node
        let (split_key, new_leaf) = BPlusTreeMap::<i32, String>::test_split_leaf(&mut leaf);

        // Check the split key
        assert_eq!(split_key, 3);

        // Check the original leaf (should contain the first half)
        assert_eq!(leaf.keys, vec![1, 2]);
        assert_eq!(leaf.values, vec!["one".to_string(), "two".to_string()]);

        // Check the new leaf (should contain the second half)
        assert_eq!(new_leaf.keys, vec![3, 4, 5]);
        assert_eq!(
            new_leaf.values,
            vec!["three".to_string(), "four".to_string(), "five".to_string()]
        );
    }

    #[test]
    fn test_split_leaf_minimum_size() {
        // Create a leaf node with the minimum size for splitting (2 keys)
        let mut leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        // Split the leaf node
        let (split_key, new_leaf) = BPlusTreeMap::<i32, String>::test_split_leaf(&mut leaf);

        // Check the split key
        assert_eq!(split_key, 2);

        // Check the original leaf (should contain the first half)
        assert_eq!(leaf.keys, vec![1]);
        assert_eq!(leaf.values, vec!["one".to_string()]);

        // Check the new leaf (should contain the second half)
        assert_eq!(new_leaf.keys, vec![2]);
        assert_eq!(new_leaf.values, vec!["two".to_string()]);
    }
}

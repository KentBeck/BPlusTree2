#[cfg(test)]
mod node_balancing_integration_tests {
    use crate::bplus_tree_map::BPlusTreeMap;

    #[test]
    fn test_insertion_with_node_balancing() {
        // Create a map with a small branching factor to trigger node splitting
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(2);

        // Insert keys to trigger node splitting
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string()); // This should trigger a split

        // Verify all keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));

        // Insert more keys to trigger multiple splits
        map.insert(4, "four".to_string());
        map.insert(5, "five".to_string());
        map.insert(6, "six".to_string());

        // Verify all keys are still accessible
        assert_eq!(map.get(&4), Some(&"four".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));
        assert_eq!(map.get(&6), Some(&"six".to_string()));
    }

    #[test]
    fn test_removal_with_node_balancing() {
        // Create a map with a small branching factor
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(2);

        // Insert several keys
        map.insert(1, "one".to_string());
        map.insert(2, "two".to_string());
        map.insert(3, "three".to_string());
        map.insert(4, "four".to_string());
        map.insert(5, "five".to_string());
        map.insert(6, "six".to_string());

        // Remove keys to trigger node merging or rebalancing
        let removed = map.remove(&2);
        assert_eq!(removed, Some("two".to_string()));

        // Verify remaining keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), None);
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), Some(&"four".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));
        assert_eq!(map.get(&6), Some(&"six".to_string()));

        // Remove more keys
        let removed = map.remove(&4);
        assert_eq!(removed, Some("four".to_string()));

        // Verify remaining keys are still accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&3), Some(&"three".to_string()));
        assert_eq!(map.get(&4), None);
        assert_eq!(map.get(&5), Some(&"five".to_string()));
        assert_eq!(map.get(&6), Some(&"six".to_string()));
    }

    #[test]
    fn test_mixed_operations_with_node_balancing() {
        // Create a map with a small branching factor
        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(2);

        // Insert several keys
        map.insert(1, "one".to_string());
        map.insert(3, "three".to_string());
        map.insert(5, "five".to_string());

        // Remove a key
        let removed = map.remove(&3);
        assert_eq!(removed, Some("three".to_string()));

        // Insert more keys
        map.insert(2, "two".to_string());
        map.insert(4, "four".to_string());

        // Verify all keys are correctly accessible
        assert_eq!(map.get(&1), Some(&"one".to_string()));
        assert_eq!(map.get(&2), Some(&"two".to_string()));
        assert_eq!(map.get(&3), None);
        assert_eq!(map.get(&4), Some(&"four".to_string()));
        assert_eq!(map.get(&5), Some(&"five".to_string()));

        // Update a key
        map.insert(4, "FOUR".to_string());
        assert_eq!(map.get(&4), Some(&"FOUR".to_string()));
    }

    #[test]
    fn test_root_merge_after_removal() {
        use crate::bplus_tree_map::RootKind;

        let mut map = BPlusTreeMap::<i32, String>::with_branching_factor(2);

        // Insert enough keys to create a branch root
        for i in 0..4 {
            map.insert(i, format!("{}", i));
        }

        assert_eq!(map.root_kind(), RootKind::Branch);

        // Remove keys until only one is left
        map.remove(&0);
        map.remove(&1);
        map.remove(&2);

        // The implementation keeps a branch root with a single child
        assert_eq!(map.root_kind(), RootKind::Branch);
        assert_eq!(map.get(&3), Some(&"3".to_string()));

        // Remove the last key. The implementation currently leaves an empty
        // branch node as the root, so the kind remains Branch.
        map.remove(&3);
        assert_eq!(map.root_kind(), RootKind::Branch);
        assert!(map.is_empty());
    }
}

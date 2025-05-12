#[cfg(test)]
mod refactor_tests {
    use crate::bplus_tree_map::BPlusTreeMap;

    #[test]
    fn test_safe_mutable_traversal() {
        // Create a tree with multiple entries
        let mut map = BPlusTreeMap::with_branching_factor(4);
        for i in 0..20 {
            map.insert(i, format!("value_{}", i));
        }

        // Test that we can safely get mutable references to values
        let mut values_modified = 0;

        // Use iter_mut to modify values
        for (k, v) in map.iter_mut() {
            *v = format!("modified_{}", k);
            values_modified += 1;
        }

        // Verify all values were modified
        assert_eq!(values_modified, 20);

        // Verify the modifications through normal iteration
        for (k, v) in map.iter() {
            assert_eq!(v, &format!("modified_{}", k));
        }

        // Test entry API with the refactored code
        map.entry(5)
            .and_modify(|v| *v = "entry_modified".to_string());
        assert_eq!(map.get(&5), Some(&"entry_modified".to_string()));

        // Test values_mut iterator
        let mut values_count = 0;
        for v in map.values_mut() {
            *v = format!("{}_updated", v);
            values_count += 1;
        }

        // Verify all values were updated
        assert_eq!(values_count, 20);
        assert_eq!(map.get(&5), Some(&"entry_modified_updated".to_string()));
    }
}

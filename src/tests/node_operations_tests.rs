#[cfg(test)]
mod node_operations_tests {
    use crate::bplus_tree_map::{BranchNode, LeafNode, Node};
    use crate::node_operations::{
        BranchNodeSplitter, LeafNodeMerger, LeafNodeSplitter, MergeResult, NodeMerger,
        NodeSplitter, SplitResult,
    };

    // Define a simple BranchNodeMerger for testing
    struct BranchNodeMerger {
        min_keys: usize,
    }

    impl BranchNodeMerger {
        fn new(branching_factor: usize) -> Self {
            let min_keys = branching_factor / 2;
            Self { min_keys }
        }
    }

    impl<K, V> NodeMerger<K, V, BranchNode<K, V>> for BranchNodeMerger
    where
        K: Ord + Clone,
        V: Clone,
    {
        fn needs_merge(&self, left: &BranchNode<K, V>, right: &BranchNode<K, V>) -> bool {
            left.keys.len() < self.min_keys || right.keys.len() < self.min_keys
        }

        fn merge(
            &self,
            mut left: BranchNode<K, V>,
            mut right: BranchNode<K, V>,
            separator: K,
        ) -> MergeResult<K, BranchNode<K, V>> {
            if !self.needs_merge(&left, &right) {
                return MergeResult::NoMerge::<K, BranchNode<K, V>> {
                    left,
                    right,
                    separator,
                };
            }

            // Merge the nodes
            left.keys.push(separator);
            left.keys.append(&mut right.keys);
            left.children.append(&mut right.children);

            MergeResult::Merged::<K, BranchNode<K, V>>(left)
        }
    }

    #[test]
    fn test_leaf_node_splitter() {
        // Create a leaf node with keys and values
        let leaf = LeafNode {
            keys: vec![1, 2, 3, 4, 5],
            values: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
                "five".to_string(),
            ],
        };

        // Create a splitter with branching factor 3
        let splitter = LeafNodeSplitter::new(3);

        // Check if the node needs splitting
        assert!(splitter.needs_split(&leaf));

        // Split the node
        let split_result = splitter.split(leaf);

        // Verify the split result
        match split_result {
            SplitResult::Split {
                left,
                right,
                separator,
            } => {
                // Check left node
                if let LeafNode { keys, values } = left {
                    assert_eq!(keys, vec![1, 2]);
                    assert_eq!(values, vec!["one".to_string(), "two".to_string()]);
                } else {
                    panic!("Expected left node to be a LeafNode");
                }

                // Check right node
                if let LeafNode { keys, values } = right {
                    assert_eq!(keys, vec![3, 4, 5]);
                    assert_eq!(
                        values,
                        vec!["three".to_string(), "four".to_string(), "five".to_string()]
                    );
                } else {
                    panic!("Expected right node to be a LeafNode");
                }

                // Check separator key
                assert_eq!(separator, 3);
            }
            SplitResult::NoSplit(_) => {
                panic!("Expected node to be split");
            }
        }
    }

    #[test]
    fn test_leaf_node_no_split_needed() {
        // Create a leaf node with keys and values
        let leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        // Create a splitter with branching factor 3
        let splitter = LeafNodeSplitter::new(3);

        // Check if the node needs splitting
        assert!(!splitter.needs_split(&leaf));

        // Split the node
        let split_result = splitter.split(leaf);

        // Verify the split result
        match split_result {
            SplitResult::NoSplit(node) => {
                // Check node is unchanged
                if let LeafNode { keys, values } = node {
                    assert_eq!(keys, vec![1, 2]);
                    assert_eq!(values, vec!["one".to_string(), "two".to_string()]);
                } else {
                    panic!("Expected node to be a LeafNode");
                }
            }
            SplitResult::Split { .. } => {
                panic!("Expected node not to be split");
            }
        }
    }

    #[test]
    fn test_branch_node_splitter() {
        // Create child leaf nodes
        let leaf1 = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };
        let leaf2 = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };
        let leaf3 = LeafNode {
            keys: vec![7, 8],
            values: vec!["seven".to_string(), "eight".to_string()],
        };
        let leaf4 = LeafNode {
            keys: vec![10, 11],
            values: vec!["ten".to_string(), "eleven".to_string()],
        };

        // Create a branch node with keys and children
        let branch = BranchNode {
            keys: vec![3, 6, 9],
            children: vec![
                crate::bplus_tree_map::Node::Leaf(leaf1),
                crate::bplus_tree_map::Node::Leaf(leaf2),
                crate::bplus_tree_map::Node::Leaf(leaf3),
                crate::bplus_tree_map::Node::Leaf(leaf4),
            ],
        };

        // Create a splitter with branching factor 2
        let splitter = BranchNodeSplitter::new(2);

        // Check if the node needs splitting
        assert!(splitter.needs_split(&branch));

        // Split the node
        let split_result = splitter.split(branch);

        // Verify the split result
        match split_result {
            SplitResult::Split {
                left,
                right,
                separator,
            } => {
                // Check left node
                if let BranchNode { keys, children } = left {
                    assert_eq!(keys.len(), 1);
                    assert_eq!(keys[0], 3);
                    assert_eq!(children.len(), 2);
                } else {
                    panic!("Expected left node to be a BranchNode");
                }

                // Check right node
                if let BranchNode { keys, children } = right {
                    assert_eq!(keys.len(), 1);
                    assert_eq!(keys[0], 9);
                    assert_eq!(children.len(), 2);
                } else {
                    panic!("Expected right node to be a BranchNode");
                }

                // Check separator key
                assert_eq!(separator, 6);
            }
            SplitResult::NoSplit(_) => {
                panic!("Expected node to be split");
            }
        }
    }

    #[test]
    fn test_branch_node_no_split_needed() {
        // Create child leaf nodes
        let leaf1 = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };
        let leaf2 = LeafNode {
            keys: vec![4, 5],
            values: vec!["four".to_string(), "five".to_string()],
        };

        // Create a branch node with keys and children
        let branch = BranchNode {
            keys: vec![3],
            children: vec![
                crate::bplus_tree_map::Node::Leaf(leaf1),
                crate::bplus_tree_map::Node::Leaf(leaf2),
            ],
        };

        // Create a splitter with branching factor 2
        let splitter = BranchNodeSplitter::new(2);

        // Check if the node needs splitting
        assert!(!splitter.needs_split(&branch));

        // Split the node
        let split_result = splitter.split(branch);

        // Verify the split result
        match split_result {
            SplitResult::NoSplit(node) => {
                // Check node is unchanged
                if let BranchNode { keys, children } = node {
                    assert_eq!(keys.len(), 1);
                    assert_eq!(keys[0], 3);
                    assert_eq!(children.len(), 2);
                } else {
                    panic!("Expected node to be a BranchNode");
                }
            }
            SplitResult::Split { .. } => {
                panic!("Expected node not to be split");
            }
        }
    }

    #[test]
    fn test_leaf_node_merger() {
        // Create leaf nodes
        let left = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };
        let right = LeafNode {
            keys: vec![3, 4],
            values: vec!["three".to_string(), "four".to_string()],
        };

        // Create a merger with branching factor 4
        let merger = LeafNodeMerger::new(4);

        // Check if the nodes need merging
        assert!(merger.needs_merge(&left, &right));

        // Merge the nodes
        let merge_result = merger.merge(left, right, 3);

        // Verify the merge result
        match merge_result {
            MergeResult::Merged(node) => {
                // Check merged node
                assert_eq!(node.keys, vec![1, 2, 3, 4]);
                assert_eq!(
                    node.values,
                    vec![
                        "one".to_string(),
                        "two".to_string(),
                        "three".to_string(),
                        "four".to_string()
                    ]
                );
            }
            _ => {
                panic!("Expected nodes to be merged");
            }
        }
    }

    #[test]
    fn test_leaf_node_rebalance() {
        // Create leaf nodes with uneven distribution
        let left = LeafNode {
            keys: vec![1, 2, 3, 4],
            values: vec![
                "one".to_string(),
                "two".to_string(),
                "three".to_string(),
                "four".to_string(),
            ],
        };
        let right = LeafNode {
            keys: vec![5],
            values: vec!["five".to_string()],
        };

        // Create a merger with branching factor 4
        let merger = LeafNodeMerger::new(4);

        // Check if the nodes need merging
        assert!(merger.needs_merge(&left, &right));

        // Merge the nodes
        let merge_result = merger.merge(left, right, 5);

        // Verify the rebalance result
        match merge_result {
            MergeResult::Rebalanced {
                left,
                right,
                separator,
            } => {
                // Check rebalanced nodes
                assert_eq!(left.keys, vec![1, 2]);
                assert_eq!(left.values, vec!["one".to_string(), "two".to_string()]);
                assert_eq!(right.keys, vec![3, 4, 5]);
                assert_eq!(
                    right.values,
                    vec!["three".to_string(), "four".to_string(), "five".to_string()]
                );
                assert_eq!(separator, 3);
            }
            _ => {
                panic!("Expected nodes to be rebalanced");
            }
        }
    }

    #[test]
    fn test_branch_node_merger() {
        // Create child leaf nodes
        let leaf1 = LeafNode {
            keys: vec![1],
            values: vec!["one".to_string()],
        };
        let leaf2 = LeafNode {
            keys: vec![3],
            values: vec!["three".to_string()],
        };
        let leaf3 = LeafNode {
            keys: vec![5],
            values: vec!["five".to_string()],
        };
        let leaf4 = LeafNode {
            keys: vec![7],
            values: vec!["seven".to_string()],
        };

        // Create branch nodes
        let left = BranchNode {
            keys: vec![2],
            children: vec![Node::Leaf(leaf1), Node::Leaf(leaf2)],
        };
        let right = BranchNode {
            keys: vec![6],
            children: vec![Node::Leaf(leaf3), Node::Leaf(leaf4)],
        };

        // Create a merger with branching factor 4
        let merger = BranchNodeMerger::new(4);

        // Check if the nodes need merging
        assert!(merger.needs_merge(&left, &right));

        // Merge the nodes with separator key 4
        let merge_result = merger.merge(left, right, 4);

        // Verify the merge result
        match merge_result {
            MergeResult::Merged(node) => {
                // Check merged node
                assert_eq!(node.keys, vec![2, 4, 6]);
                assert_eq!(node.children.len(), 4);
            }
            _ => {
                panic!("Expected nodes to be merged");
            }
        }
    }
}

#[cfg(test)]
mod node_balancer_tests {
    use crate::bplus_tree_map::{BranchNode, LeafNode, Node};
    use crate::node_balancer::{BalanceResult, InsertionBalancer, NodeBalancer, RemovalBalancer};
    use crate::node_operations::NodeMerger;

    #[test]
    fn test_insertion_balancer_leaf_node() {
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

        // Create an insertion balancer with branching factor 3
        let balancer = InsertionBalancer::new(3);

        // Balance the node
        let balance_result = balancer.balance_node(Node::Leaf(leaf));

        // Verify the balance result
        match balance_result {
            BalanceResult::Split {
                left,
                right,
                separator,
            } => {
                // Check left node
                match left {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, vec![1, 2]);
                        assert_eq!(leaf.values, vec!["one".to_string(), "two".to_string()]);
                    }
                    _ => panic!("Expected left node to be a LeafNode"),
                }

                // Check right node
                match right {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, vec![3, 4, 5]);
                        assert_eq!(
                            leaf.values,
                            vec!["three".to_string(), "four".to_string(), "five".to_string()]
                        );
                    }
                    _ => panic!("Expected right node to be a LeafNode"),
                }

                // Check separator key
                assert_eq!(separator, 3);
            }
            _ => panic!("Expected node to be split"),
        }
    }

    #[test]
    fn test_insertion_balancer_branch_node() {
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
                Node::Leaf(leaf1),
                Node::Leaf(leaf2),
                Node::Leaf(leaf3),
                Node::Leaf(leaf4),
            ],
        };

        // Create an insertion balancer with branching factor 2
        let balancer = InsertionBalancer::new(2);

        // Balance the node
        let balance_result = balancer.balance_node(Node::Branch(branch));

        // Verify the balance result
        match balance_result {
            BalanceResult::Split {
                left,
                right,
                separator,
            } => {
                // Check left node
                match left {
                    Node::Branch(branch) => {
                        assert_eq!(branch.keys.len(), 1);
                        assert_eq!(branch.keys[0], 3);
                        assert_eq!(branch.children.len(), 2);
                    }
                    _ => panic!("Expected left node to be a BranchNode"),
                }

                // Check right node
                match right {
                    Node::Branch(branch) => {
                        assert_eq!(branch.keys.len(), 1);
                        assert_eq!(branch.keys[0], 9);
                        assert_eq!(branch.children.len(), 2);
                    }
                    _ => panic!("Expected right node to be a BranchNode"),
                }

                // Check separator key
                assert_eq!(separator, 6);
            }
            _ => panic!("Expected node to be split"),
        }
    }

    #[test]
    fn test_insertion_balancer_no_split_needed() {
        // Create a leaf node with keys and values
        let leaf = LeafNode {
            keys: vec![1, 2],
            values: vec!["one".to_string(), "two".to_string()],
        };

        // Create an insertion balancer with branching factor 3
        let balancer = InsertionBalancer::new(3);

        // Balance the node
        let balance_result = balancer.balance_node(Node::Leaf(leaf));

        // Verify the balance result
        match balance_result {
            BalanceResult::NoChange(node) => match node {
                Node::Leaf(leaf) => {
                    assert_eq!(leaf.keys, vec![1, 2]);
                    assert_eq!(leaf.values, vec!["one".to_string(), "two".to_string()]);
                }
                _ => panic!("Expected node to be a LeafNode"),
            },
            _ => panic!("Expected no change to the node"),
        }
    }

    #[test]
    fn test_removal_balancer_merge_needed() {
        // Create leaf nodes with few keys
        let left = LeafNode {
            keys: vec![1],
            values: vec!["one".to_string()],
        };
        let right = LeafNode {
            keys: vec![3],
            values: vec!["three".to_string()],
        };

        // Create a removal balancer with min keys = 2
        let balancer = RemovalBalancer::new(4); // branching factor 4, min keys = 2

        // Balance the nodes
        let balance_result = balancer.balance_nodes(
            Node::Leaf(left),
            Node::Leaf(right),
            2, // separator key
        );

        // Verify the balance result
        match balance_result {
            BalanceResult::Merged(node) => match node {
                Node::Leaf(leaf) => {
                    assert_eq!(leaf.keys, vec![1, 3]);
                    assert_eq!(leaf.values, vec!["one".to_string(), "three".to_string()]);
                }
                _ => panic!("Expected node to be a LeafNode"),
            },
            _ => panic!("Expected nodes to be merged"),
        }
    }

    #[test]
    fn test_removal_balancer_rebalance_needed() {
        // Create leaf nodes with uneven distribution
        let left = LeafNode {
            keys: vec![1, 2, 3],
            values: vec!["one".to_string(), "two".to_string(), "three".to_string()],
        };
        let right = LeafNode {
            keys: vec![5],
            values: vec!["five".to_string()],
        };

        // Create a removal balancer with min keys = 2
        let balancer = RemovalBalancer::new(4); // branching factor 4, min keys = 2

        // Balance the nodes
        let balance_result = balancer.balance_nodes(
            Node::Leaf(left),
            Node::Leaf(right),
            4, // separator key
        );

        // Verify the balance result
        match balance_result {
            BalanceResult::Rebalanced {
                left,
                right,
                separator,
            } => {
                // Check left node
                match left {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, vec![1, 2]);
                        assert_eq!(leaf.values, vec!["one".to_string(), "two".to_string()]);
                    }
                    _ => panic!("Expected left node to be a LeafNode"),
                }

                // Check right node
                match right {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, vec![3, 5]);
                        assert_eq!(leaf.values, vec!["three".to_string(), "five".to_string()]);
                    }
                    _ => panic!("Expected right node to be a LeafNode"),
                }

                // Check separator key
                assert_eq!(separator, 3);
            }
            _ => panic!("Expected nodes to be rebalanced"),
        }
    }

    #[test]
    fn test_removal_balancer_no_change_needed() {
        // Create leaf nodes with sufficient keys (avoid using exactly 2 keys per node)
        let left = LeafNode {
            keys: vec![1, 3, 6],
            values: vec!["one".to_string(), "three".to_string(), "six".to_string()],
        };
        let right = LeafNode {
            keys: vec![4, 5, 7],
            values: vec!["four".to_string(), "five".to_string(), "seven".to_string()],
        };

        // Create a removal balancer with min keys = 2
        let balancer = RemovalBalancer::new(5); // branching factor 5, min keys = 2

        // Verify that the merger doesn't think these nodes need merging
        let merger = crate::node_operations::LeafNodeMerger::new(5);
        assert!(!merger.needs_merge(&left, &right));

        // Balance the nodes
        let balance_result = balancer.balance_nodes(
            Node::Leaf(left.clone()),
            Node::Leaf(right.clone()),
            3, // separator key
        );

        // Verify the balance result
        match balance_result {
            BalanceResult::Rebalanced {
                left: left_node,
                right: right_node,
                separator: sep,
            } => {
                // Check left node
                match left_node {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, left.keys);
                        assert_eq!(leaf.values, left.values);
                    }
                    _ => panic!("Expected left node to be a LeafNode"),
                }

                // Check right node
                match right_node {
                    Node::Leaf(leaf) => {
                        assert_eq!(leaf.keys, right.keys);
                        assert_eq!(leaf.values, right.values);
                    }
                    _ => panic!("Expected right node to be a LeafNode"),
                }

                // Check separator key
                assert_eq!(sep, 3);
            }
            _ => panic!("Expected nodes to be rebalanced"),
        }
    }
}

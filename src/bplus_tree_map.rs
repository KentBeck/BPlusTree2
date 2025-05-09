// Implementation of BPlusTreeMap

pub struct BPlusTreeMap<K, V> {
    root: Option<TreeNode<K, V>>,
}

// Enum to represent different types of nodes in the tree
pub(crate) enum TreeNode<K, V> {
    Branch(BranchNode<K, V>),
    Leaf(LeafNode<K, V>),
}

// Branch node holds keys and child nodes
pub(crate) struct BranchNode<K, V> {
    pub keys: Vec<K>,
    pub children: Vec<TreeNode<K, V>>,
}

// Leaf node holds keys and values
pub(crate) struct LeafNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}

impl<K, V> TreeNode<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    // Helper method to insert into a leaf node
    fn insert_into_leaf(leaf: &mut LeafNode<K, V>, key: K, value: V) -> Option<V> {
        let pos = match leaf.keys.binary_search(&key) {
            Ok(pos) => {
                // Key already exists, replace the value
                let old_value = std::mem::replace(&mut leaf.values[pos], value);
                return Some(old_value);
            }
            Err(pos) => pos,
        };

        // Insert the key and value at the found position
        leaf.keys.insert(pos, key);
        leaf.values.insert(pos, value);
        None
    }
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    pub fn new() -> Self {
        BPlusTreeMap { root: None }
    }

    // Helper method to create a tree with a branch node as root (for testing)
    #[cfg(test)]
    pub(crate) fn with_branch_root(
        key: K,
        left_leaf: LeafNode<K, V>,
        right_leaf: LeafNode<K, V>,
    ) -> Self {
        let branch = BranchNode {
            keys: vec![key],
            children: vec![TreeNode::Leaf(left_leaf), TreeNode::Leaf(right_leaf)],
        };

        BPlusTreeMap {
            root: Some(TreeNode::Branch(branch)),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.root {
            None => {
                // Create a new leaf node as the root
                let leaf_node = LeafNode {
                    keys: vec![key],
                    values: vec![value],
                };
                self.root = Some(TreeNode::Leaf(leaf_node));
                None
            }
            Some(tree_node) => {
                match tree_node {
                    TreeNode::Leaf(leaf) => {
                        // Insert directly into the leaf node
                        TreeNode::insert_into_leaf(leaf, key, value)
                    }
                    TreeNode::Branch(branch) => {
                        // Find the appropriate child node to insert into
                        let child_index = match branch.keys.binary_search(&key) {
                            Ok(idx) => idx + 1, // If key exists, go to the right child
                            Err(idx) => idx, // If key doesn't exist, go to the child at the insertion point
                        };

                        // If we don't have enough children, create a new leaf node
                        if child_index >= branch.children.len() {
                            let new_leaf = LeafNode {
                                keys: vec![key.clone()],
                                values: vec![value.clone()],
                            };
                            branch.children.push(TreeNode::Leaf(new_leaf));

                            // Also add the key to the branch node
                            if child_index > 0 {
                                branch.keys.push(key);
                            } else {
                                branch.keys.insert(0, key);
                            }

                            None
                        } else {
                            // Recursively insert into the child node
                            match &mut branch.children[child_index] {
                                TreeNode::Leaf(leaf) => {
                                    TreeNode::insert_into_leaf(leaf, key, value)
                                }
                                TreeNode::Branch(_) => {
                                    // For simplicity, we're not handling recursive insertion into branch nodes yet
                                    // This would require more complex logic for splitting nodes when they get too full
                                    unimplemented!(
                                        "Recursive insertion into branch nodes not implemented yet"
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

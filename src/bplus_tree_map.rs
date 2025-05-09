// Implementation of BPlusTreeMap

pub struct BPlusTreeMap<K, V> {
    root: Option<TreeNode<K, V>>,
    branching_factor: usize,
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

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
{
    // Default branching factor
    const DEFAULT_BRANCHING_FACTOR: usize = 4;

    pub fn new() -> Self {
        BPlusTreeMap {
            root: None,
            branching_factor: Self::DEFAULT_BRANCHING_FACTOR,
        }
    }

    pub fn with_branching_factor(branching_factor: usize) -> Self {
        if branching_factor < 2 {
            panic!("Branching factor must be at least 2");
        }

        BPlusTreeMap {
            root: None,
            branching_factor,
        }
    }

    // Helper method to create a tree with a branch node as root (for testing)
    #[cfg(test)]
    pub(crate) fn with_branch_root(
        key: K,
        left_leaf: LeafNode<K, V>,
        right_leaf: LeafNode<K, V>,
        branching_factor: Option<usize>,
    ) -> Self {
        let branch = BranchNode {
            keys: vec![key],
            children: vec![TreeNode::Leaf(left_leaf), TreeNode::Leaf(right_leaf)],
        };

        BPlusTreeMap {
            root: Some(TreeNode::Branch(branch)),
            branching_factor: branching_factor.unwrap_or(Self::DEFAULT_BRANCHING_FACTOR),
        }
    }

    // Check if a node needs to be split based on the branching factor
    fn should_split(branching_factor: usize, node_size: usize) -> bool {
        node_size >= branching_factor
    }

    // Helper method to split a leaf node
    fn split_leaf(leaf: &mut LeafNode<K, V>) -> (K, LeafNode<K, V>) {
        let mid = leaf.keys.len() / 2;
        let split_key = leaf.keys[mid].clone();

        // Create a new leaf with the second half of the keys/values
        let new_leaf = LeafNode {
            keys: leaf.keys.drain(mid..).collect(),
            values: leaf.values.drain(mid..).collect(),
        };

        (split_key, new_leaf)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Ord + ?Sized + PartialEq,
    {
        match &self.root {
            None => None,
            Some(tree_node) => match tree_node {
                TreeNode::Leaf(leaf) => {
                    // Use position to find the key
                    let pos = leaf.keys.iter().position(|k| k.borrow() == key);
                    pos.map(|idx| &leaf.values[idx])
                }
                TreeNode::Branch(branch) => {
                    // Find the appropriate child node to search in
                    // First check if the key is in the branch keys
                    let mut child_index = 0;
                    for (idx, k) in branch.keys.iter().enumerate() {
                        // Compare using PartialEq
                        if k.borrow() == key {
                            // If key exists, go to the right child
                            child_index = idx + 1;
                            break;
                        } else if k.borrow().cmp(&key) == std::cmp::Ordering::Greater {
                            // If key is less than current key, go to the left child
                            child_index = idx;
                            break;
                        } else {
                            // Continue searching
                            child_index = idx + 1;
                        }
                    }

                    if child_index < branch.children.len() {
                        match &branch.children[child_index] {
                            TreeNode::Leaf(leaf) => {
                                // Use position to find the key
                                let pos = leaf.keys.iter().position(|k| k.borrow() == key);
                                pos.map(|idx| &leaf.values[idx])
                            }
                            TreeNode::Branch(_) => {
                                // For simplicity, we're not handling recursive search into branch nodes yet
                                unimplemented!(
                                    "Recursive search into branch nodes not implemented yet"
                                )
                            }
                        }
                    } else {
                        None
                    }
                }
            },
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
                        // Check if the key already exists
                        match leaf.keys.binary_search(&key) {
                            Ok(pos) => {
                                // Key already exists, replace the value
                                let old_value = std::mem::replace(&mut leaf.values[pos], value);
                                return Some(old_value);
                            }
                            Err(pos) => {
                                // Insert the key and value at the found position
                                leaf.keys.insert(pos, key);
                                leaf.values.insert(pos, value);

                                // Check if the leaf node needs to be split
                                if Self::should_split(self.branching_factor, leaf.keys.len()) {
                                    // Split the leaf node
                                    let (split_key, new_leaf) = Self::split_leaf(leaf);

                                    // Create a new branch node as the root
                                    let branch = BranchNode {
                                        keys: vec![split_key],
                                        children: vec![
                                            TreeNode::Leaf(std::mem::replace(
                                                leaf,
                                                LeafNode {
                                                    keys: vec![],
                                                    values: vec![],
                                                },
                                            )),
                                            TreeNode::Leaf(new_leaf),
                                        ],
                                    };

                                    // Replace the root with the new branch node
                                    *tree_node = TreeNode::Branch(branch);
                                }

                                None
                            }
                        }
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

                            // Check if the branch node needs to be split
                            if Self::should_split(self.branching_factor, branch.keys.len()) {
                                // For simplicity, we're not handling branch node splitting yet
                                // This would require more complex logic
                                unimplemented!("Branch node splitting not implemented yet");
                            }

                            None
                        } else {
                            // Recursively insert into the child node
                            match &mut branch.children[child_index] {
                                TreeNode::Leaf(leaf) => {
                                    // Insert into the leaf node
                                    let result = match leaf.keys.binary_search(&key) {
                                        Ok(pos) => {
                                            // Key already exists, replace the value
                                            let old_value =
                                                std::mem::replace(&mut leaf.values[pos], value);
                                            Some(old_value)
                                        }
                                        Err(pos) => {
                                            // Insert the key and value at the found position
                                            leaf.keys.insert(pos, key.clone());
                                            leaf.values.insert(pos, value.clone());

                                            // Check if the leaf node needs to be split
                                            if Self::should_split(
                                                self.branching_factor,
                                                leaf.keys.len(),
                                            ) {
                                                // Split the leaf node
                                                let (split_key, new_leaf) = Self::split_leaf(leaf);

                                                // Insert the new leaf as a child of the branch node
                                                branch.children.insert(
                                                    child_index + 1,
                                                    TreeNode::Leaf(new_leaf),
                                                );

                                                // Insert the split key into the branch node
                                                branch.keys.insert(child_index, split_key);

                                                // Check if the branch node needs to be split
                                                if Self::should_split(
                                                    self.branching_factor,
                                                    branch.keys.len(),
                                                ) {
                                                    // For simplicity, we're not handling branch node splitting yet
                                                    unimplemented!(
                                                        "Branch node splitting not implemented yet"
                                                    );
                                                }
                                            }

                                            None
                                        }
                                    };

                                    result
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

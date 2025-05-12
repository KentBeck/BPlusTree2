use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::ops::Index;
use std::vec;

// Node types for the B+ tree
#[derive(Clone)]
pub struct LeafNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}

#[derive(Clone)]
pub struct BranchNode<K, V> {
    keys: Vec<K>,
    children: Vec<Node<K, V>>,
}

// Enum to represent different node types
#[derive(Clone)]
enum Node<K, V> {
    Leaf(LeafNode<K, V>),
    Branch(BranchNode<K, V>),
}

// Main B+ tree map structure
pub struct BPlusTreeMap<K, V> {
    root: Option<Node<K, V>>,
    branching_factor: usize,
    size: usize,
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Creates a new empty BPlusTreeMap with default branching factor of 4
    pub fn new() -> Self {
        Self::with_branching_factor(4)
    }

    /// Creates a new empty BPlusTreeMap with the specified branching factor
    pub fn with_branching_factor(branching_factor: usize) -> Self {
        if branching_factor < 2 {
            panic!("Branching factor must be at least 2");
        }
        BPlusTreeMap {
            root: None,
            branching_factor,
            size: 0,
        }
    }

    /// Creates a BPlusTreeMap with a branch node as root
    pub fn with_branch_root(
        branching_factor: usize,
        left_leaf: LeafNode<K, V>,
        right_leaf: LeafNode<K, V>,
        separator_key: Option<K>,
    ) -> Self {
        if branching_factor < 2 {
            panic!("Branching factor must be at least 2");
        }

        // Calculate the size
        let size = left_leaf.keys.len() + right_leaf.keys.len();

        // Use the first key of the right leaf as separator if not provided
        let separator = match separator_key {
            Some(key) => key,
            None => right_leaf.keys[0].clone(),
        };

        // Create the branch node
        let branch = BranchNode {
            keys: vec![separator],
            children: vec![Node::Leaf(left_leaf), Node::Leaf(right_leaf)],
        };

        // Create the tree map
        BPlusTreeMap {
            root: Some(Node::Branch(branch)),
            branching_factor,
            size,
        }
    }

    /// Returns the number of elements in the map
    pub fn len(&self) -> usize {
        self.size
    }

    /// Returns true if the map is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Inserts a key-value pair into the map
    /// Returns the old value if the key already existed
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let branching_factor = self.branching_factor;

        match self.root.take() {
            None => {
                // Create a new leaf node for the first insertion
                let leaf = LeafNode {
                    keys: vec![key],
                    values: vec![value],
                };
                self.root = Some(Node::Leaf(leaf));
                self.size = 1;
                None
            }
            Some(root) => {
                // Handle insertion into an existing tree
                let (new_root, old_value) =
                    Self::insert_recursive(root, key, value, branching_factor);
                self.root = Some(new_root);

                // Update size if this is a new key
                if old_value.is_none() {
                    self.size += 1;
                }

                old_value
            }
        }
    }

    /// Recursive helper for insertion
    fn insert_recursive(
        node: Node<K, V>,
        key: K,
        value: V,
        branching_factor: usize,
    ) -> (Node<K, V>, Option<V>) {
        match node {
            Node::Leaf(mut leaf) => {
                // Find the position to insert the key
                match leaf.keys.binary_search(&key) {
                    Ok(idx) => {
                        // Key already exists, replace the value
                        let old_value = std::mem::replace(&mut leaf.values[idx], value);
                        (Node::Leaf(leaf), Some(old_value))
                    }
                    Err(idx) => {
                        // Key doesn't exist, insert it
                        leaf.keys.insert(idx, key);
                        leaf.values.insert(idx, value);

                        // Check if the leaf needs to be split
                        if leaf.keys.len() > branching_factor {
                            // Split the leaf node
                            let split_idx = leaf.keys.len() / 2;
                            let split_key = leaf.keys[split_idx].clone();

                            // Create a new leaf with the right half of the keys/values
                            let right_keys = leaf.keys.drain(split_idx..).collect();
                            let right_values = leaf.values.drain(split_idx..).collect();
                            let right_leaf = LeafNode {
                                keys: right_keys,
                                values: right_values,
                            };

                            // Create a branch node with the split key and the two leaf nodes
                            let branch = BranchNode {
                                keys: vec![split_key],
                                children: vec![Node::Leaf(leaf), Node::Leaf(right_leaf)],
                            };

                            (Node::Branch(branch), None)
                        } else {
                            // No need to split
                            (Node::Leaf(leaf), None)
                        }
                    }
                }
            }
            Node::Branch(mut branch) => {
                // Find the child node to insert into
                let idx = match branch.keys.binary_search(&key) {
                    Ok(idx) => idx + 1, // If key exists, go to the right child
                    Err(idx) => idx,    // Otherwise, go to the appropriate child
                };

                // Check if the index is valid
                if idx >= branch.children.len() {
                    // This can happen if we're trying to insert a key that's greater than all existing keys
                    // In this case, we need to add a new child node
                    branch.children.push(Node::Leaf(Self::create_empty_leaf()));
                }

                // Take the child node out
                let child = std::mem::replace(
                    &mut branch.children[idx],
                    Node::Leaf(Self::create_empty_leaf()),
                );

                // Recursively insert into the child node
                let (new_child, old_value) =
                    Self::insert_recursive(child, key, value, branching_factor);

                // Put the child back
                branch.children[idx] = new_child;

                // Check if the child was split and we need to update the branch
                if let Node::Branch(new_branch) = &branch.children[idx] {
                    // If the child is now a branch node, it means it was split
                    // We need to extract the middle key and add the new child
                    if new_branch.keys.len() == 1 && new_branch.children.len() == 2 {
                        // Extract the middle key and the right child
                        let middle_key = new_branch.keys[0].clone();
                        let right_child = new_branch.children[1].clone();

                        // Replace the child with its left child
                        branch.children[idx] = new_branch.children[0].clone();

                        // Insert the middle key and the right child into the branch
                        branch.keys.insert(idx, middle_key);
                        branch.children.insert(idx + 1, right_child);
                    }
                }

                // Check if the branch needs to be split
                if branch.keys.len() > branching_factor {
                    // Split the branch node
                    let split_idx = branch.keys.len() / 2;
                    let split_key = branch.keys[split_idx].clone();

                    // Create a new branch with the right half of the keys/children
                    let right_keys = branch.keys.drain(split_idx + 1..).collect();
                    let right_children = branch.children.drain(split_idx + 1..).collect();
                    let right_branch = BranchNode {
                        keys: right_keys,
                        children: right_children,
                    };

                    // Remove the split key from the left branch
                    branch.keys.pop();

                    // Create a new branch node with the split key and the two branch nodes
                    let new_branch = BranchNode {
                        keys: vec![split_key],
                        children: vec![Node::Branch(branch), Node::Branch(right_branch)],
                    };

                    (Node::Branch(new_branch), old_value)
                } else {
                    // No need to split
                    (Node::Branch(branch), old_value)
                }
            }
        }
    }

    /// Gets a reference to the value associated with the key
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        // Use the find_leaf_for_key helper to locate the leaf node that might contain the key
        if let Some((leaf, _)) = self.find_leaf_for_key(key) {
            // Search for the key in the leaf node
            for (i, k) in leaf.keys.iter().enumerate() {
                if k.borrow() == key {
                    return Some(&leaf.values[i]);
                }
            }
        }

        None
    }

    /// Checks if a key exists in the map
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get(key).is_some()
    }

    /// Removes a key-value pair from the map
    /// Returns the value if the key was present in the map
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let branching_factor = self.branching_factor;

        match self.root.take() {
            None => None,
            Some(root) => {
                let (new_root, removed_value) = Self::remove_recursive(root, key, branching_factor);
                self.root = new_root;

                // Update size if a key was removed
                if removed_value.is_some() {
                    self.size -= 1;
                }

                removed_value
            }
        }
    }

    /// Recursive helper for remove
    fn remove_recursive<Q>(
        node: Node<K, V>,
        key: &Q,
        _branching_factor: usize,
    ) -> (Option<Node<K, V>>, Option<V>)
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match node {
            Node::Leaf(mut leaf) => {
                // Find the position of the key
                let mut found_idx = None;
                for (i, k) in leaf.keys.iter().enumerate() {
                    if k.borrow() == key {
                        found_idx = Some(i);
                        break;
                    }
                }

                // If the key is found, remove it
                if let Some(idx) = found_idx {
                    let _removed_key = leaf.keys.remove(idx);
                    let removed_value = leaf.values.remove(idx);

                    // If the leaf is now empty, return None for the node
                    if leaf.keys.is_empty() {
                        return (None, Some(removed_value));
                    }

                    // Otherwise, return the updated leaf
                    return (Some(Node::Leaf(leaf)), Some(removed_value));
                }

                // Key not found
                (Some(Node::Leaf(leaf)), None)
            }
            Node::Branch(mut branch) => {
                // Find the child node to remove from
                let mut idx = 0;
                for (i, k) in branch.keys.iter().enumerate() {
                    if key.cmp(k.borrow()) == Ordering::Less {
                        break;
                    }
                    idx = i + 1;
                }

                // Check if the index is valid
                if idx < branch.children.len() {
                    // Take the child node out
                    let child = std::mem::replace(
                        &mut branch.children[idx],
                        Node::Leaf(Self::create_empty_leaf()),
                    );

                    // Recursively remove from the child node
                    let (new_child, removed_value) =
                        Self::remove_recursive(child, key, _branching_factor);

                    // Update the branch node
                    if let Some(child) = new_child {
                        branch.children[idx] = child;
                    } else {
                        // Child node is now empty, remove it
                        branch.children.remove(idx);
                        if idx > 0 {
                            branch.keys.remove(idx - 1);
                        } else if !branch.keys.is_empty() {
                            branch.keys.remove(0);
                        }
                    }

                    // Return the updated branch and removed value
                    return (Some(Node::Branch(branch)), removed_value);
                }

                // Key not found
                (Some(Node::Branch(branch)), None)
            }
        }
    }
}

impl<K, V> FromIterator<(K, V)> for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        let mut map = BPlusTreeMap::new();
        for (k, v) in iter {
            map.insert(k, v);
        }
        map
    }
}

impl<K, V> Extend<(K, V)> for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, iter: I) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

/// An owning iterator over the entries of a `BPlusTreeMap`.
pub struct IntoIter<K, V> {
    // We'll use a simple vector-based approach for now
    // In a more advanced implementation, we might want to iterate through the tree structure directly
    entries: vec::IntoIter<(K, V)>,
}

impl<K, V> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.entries.next()
    }
}

/// A mutable iterator over the entries of a `BPlusTreeMap`.
pub struct IterMut<'a, K, V> {
    // Store key-value pairs as (K, &'a mut V) to avoid lifetime issues
    entries: Vec<(K, &'a mut V)>,
    position: usize,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V>
where
    K: Ord + Clone + Debug + 'a,
{
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.entries.len() {
            let position = self.position;
            self.position += 1;

            // Get a reference to the key and a mutable reference to the value
            let entry = &mut self.entries[position];

            // This is safe because we're returning each entry exactly once
            // and we know the indices are valid
            unsafe {
                let key_ptr = &entry.0 as *const K;
                let value_ptr = &mut *(entry.1 as *mut V);
                Some((&*key_ptr, value_ptr))
            }
        } else {
            None
        }
    }
}

impl<K, V> IntoIterator for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        // Collect all entries into a vector
        let mut entries = Vec::new();

        // Extract entries from the tree
        if let Some(root) = self.root {
            Self::collect_entries(root, &mut entries);
        }

        IntoIter {
            entries: entries.into_iter(),
        }
    }
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    // Helper method to collect all entries from the tree into a vector
    fn collect_entries(node: Node<K, V>, entries: &mut Vec<(K, V)>) {
        // Create a temporary BPlusTreeMap with the given node as root
        let temp_map = BPlusTreeMap {
            root: Some(node),
            branching_factor: 4, // Default value, doesn't matter for this operation
            size: 0,             // Doesn't matter for this operation
        };

        // Use the traverse method to collect all entries
        let collected = temp_map.traverse(|k, v| (k.clone(), v.clone()));

        // Add the collected entries to the provided vector
        entries.extend(collected);
    }
}

impl<K, V> Debug for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Start with the map opening
        write!(f, "{{")?;

        // Create a clone of the map and collect all entries
        let map_clone = self.clone();

        // Convert the clone into an iterator and collect all entries
        let all_entries: Vec<(K, V)> = map_clone.into_iter().collect();

        // Format each entry
        let mut first = true;
        for (k, v) in &all_entries {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{:?}: {:?}", k, v)?;
            first = false;
        }

        // Close the map
        write!(f, "}}")
    }
}

// Implement Clone for BPlusTreeMap to support Debug implementation
impl<K, V> Clone for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    fn clone(&self) -> Self {
        // Create a new map with the same branching factor
        let mut new_map = BPlusTreeMap::with_branching_factor(self.branching_factor);

        // Use the existing into_iter implementation to get all entries
        // We need to create a temporary copy to avoid consuming self
        let entries = self.into_iter_without_consuming();

        // Insert all entries into the new map
        for (k, v) in entries {
            new_map.insert(k, v);
        }

        new_map
    }
}

// Implement Default for BPlusTreeMap
impl<K, V> Default for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    fn default() -> Self {
        Self::new()
    }
}

// Implement Index for BPlusTreeMap
impl<K, V, Q> Index<&Q> for BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug + Borrow<Q>,
    V: Clone + Debug,
    Q: Ord + ?Sized,
{
    type Output = V;

    fn index(&self, key: &Q) -> &Self::Output {
        self.get(key).expect("no entry found for key")
    }
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Returns an iterator over the key-value pairs of the map.
    /// The iterator yields all key-value pairs in ascending order by key.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
        // For simplicity, we'll just collect all entries into a vector
        // and then create an iterator over the references.
        // In a real implementation, we would iterate through the tree directly.
        let mut entries = Vec::new();

        if let Some(root) = &self.root {
            Self::collect_entries_for_iter(root, &mut entries);
        }

        // Sort entries by key for consistent iteration order
        entries.sort_by(|a, b| a.0.cmp(b.0));

        // Return an iterator over the entries
        entries.into_iter()
    }

    /// Returns an iterator over the keys of the map.
    /// The iterator yields all keys in ascending order.
    pub fn keys(&self) -> impl Iterator<Item = &K> + '_ {
        // Use the iter method and map to extract just the keys
        self.iter().map(|(k, _)| k)
    }

    /// Returns an iterator over the values of the map.
    /// The iterator yields all values in ascending order by key.
    pub fn values(&self) -> impl Iterator<Item = &V> + '_ {
        // Use the iter method and map to extract just the values
        self.iter().map(|(_, v)| v)
    }

    /// Returns a mutable iterator over the key-value pairs of the map.
    /// The iterator yields all key-value pairs in ascending order by key.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        // Create a new mutable iterator
        let mut entries = Vec::new();

        // Collect all entries from the tree
        if let Some(root) = &mut self.root {
            Self::collect_entries_for_iter_mut(root, &mut entries);
        }

        // Sort entries by key for consistent iteration order
        entries.sort_by(|a, b| a.0.cmp(&b.0));

        // Return the iterator
        IterMut {
            entries,
            position: 0,
        }
    }

    /// Helper method to collect entries for the iter_mut method
    fn collect_entries_for_iter_mut<'a>(
        node: &'a mut Node<K, V>,
        entries: &mut Vec<(K, &'a mut V)>,
    ) {
        match node {
            Node::Leaf(leaf) => {
                // We need to handle this differently to avoid multiple mutable borrows
                let keys_len = leaf.keys.len();

                // Clone all keys first
                let keys: Vec<K> = leaf.keys.iter().cloned().collect();

                // Then get mutable references to values one by one
                for i in 0..keys_len {
                    // This is safe because we're accessing each index exactly once
                    // and we know the indices are valid
                    unsafe {
                        let value_ptr = leaf.values.as_mut_ptr().add(i);
                        entries.push((keys[i].clone(), &mut *value_ptr));
                    }
                }
            }
            Node::Branch(branch) => {
                // Recursively collect entries from all children
                for child in &mut branch.children {
                    Self::collect_entries_for_iter_mut(child, entries);
                }
            }
        }
    }

    /// Helper method to collect entries for the iter method
    fn collect_entries_for_iter<'a>(node: &'a Node<K, V>, entries: &mut Vec<(&'a K, &'a V)>) {
        match node {
            Node::Leaf(leaf) => {
                // Add all entries from this leaf node
                for i in 0..leaf.keys.len() {
                    entries.push((&leaf.keys[i], &leaf.values[i]));
                }
            }
            Node::Branch(branch) => {
                // Recursively collect entries from all children
                for child in &branch.children {
                    Self::collect_entries_for_iter(child, entries);
                }
            }
        }
    }
}

// Tree traversal and helper methods
impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Creates an empty leaf node
    fn create_empty_leaf() -> LeafNode<K, V> {
        LeafNode {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Traverses the tree and applies the visitor function to each key-value pair
    /// The visitor function can transform the key-value pairs and collect them
    fn traverse<F, R>(&self, visitor: F) -> Vec<R>
    where
        F: Fn(&K, &V) -> R,
    {
        let mut results = Vec::new();

        if let Some(root) = &self.root {
            Self::traverse_node(root, &mut results, &visitor);
        }

        results
    }

    /// Recursively traverses a node and applies the visitor function to each key-value pair
    fn traverse_node<F, R>(node: &Node<K, V>, results: &mut Vec<R>, visitor: &F)
    where
        F: Fn(&K, &V) -> R,
    {
        match node {
            Node::Leaf(leaf) => {
                // Process all entries in this leaf node
                for i in 0..leaf.keys.len() {
                    results.push(visitor(&leaf.keys[i], &leaf.values[i]));
                }
            }
            Node::Branch(branch) => {
                // Recursively process all children
                for child in &branch.children {
                    Self::traverse_node(child, results, visitor);
                }
            }
        }
    }

    /// Finds a leaf node that might contain the given key
    /// Returns the leaf node and its index in the tree
    fn find_leaf_for_key<Q>(&self, key: &Q) -> Option<(&LeafNode<K, V>, usize)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match &self.root {
            None => None,
            Some(Node::Leaf(leaf)) => Some((leaf, 0)),
            Some(Node::Branch(branch)) => {
                // Find the child node to search in
                let mut idx = 0;
                for (i, k) in branch.keys.iter().enumerate() {
                    if key.cmp(k.borrow()) == Ordering::Less {
                        break;
                    }
                    idx = i + 1;
                }

                // Check if the index is valid
                if idx < branch.children.len() {
                    match &branch.children[idx] {
                        Node::Leaf(leaf) => Some((leaf, idx)),
                        Node::Branch(_) => {
                            // Recursively search deeper in the tree
                            Self::find_leaf_for_key_recursive(&branch.children[idx], key)
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Recursively finds a leaf node that might contain the given key
    fn find_leaf_for_key_recursive<'a, 'b, Q>(
        node: &'a Node<K, V>,
        key: &'b Q,
    ) -> Option<(&'a LeafNode<K, V>, usize)>
    where
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        match node {
            Node::Leaf(leaf) => Some((leaf, 0)),
            Node::Branch(branch) => {
                // Find the child node to search in
                let mut idx = 0;
                for (i, k) in branch.keys.iter().enumerate() {
                    if key.cmp(k.borrow()) == Ordering::Less {
                        break;
                    }
                    idx = i + 1;
                }

                // Check if the index is valid
                if idx < branch.children.len() {
                    match &branch.children[idx] {
                        Node::Leaf(leaf) => Some((leaf, idx)),
                        Node::Branch(_) => {
                            // Recursively search deeper in the tree
                            Self::find_leaf_for_key_recursive(&branch.children[idx], key)
                        }
                    }
                } else {
                    None
                }
            }
        }
    }

    // A non-consuming version of into_iter that collects entries without consuming self
    fn into_iter_without_consuming(&self) -> Vec<(K, V)> {
        self.traverse(|k, v| (k.clone(), v.clone()))
    }
}

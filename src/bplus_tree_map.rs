use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::ops::Index;
use std::vec;

use crate::node_balancer::{BalanceResult, InsertionBalancer, NodeBalancer, RemovalBalancer};

// Node types for the B+ tree
#[derive(Clone)]
pub struct LeafNode<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}

#[derive(Clone)]
pub struct BranchNode<K, V> {
    pub keys: Vec<K>,
    pub children: Vec<Node<K, V>>,
}

// Enum to represent different node types
#[derive(Clone)]
pub enum Node<K, V> {
    Leaf(LeafNode<K, V>),
    Branch(BranchNode<K, V>),
}

/// The type of node stored at the root of the tree. This is useful in tests
/// and for debugging the tree structure.
#[derive(Debug, PartialEq, Eq)]
pub enum RootKind {
    /// The tree is empty.
    Empty,
    /// The root is a leaf node.
    Leaf,
    /// The root is a branch node.
    Branch,
}

// Main B+ tree map structure
pub struct BPlusTreeMap<K, V> {
    root: Option<Node<K, V>>,
    branching_factor: usize,
    size: usize,
    insertion_balancer: InsertionBalancer,
    removal_balancer: RemovalBalancer,
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
            insertion_balancer: InsertionBalancer::new(branching_factor),
            removal_balancer: RemovalBalancer::new(branching_factor),
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
            insertion_balancer: InsertionBalancer::new(branching_factor),
            removal_balancer: RemovalBalancer::new(branching_factor),
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

    /// Returns the type of node stored at the root of the tree. This is mainly
    /// for testing and debugging purposes.
    pub fn root_kind(&self) -> RootKind {
        match &self.root {
            None => RootKind::Empty,
            Some(Node::Leaf(_)) => RootKind::Leaf,
            Some(Node::Branch(_)) => RootKind::Branch,
        }
    }

    /// Inserts a key-value pair into the map
    /// Returns the old value if the key already existed
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
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
                    Self::insert_recursive(root, key, value, &self.insertion_balancer);
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
        balancer: &InsertionBalancer,
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

                        // Use the balancer to check if the node needs to be split
                        match balancer.balance_node(Node::Leaf(leaf)) {
                            BalanceResult::Split {
                                left,
                                right,
                                separator,
                            } => {
                                // Create a branch node with the separator key and the two nodes
                                let branch = BranchNode {
                                    keys: vec![separator],
                                    children: vec![left, right],
                                };

                                (Node::Branch(branch), None)
                            }
                            BalanceResult::NoChange(node) => (node, None),
                            _ => panic!("Unexpected balance result for insertion"),
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
                let (new_child, old_value) = Self::insert_recursive(child, key, value, balancer);

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

                // Use the balancer to check if the branch node needs to be split
                match balancer.balance_node(Node::Branch(branch)) {
                    BalanceResult::Split {
                        left,
                        right,
                        separator,
                    } => {
                        // Create a new branch node with the separator key and the two branch nodes
                        let new_branch = BranchNode {
                            keys: vec![separator],
                            children: vec![left, right],
                        };

                        (Node::Branch(new_branch), old_value)
                    }
                    BalanceResult::NoChange(node) => (node, old_value),
                    _ => panic!("Unexpected balance result for insertion"),
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
        match self.root.take() {
            None => None,
            Some(root) => {
                let (new_root, removed_value) =
                    Self::remove_recursive(root, key, &self.removal_balancer);
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
        balancer: &RemovalBalancer,
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
                    let (new_child, removed_value) = Self::remove_recursive(child, key, balancer);

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

                    // Check if we need to balance adjacent nodes
                    if idx > 0 && idx < branch.children.len() {
                        let left_child = std::mem::replace(
                            &mut branch.children[idx - 1],
                            Node::Leaf(Self::create_empty_leaf()),
                        );
                        let right_child = std::mem::replace(
                            &mut branch.children[idx],
                            Node::Leaf(Self::create_empty_leaf()),
                        );
                        let separator = branch.keys[idx - 1].clone();

                        // Clone the right child for potential use later
                        let right_child_clone = right_child.clone();

                        // Balance the nodes
                        match balancer.balance_nodes(left_child, right_child, separator) {
                            BalanceResult::Merged(merged_node) => {
                                // Replace the left child with the merged node
                                branch.children[idx - 1] = merged_node;
                                // Remove the right child and the separator
                                branch.children.remove(idx);
                                branch.keys.remove(idx - 1);
                            }
                            BalanceResult::Rebalanced {
                                left,
                                right,
                                separator,
                            } => {
                                // Update the children and separator
                                branch.children[idx - 1] = left;
                                branch.children[idx] = right;
                                branch.keys[idx - 1] = separator;
                            }
                            BalanceResult::NoChange(node) => {
                                // Put the left child back
                                branch.children[idx - 1] = node;
                                // We need to put the right child back too
                                branch.children[idx] = right_child_clone;
                            }
                            _ => panic!("Unexpected balance result for removal"),
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

/// A common base iterator for all BPlusTreeMap iterators.
/// This provides a unified way to iterate over the tree's entries.
pub struct TreeIterator<T> {
    /// The entries to iterate over
    entries: Vec<T>,
    /// The current position in the entries
    position: usize,
}

impl<T> TreeIterator<T> {
    /// Creates a new TreeIterator with the given entries
    pub fn new(entries: Vec<T>) -> Self {
        Self {
            entries,
            position: 0,
        }
    }
}

impl<T> Iterator for TreeIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.entries.len() {
            let item = self.entries[self.position].clone();
            self.position += 1;
            Some(item)
        } else {
            None
        }
    }
}

/// An owning iterator over the entries of a `BPlusTreeMap`.
pub struct IntoIter<K, V> {
    inner: TreeIterator<(K, V)>,
}

impl<K, V> Iterator for IntoIter<K, V>
where
    K: Clone,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A reference iterator over the entries of a `BPlusTreeMap`.
pub struct Iter<'a, K, V> {
    inner: TreeIterator<(&'a K, &'a V)>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: 'a,
    V: 'a,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
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

/// An iterator over the keys of a `BPlusTreeMap`.
pub struct Keys<'a, K> {
    inner: TreeIterator<&'a K>,
}

impl<'a, K> Iterator for Keys<'a, K>
where
    K: 'a + Clone,
{
    type Item = &'a K;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// An iterator over the values of a `BPlusTreeMap`.
pub struct Values<'a, V> {
    inner: TreeIterator<&'a V>,
}

impl<'a, V> Iterator for Values<'a, V>
where
    V: 'a + Clone,
{
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

/// A mutable iterator over the values of a `BPlusTreeMap`.
pub struct ValuesMut<'a, V> {
    // We can't use TreeIterator for mutable references because they don't implement Clone
    entries: Vec<&'a mut V>,
    position: usize,
}

impl<'a, V> ValuesMut<'a, V> {
    /// Creates a new ValuesMut with the given entries
    pub fn new(entries: Vec<&'a mut V>) -> Self {
        Self {
            entries,
            position: 0,
        }
    }
}

impl<'a, V> Iterator for ValuesMut<'a, V>
where
    V: 'a,
{
    type Item = &'a mut V;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.entries.len() {
            let position = self.position;
            self.position += 1;

            // This is safe because we're returning each entry exactly once
            // and we know the indices are valid
            unsafe {
                let value_ptr = self.entries.as_mut_ptr().add(position);
                Some(&mut *value_ptr)
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
            inner: TreeIterator::new(entries),
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
            insertion_balancer: InsertionBalancer::new(4), // Default value, doesn't matter for this operation
            removal_balancer: RemovalBalancer::new(4), // Default value, doesn't matter for this operation
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
    /// Gets the given key's corresponding entry in the map for in-place manipulation.
    /// This method provides a more efficient way to manipulate entries in the map
    /// without having to do multiple lookups.
    pub fn entry(&mut self, key: K) -> Entry<'_, K, V> {
        if self.contains_key(&key) {
            Entry::Occupied(OccupiedEntry { map: self, key })
        } else {
            Entry::Vacant(VacantEntry { map: self, key })
        }
    }

    /// Returns an iterator over the key-value pairs of the map.
    /// The iterator yields all key-value pairs in ascending order by key.
    pub fn iter(&self) -> Iter<'_, K, V> {
        // Use the visitor pattern to collect references
        let entries = self.collect_refs();
        Iter {
            inner: TreeIterator::new(entries),
        }
    }

    /// Returns an iterator over the keys of the map.
    /// The iterator yields all keys in ascending order.
    pub fn keys(&self) -> Keys<'_, K> {
        // Collect all keys from the tree
        let keys = self.collect_refs().into_iter().map(|(k, _)| k).collect();
        Keys {
            inner: TreeIterator::new(keys),
        }
    }

    /// Returns an iterator over the values of the map.
    /// The iterator yields all values in ascending order by key.
    pub fn values(&self) -> Values<'_, V> {
        // Collect all values from the tree
        let values = self.collect_refs().into_iter().map(|(_, v)| v).collect();
        Values {
            inner: TreeIterator::new(values),
        }
    }

    /// Returns a mutable iterator over the values of the map.
    /// The iterator yields all values in ascending order by key.
    pub fn values_mut(&mut self) -> ValuesMut<'_, V> {
        use crate::safe_traversal::SafeValuesMutVisitor;

        // Use the safe visitor to collect mutable values
        let mut visitor = SafeValuesMutVisitor::new();
        self.accept_visitor_mut(&mut visitor);
        let values = <SafeValuesMutVisitor<'_, V> as NodeVisitorMut<K, V>>::result(visitor);
        ValuesMut::new(values)
    }

    /// Returns a mutable iterator over the key-value pairs of the map.
    /// The iterator yields all key-value pairs in ascending order by key.
    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        // Use the visitor pattern to collect mutable references
        let entries = self.collect_mut_refs();

        // Return the iterator
        IterMut {
            entries,
            position: 0,
        }
    }
}

/// A trait for visiting nodes in a B+ tree
pub trait NodeVisitor<K, V> {
    /// The type of result produced by the visitor
    type Result;

    /// Visit a leaf node
    fn visit_leaf(&mut self, leaf: &LeafNode<K, V>);

    /// Visit a branch node
    fn visit_branch(&mut self, branch: &BranchNode<K, V>);

    /// Get the accumulated result
    fn result(self) -> Self::Result;
}

/// A trait for visiting nodes in a B+ tree with mutable access
pub trait NodeVisitorMut<K, V> {
    /// The type of result produced by the visitor
    type Result;

    /// Visit a leaf node with mutable access
    fn visit_leaf(&mut self, leaf: &mut LeafNode<K, V>);

    /// Visit a branch node with mutable access
    fn visit_branch(&mut self, branch: &mut BranchNode<K, V>);

    /// Get the accumulated result
    fn result(self) -> Self::Result;
}

/// A visitor that collects key-value pairs with a transformation function
pub struct CollectingVisitor<K, V, F, R>
where
    F: Fn(&K, &V) -> R,
{
    visitor_fn: F,
    results: Vec<R>,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V, F, R> CollectingVisitor<K, V, F, R>
where
    F: Fn(&K, &V) -> R,
{
    /// Create a new collecting visitor with the given transformation function
    pub fn new(visitor_fn: F) -> Self {
        Self {
            visitor_fn,
            results: Vec::new(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<K, V, F, R> NodeVisitor<K, V> for CollectingVisitor<K, V, F, R>
where
    F: Fn(&K, &V) -> R,
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    type Result = Vec<R>;

    fn visit_leaf(&mut self, leaf: &LeafNode<K, V>) {
        // Process all entries in this leaf node
        for i in 0..leaf.keys.len() {
            self.results
                .push((self.visitor_fn)(&leaf.keys[i], &leaf.values[i]));
        }
    }

    fn visit_branch(&mut self, _branch: &BranchNode<K, V>) {
        // No direct processing for branch nodes in this visitor
    }

    fn result(self) -> Self::Result {
        self.results
    }
}

/// An entry in a `BPlusTreeMap`. It is part of the map API and can be used to
/// manipulate the map without having to do multiple lookups.
pub enum Entry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, K, V>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, K, V>),
}

/// A view into an occupied entry in a `BPlusTreeMap`.
/// It is part of the Entry API.
pub struct OccupiedEntry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// The map this entry belongs to
    map: &'a mut BPlusTreeMap<K, V>,
    /// The key for this entry
    key: K,
}

/// A view into a vacant entry in a `BPlusTreeMap`.
/// It is part of the Entry API.
pub struct VacantEntry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// The map this entry belongs to
    map: &'a mut BPlusTreeMap<K, V>,
    /// The key for this entry
    key: K,
}

impl<'a, K, V> Entry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Ensures a value is in the entry by inserting the default if empty, and returns
    /// a mutable reference to the value in the entry.
    pub fn or_insert(self, default: V) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if empty,
    /// and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F: FnOnce() -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => entry.insert(default()),
        }
    }

    /// Ensures a value is in the entry by inserting, if empty, the result of the default function.
    /// This method allows for generating key-derived values for insertion.
    pub fn or_insert_with_key<F: FnOnce(&K) -> V>(self, default: F) -> &'a mut V {
        match self {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let value = default(entry.key());
                entry.insert(value)
            }
        }
    }

    /// Returns a reference to this entry's key.
    pub fn key(&self) -> &K {
        match self {
            Entry::Occupied(entry) => entry.key(),
            Entry::Vacant(entry) => entry.key(),
        }
    }

    /// Provides in-place mutable access to an occupied entry before any
    /// potential inserts into the map.
    pub fn and_modify<F>(self, f: F) -> Self
    where
        F: FnOnce(&mut V),
    {
        match self {
            Entry::Occupied(mut entry) => {
                f(entry.get_mut());
                Entry::Occupied(entry)
            }
            Entry::Vacant(entry) => Entry::Vacant(entry),
        }
    }
}

impl<'a, K, V> OccupiedEntry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Gets a reference to the key in the entry.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Gets a reference to the value in the entry.
    pub fn get(&self) -> &V {
        // We know the key exists, so unwrap is safe
        self.map.get(&self.key).unwrap()
    }

    /// Gets a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut V {
        use crate::safe_traversal::FindValueMutVisitor;

        // Use the safe visitor to find the value
        let mut visitor = FindValueMutVisitor::new(&self.key);
        self.map.accept_visitor_mut(&mut visitor);
        match <FindValueMutVisitor<'_, V, K> as NodeVisitorMut<K, V>>::result(visitor) {
            Some(value) => value,
            None => panic!("Key not found in map"),
        }
    }

    /// Converts the entry into a mutable reference to its value.
    pub fn into_mut(self) -> &'a mut V {
        // We need to use the collect_mut_refs method which already handles lifetimes correctly
        let entries = self.map.collect_mut_refs();
        for (k, v) in entries {
            if k == self.key {
                return v;
            }
        }
        panic!("Key not found in map");
    }

    /// Sets the value of the entry with the key already in the map.
    pub fn insert(&mut self, value: V) -> V {
        // We know the key exists, so unwrap is safe
        std::mem::replace(self.get_mut(), value)
    }

    /// Takes the value out of the entry, and returns it.
    pub fn remove(self) -> V {
        // We know the key exists, so unwrap is safe
        self.map.remove(&self.key).unwrap()
    }
}

impl<'a, K, V> VacantEntry<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: Clone + Debug,
{
    /// Gets a reference to the key that would be used when inserting a value
    /// through the `VacantEntry`.
    pub fn key(&self) -> &K {
        &self.key
    }

    /// Sets the value of the entry with the `VacantEntry`'s key,
    /// and returns a mutable reference to it.
    pub fn insert(self, value: V) -> &'a mut V {
        self.map.insert(self.key.clone(), value);

        // We need to use the collect_mut_refs method which already handles lifetimes correctly
        let entries = self.map.collect_mut_refs();
        for (k, v) in entries {
            if k == self.key {
                return v;
            }
        }
        panic!("Key not found in map after insertion");
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

    /// Collects references to key-value pairs from the tree
    pub fn collect_refs<'a>(&'a self) -> Vec<(&'a K, &'a V)> {
        let mut entries = Vec::new();
        if let Some(root) = &self.root {
            Self::collect_refs_from_node(root, &mut entries);
        }
        entries.sort_by(|a, b| a.0.cmp(b.0));
        entries
    }

    /// Recursively collects references to key-value pairs from a node
    fn collect_refs_from_node<'a>(node: &'a Node<K, V>, entries: &mut Vec<(&'a K, &'a V)>) {
        match node {
            Node::Leaf(leaf) => {
                // Add all entries from this leaf node
                for i in 0..leaf.keys.len() {
                    entries.push((&leaf.keys[i], &leaf.values[i]));
                }
            }
            Node::Branch(branch) => {
                // Recursively process all children
                for child in &branch.children {
                    Self::collect_refs_from_node(child, entries);
                }
            }
        }
    }

    /// Collects mutable references to values with cloned keys from the tree
    pub fn collect_mut_refs<'a>(&'a mut self) -> Vec<(K, &'a mut V)> {
        use crate::safe_traversal::SafeMutableVisitor;

        let mut visitor = SafeMutableVisitor::new();
        self.accept_visitor_mut(&mut visitor);
        let mut entries = visitor.result();
        entries.sort_by(|a, b| a.0.cmp(&b.0));
        entries
    }

    /// Accepts a visitor and traverses the tree
    pub fn accept<Visitor: NodeVisitor<K, V>>(&self, visitor: &mut Visitor) {
        if let Some(root) = &self.root {
            Self::accept_node(root, visitor);
        }
    }

    /// Accepts a visitor and traverses the tree with mutable access
    pub fn accept_mut<'a, Visitor: NodeVisitor<K, V>>(&'a mut self, visitor: &mut Visitor) {
        if let Some(root) = &mut self.root {
            Self::accept_node_mut(root, visitor);
        }
    }

    /// Accepts a visitor with mutable access to nodes and traverses the tree
    pub fn accept_visitor_mut<'a, Visitor: NodeVisitorMut<K, V>>(
        &'a mut self,
        visitor: &mut Visitor,
    ) {
        if let Some(root) = &mut self.root {
            Self::accept_node_visitor_mut(root, visitor);
        }
    }

    /// Recursively traverses a node and applies the visitor
    fn accept_node<Visitor: NodeVisitor<K, V>>(node: &Node<K, V>, visitor: &mut Visitor) {
        match node {
            Node::Leaf(leaf) => {
                visitor.visit_leaf(leaf);
            }
            Node::Branch(branch) => {
                visitor.visit_branch(branch);
                // Recursively process all children
                for child in &branch.children {
                    Self::accept_node(child, visitor);
                }
            }
        }
    }

    /// Recursively traverses a node and applies the visitor with mutable access
    fn accept_node_mut<'a, Visitor: NodeVisitor<K, V>>(
        node: &'a mut Node<K, V>,
        visitor: &mut Visitor,
    ) {
        match node {
            Node::Leaf(leaf) => {
                visitor.visit_leaf(leaf);
            }
            Node::Branch(branch) => {
                visitor.visit_branch(branch);
                // Recursively process all children
                for child in &mut branch.children {
                    Self::accept_node_mut(child, visitor);
                }
            }
        }
    }

    /// Recursively traverses a node and applies the visitor with mutable access to nodes
    fn accept_node_visitor_mut<'a, Visitor: NodeVisitorMut<K, V>>(
        node: &'a mut Node<K, V>,
        visitor: &mut Visitor,
    ) {
        match node {
            Node::Leaf(leaf) => {
                visitor.visit_leaf(leaf);
            }
            Node::Branch(branch) => {
                visitor.visit_branch(branch);
                // Recursively process all children
                for child in &mut branch.children {
                    Self::accept_node_visitor_mut(child, visitor);
                }
            }
        }
    }

    /// Traverses the tree and applies the visitor function to each key-value pair
    /// The visitor function can transform the key-value pairs and collect them
    fn traverse<F, R>(&self, visitor_fn: F) -> Vec<R>
    where
        F: Fn(&K, &V) -> R,
    {
        let mut visitor = CollectingVisitor::new(visitor_fn);
        self.accept(&mut visitor);
        visitor.result()
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

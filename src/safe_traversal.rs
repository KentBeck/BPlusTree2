use std::fmt::Debug;
use std::marker::PhantomData;

use crate::bplus_tree_map::{BranchNode, LeafNode, NodeVisitorMut};

/// Returns a raw mutable pointer to the element at `index` from `slice`.
///
/// # Safety
/// The caller must ensure that `index` is within bounds. The returned pointer
/// must not outlive the borrow of `slice` and must not be used to create
/// multiple mutable references simultaneously.
unsafe fn ptr_at_mut<T>(slice: &mut [T], index: usize) -> *mut T {
    unsafe { slice.as_mut_ptr().add(index) }
}

/// A visitor that safely collects mutable references to values in a B+ tree
pub struct SafeMutableVisitor<'a, K, V> {
    /// The collected entries (key clones and mutable references to values)
    entries: Vec<(K, &'a mut V)>,
    /// Phantom data to track lifetime
    _marker: PhantomData<&'a mut V>,
}

impl<'a, K, V> SafeMutableVisitor<'a, K, V>
where
    K: Clone,
{
    /// Creates a new SafeMutableVisitor
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<'a, K, V> NodeVisitorMut<K, V> for SafeMutableVisitor<'a, K, V>
where
    K: Ord + Clone + Debug,
    V: 'a,
{
    type Result = Vec<(K, &'a mut V)>;

    fn visit_leaf(&mut self, leaf: &mut LeafNode<K, V>) {
        // Safely collect mutable references to values with cloned keys
        for i in 0..leaf.keys.len() {
            let key = leaf.keys[i].clone();
            unsafe {
                let value_ptr = ptr_at_mut(&mut leaf.values, i);
                self.entries.push((key, &mut *value_ptr));
            }
        }
    }

    fn visit_branch(&mut self, _branch: &mut BranchNode<K, V>) {
        // No values to collect in branch nodes
    }

    fn result(self) -> Self::Result {
        self.entries
    }
}

/// A visitor that safely collects mutable references to values in a B+ tree
pub struct SafeValuesMutVisitor<'a, V> {
    /// The collected mutable references to values
    values: Vec<&'a mut V>,
    /// Phantom data to track lifetime
    _marker: PhantomData<&'a mut V>,
}

impl<'a, V> SafeValuesMutVisitor<'a, V> {
    /// Creates a new SafeValuesMutVisitor
    pub fn new() -> Self {
        Self {
            values: Vec::new(),
            _marker: PhantomData,
        }
    }
}

impl<'a, K, V> NodeVisitorMut<K, V> for SafeValuesMutVisitor<'a, V>
where
    K: Ord + Clone + Debug,
    V: 'a,
{
    type Result = Vec<&'a mut V>;

    fn visit_leaf(&mut self, leaf: &mut LeafNode<K, V>) {
        // Safely collect mutable references to values
        // We need to use raw pointers to avoid multiple mutable borrows
        for i in 0..leaf.values.len() {
            unsafe {
                let value_ptr = ptr_at_mut(&mut leaf.values, i);
                self.values.push(&mut *value_ptr);
            }
        }
    }

    fn visit_branch(&mut self, _branch: &mut BranchNode<K, V>) {
        // No values to collect in branch nodes
    }

    fn result(self) -> Self::Result {
        self.values
    }
}

/// A visitor that safely finds a mutable reference to a specific value in a B+ tree
pub struct FindValueMutVisitor<'a, V, Q: ?Sized> {
    /// The key to find
    key: &'a Q,
    /// The found value, if any
    value: Option<&'a mut V>,
    /// Phantom data to track lifetime
    _marker: PhantomData<&'a mut V>,
}

impl<'a, V, Q: ?Sized> FindValueMutVisitor<'a, V, Q>
where
    Q: Ord,
    V: 'a,
{
    /// Creates a new FindValueMutVisitor
    pub fn new(key: &'a Q) -> Self {
        Self {
            key,
            value: None,
            _marker: PhantomData,
        }
    }
}

impl<'a, K, V, Q: ?Sized> NodeVisitorMut<K, V> for FindValueMutVisitor<'a, V, Q>
where
    K: Ord + Clone + Debug + std::borrow::Borrow<Q>,
    Q: Ord,
    V: 'a,
{
    type Result = Option<&'a mut V>;

    fn visit_leaf(&mut self, leaf: &mut LeafNode<K, V>) {
        // Find the key in the leaf node
        for i in 0..leaf.keys.len() {
            if leaf.keys[i].borrow() == self.key {
                unsafe {
                    let value_ptr = ptr_at_mut(&mut leaf.values, i);
                    self.value = Some(&mut *value_ptr);
                }
                break;
            }
        }
    }

    fn visit_branch(&mut self, _branch: &mut BranchNode<K, V>) {
        // No values to find in branch nodes
    }

    fn result(self) -> Self::Result {
        self.value
    }
}

// Implementation of BPlusTreeMap

pub struct BPlusTreeMap<K, V> {
    root: Option<Node<K, V>>,
}

pub(crate) struct Node<K, V> {
    pub keys: Vec<K>,
    pub values: Vec<V>,
}

impl<K, V> BPlusTreeMap<K, V>
where
    K: Ord,
{
    pub fn new() -> Self {
        BPlusTreeMap { root: None }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        match &mut self.root {
            None => {
                let node = Node {
                    keys: vec![key],
                    values: vec![value],
                };
                self.root = Some(node);
                None
            }
            Some(node) => {
                // Find the position to insert
                let pos = match node.keys.binary_search(&key) {
                    Ok(pos) => {
                        // Key already exists, replace the value
                        let old_value = std::mem::replace(&mut node.values[pos], value);
                        return Some(old_value);
                    }
                    Err(pos) => pos,
                };

                // Insert the key and value at the found position
                node.keys.insert(pos, key);
                node.values.insert(pos, value);
                None
            }
        }
    }
}

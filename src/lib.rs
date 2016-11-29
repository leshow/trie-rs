use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::Debug;

/// A Trie is a tree where each vertex represents a single word or a prefix.
/// in such a way that searching for a string prefix is O(mn) with space O(mn)
/// This Trie implementation tries (no pun intended) to be as generic as possible
/// in what kinds of keys it can accept. Therefore the insert methods take anything
/// that can be iterated over, and the Trie will create a new Trie at each key
/// indexed by that iterable.
///
/// Existing Trie implementations on cargo were abandoned and would not compile under
/// the latest stable rustc. So I wrote this one from scratch, influenced slightly from
/// the other implementations.
///
/// here's an example of an insert, using either a slice of chars or collecting
/// a string into a slice:
/// ```
/// let mut trie: Trie<char, u8> = Trie::new();
/// trie.insert(&['a', 'b'], 20);
/// trie.insert(&"first".chars().collect::<Vec<char>>(), 40);
/// ```
///
/// and checking if the prefix exists:
/// ```
/// assert!(trie.contains_prefix(&['f', 'i']));
/// ```
///
/// Another common thing to do with a Trie is build up a list of node which match a given
/// prefix. This is useful for things like autocompletion.
///
pub trait Key: Eq + Hash + Clone + Debug {}
pub trait Value: Debug {}

#[derive(Eq, PartialEq, Clone)]
pub struct Trie<K, V>
    where K: Key,
          V: Value
{
    pub value: Option<V>,
    pub children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Debug for Trie<K, V>
    where V: Value,
          K: Key
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt,
               "Trie {{ children: {:?}, value: {:?} }} \n",
               self.children,
               self.value)
    }
}

impl<'key, K, V> Trie<K, V>
    where K: 'key + Key,
          V: Value
{
    pub fn new() -> Trie<K, V> {
        Trie {
            children: HashMap::new(),
            value: None,
        }
    }
    pub fn with_value(self, v: Option<V>) -> Trie<K, V> {
        Trie { value: v, ..self }
    }
    /// inserts something iteratable and a value into the Trie
    /// if that sequence already exists the value will be replaced with the new one
    pub fn insert<I>(&mut self, iter: I, value: V)
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter.into_iter() {
            let tmp = node;
            node = tmp.children.entry(c.clone()).or_insert_with(Trie::new);
        }
        node.value = Some(value);
    }
    /// Inserts using a fold operation over the iterator passed to it
    /// may possibly be faster than regular insert.
    pub fn insert_fold<I>(&mut self, iter: I, value: V)
        where I: IntoIterator<Item = &'key K>
    {
        let node = iter.into_iter().fold(self, |cur_node, c| {
            match cur_node.children.entry(c.clone()) {
                Vacant(v) => v.insert(Trie::new()),
                Occupied(v) => v.into_mut(),
            }
        });
        node.value = Some(value);
    }
    /// inserts using an unsafe pointer. Specifically, a raw ptr is used to move
    /// through the tree, this may be faster than the other insert functions.
    pub fn insert_raw<I>(&mut self, iter: I, value: V)
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        let mut raw_ptr: *mut Trie<K, V>; // = node as *mut Trie<K, V>;
        for c in iter.into_iter() {
            raw_ptr = node.children.entry(c.clone()).or_insert_with(Trie::new);
            unsafe {
                node = &mut *raw_ptr;
            }
        }
        node.value = Some(value);
    }
    pub fn contains_prefix<I>(&self, iter: I) -> bool
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter.into_iter() {
            if !node.children.contains_key(c) {
                return false;
            }
            std::mem::replace(&mut node, node.children.get(&c).unwrap());
        }
        true
    }
    /// Returns true if value is None and chilren is also empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }
    /// returns the node at a given position as defined by the iterable passed.
    pub fn node_as_ref<I>(&self, iter: I) -> Option<&Trie<K, V>>
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter.into_iter() {
            if !node.children.contains_key(&c) {
                return None;
            }
            node = node.children.get(&c).unwrap();
        }
        Some(node)
    }
    pub fn node_as_mut<I>(&mut self, iter: I) -> Option<&mut Trie<K, V>>
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter.into_iter() {
            let tmp = node;
            if let Some(next) = tmp.children.get_mut(&c) {
                node = next;
            } else {
                return None;
            }
        }
        Some(node)
    }
    /// if iter is in the Trie, return a Vec of the
    pub fn list_children<'a>(&'a self, iter: &[K]) -> Option<Vec<Vec<K>>> {
        self.node_as_ref(iter).and_then(|t| {
            let mut ret = Vec::new();

            let mut node = t;
            for k in node.children.keys() {
                let mut item = Vec::with_capacity(iter.len() + 1);
                item.extend_from_slice(&iter);
                item.push(k.clone());

                if let Some(tt) = t.children.get(&k) {
                    std::mem::replace(&mut node, tt);
                    if let Some(x) = self.list_children(&item) {
                        ret.extend_from_slice(&x);
                    }
                    ret.push(item);
                }
            }
            Some(ret)
        })
    }
    pub fn remove<I>(&mut self, iter: I)
        where I: IntoIterator<Item = &'key K>
    {
        // let mut node = self;
        // for c in key {
        //     // let tmp = node;
        //     if let Occupied(mut v) = node.children.entry(c.clone()) {
        //         if let Some(t) = v.get_mut().remove_k(&c) {
        //             node = &mut t;
        //         } else {
        //             break;
        //         }
        //     } else {
        //         break;
        //     }
        // }
    }
    fn remove_k(&mut self, c: &K) -> Option<Trie<K, V>> {
        self.children.remove(c)
    }
}


#[cfg(test)]
mod tests {
    use super::Trie;

    fn build_trie() -> Trie<char, u8> {
        let mut trie: Trie<char, u8> = Trie::new();
        trie.insert(&['a', 'b'], 20);
        trie.insert(&"first".chars().collect::<Vec<char>>(), 40);
        trie.insert(&"fibonnaci".chars().collect::<Vec<char>>(), 40);
        trie
    }
    #[test]
    fn test_contains() {
        let trie = build_trie();
        assert!(trie.contains_prefix(&['f', 'i']));
        assert!(trie.contains_prefix(&['a']));
        assert!(trie.contains_prefix(&['f', 'i', 'r', 's', 't']));
    }
    #[test]
    fn test_raw_insert() {
        let mut trie = Trie::new();
        trie.insert_raw(&"first".chars().collect::<Vec<char>>(), 40);
        trie.insert_raw(&"fibonnaci".chars().collect::<Vec<char>>(), 40);
        assert!(trie.contains_prefix(&"fibonn".chars().collect::<Vec<char>>()));
    }
    #[test]
    fn test_get_children() {
        let trie = build_trie();

        let mut res = Vec::new();
        res.push(vec!['f', 'i', 'r', 's', 't']);

        assert_eq!(Some(res), trie.list_children(&['f', 'i', 'r', 's']));

        assert_eq!(None, trie.list_children(&['a', 'b', 'c']));
        assert_eq!(None, trie.list_children(&['x']));
    }

}

// #![feature(test)]

// extern crate test;

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

/// for Keys
pub trait Key: Eq + Hash + Clone + Debug {}
impl<K> Key for K where K: Eq + Hash + Clone + Debug {}
/// for Values
pub trait Value: Debug {}
impl<V> Value for V where V: Debug {}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Trie<K, V>
    where K: Key,
          V: Value
{
    pub value: Option<V>,
    pub children: HashMap<K, Trie<K, V>>,
}

pub struct TrieIterator<K, V>
    where K: Key,
          V: Value
{
    prefix: Vec<K>,
    cur: Trie<K, V>,
    stack: Vec<HashMap<K, Trie<K, V>>>,
}

impl<K, V> Iterator for TrieIterator<K, V>
    where K: Key,
          V: Value
{
    type Item = Trie<K, V>;
    fn next(&mut self) -> Option<Trie<K, V>> {
        unimplemented!();
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
        for c in iter {
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
        for c in iter {
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
        for c in iter {
            if !node.children.contains_key(c) {
                return false;
            }
            std::mem::replace(&mut node, &node.children[c]); // node.children().get(c).unwrap()
        }
        true
    }
    /// Returns true if value is None and chilren is also empty.
    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }
    /// returns the node at a given position as defined by the iterable passed.
    pub fn get_node_ref<I>(&self, iter: I) -> Option<&Trie<K, V>>
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter {
            match node.children.get(c) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }
    pub fn get_node_mut<I>(&mut self, iter: I) -> Option<&mut Trie<K, V>>
        where I: IntoIterator<Item = &'key K>
    {
        let mut node = self;
        for c in iter {
            let tmp = node;
            match tmp.children.get_mut(c) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }
    /// if iter is in the Trie, return a Vec of the
    pub fn list_children(&self, iter: &[K]) -> Option<Vec<Vec<K>>> {
        self.get_node_ref(iter).and_then(|t| {
            let mut ret = Vec::new();
            let mut node = t;

            for k in node.children.keys() {
                let mut item = Vec::with_capacity(iter.len() + 1);
                item.extend_from_slice(iter);
                item.push(k.clone());

                if let Some(tt) = t.children.get(k) {
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
    // pub fn remove<I>(&mut self, iter: I)
    //     where I: IntoIterator<Item = &'key K>
    // {
    //     let mut node = self;
    //     for c in iter.into_iter() {
    //         let tmp = node;
    //         // if let Occupied(mut entry) = tmp.children.entry(c.clone()) {
    //         //     // entry.get_mut().remove(iter);
    //         //     entry.remove();
    //         //     node = entry;
    //         // }
    //         if let Some(next) = tmp.children.remove(&c) {
    //             node = &mut next;
    //         }
    //     }
    //
    // }
    pub fn remove<I>(&mut self, iter: I) -> bool
        where I: IntoIterator<Item = &'key K>
    {
        let mut prefix = iter.into_iter();
        match prefix.next() {
            None => {
                self.value = None;
            }
            Some(c) => {
                if let Occupied(mut entry) = self.children.entry(c.clone()) {
                    let delete_child = entry.get_mut().remove(prefix);
                    if delete_child {
                        entry.remove();
                    }
                }
            }
        }
        self.is_empty()
    }
}


#[cfg(test)]
mod tests {
    use super::{Trie, test};

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
        println!("{:#?}", trie); // pretty print trie

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
    fn test_list_children() {
        let trie = build_trie();

        let mut res = Vec::new();
        res.push(vec!['f', 'i', 'r', 's', 't']);

        assert_eq!(Some(res), trie.list_children(&['f', 'i', 'r', 's']));

        assert_eq!(None, trie.list_children(&['a', 'b', 'c']));
        assert_eq!(None, trie.list_children(&['x']));
    }
    #[test]
    fn test_remove() {
        let mut trie = build_trie();
        trie.remove(&['f', 'i', 'r', 's', 't']);

        let mut eq: Trie<char, u8> = Trie::new();
        eq.insert(&['a', 'b'], 20);
        eq.insert(&"fibonnaci".chars().collect::<Vec<char>>(), 40);

        assert_eq!(eq, trie);
    }
    /// in this (albeit extremely poor) benchmark, unsafe insert performs only very marginally
    /// better than a regular insert. Both insert and insert_fold seem to perform exactly the same
    /// which leads me to believe the compiler probably unrolls them into very similar assembly
    #[bench]
    fn bench_insert_unsafe(b: &mut test::Bencher) {
        let mut trie = Trie::new();

        b.iter(|| {
            trie.insert_raw(&['a', 'b'], 10);
            trie.insert_raw(&['a', 'b', 'c'], 10);
            trie.insert_raw(&['z', 't', 'q'], 10);
        });
    }
    #[bench]
    fn bench_insert(b: &mut test::Bencher) {
        let mut trie = Trie::new();

        b.iter(|| {
            trie.insert(&['a', 'b'], 10);
            trie.insert(&['a', 'b', 'c'], 10);
            trie.insert(&['z', 't', 'q'], 10);
        });
    }
    #[bench]
    fn bench_insert_fold(b: &mut test::Bencher) {
        let mut trie = Trie::new();

        b.iter(|| {
            trie.insert_fold(&['a', 'b'], 10);
            trie.insert_fold(&['a', 'b', 'c'], 10);
            trie.insert_fold(&['z', 't', 'q'], 10);
        });
    }
}

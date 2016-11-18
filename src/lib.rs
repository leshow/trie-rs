use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::Debug;


#[derive(Debug)]
pub struct Trie<K, V>
    where V: Eq,
          K: Eq + Hash + Clone
{
    pub value: Option<V>,
    pub children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Trie<K, V>
    where V: Eq,
          K: Eq + Hash + Clone
{
    pub fn new() -> Trie<K, V> {
        Trie {
            children: HashMap::new(),
            value: None,
        }
    }
}

impl<K, V> Trie<K, V>
    where V: Eq,
          K: Eq + Hash + Clone
{
    pub fn insert<I>(&mut self, iter: I, value: V)
        where I: Iterator<Item = K>
    {
        // let mut current_node = self;
        // for c in iter.into_iter() {
        //     let node = match current_node.children.entry(c) {
        //         Vacant(slot) => slot.insert(Trie::new()),
        //         Occupied(slot) => slot.into_mut(),
        //     };
        //     current_node = node;
        // }
        let node = iter.fold(self, |current_node, c| {
            match current_node.children.entry(c) {
                Vacant(slot) => slot.insert(Trie::new()),
                Occupied(slot) => slot.into_mut(),
            }
        });
        node.value = Some(value);
    }
    // fn remove<I>(&self, iter: I) -> bool {
    //     unimplemented!();
    // }
    // fn contains<I: IntoIterator>(&self, iter: I) -> bool {
    //     unimplemented!();
    // }
}


#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn test_insert() {
        let mut trie: Trie<char, u8> = Trie::new();
        trie.insert("first".chars(), 20);
        println!("{:?}", trie);
    }
}

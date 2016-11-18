use std::collections::HashMap;
use std::hash::Hash;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::fmt::Debug;

#[derive(Clone)]
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
    pub fn with_value(self, v: Option<V>) -> Trie<K, V> {
        Trie { value: v, ..self }
    }
}

impl<K, V> Debug for Trie<K, V>
    where V: Eq + Debug,
          K: Eq + Hash + Clone + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Trie {{ \n \tchildren: {:?}, \n \tvalue: {:?} \n }}",
               self.children,
               self.value)
    }
}

impl<K, V> Trie<K, V>
    where V: Eq + Clone,
          K: Eq + Hash + Clone
{
    pub fn insert<I>(&mut self, iter: I, value: V)
        where I: Iterator<Item = K>
    {
        // if let Some(first) = iter.nth(0) {
        //     self.children.entry(first.clone()).or_insert(Trie::new()).insert(iter, value);
        // } else {
        //     self.value = Some(value);
        // }
        let node = iter.fold(self, |cur_node, c| {
            match cur_node.children.entry(c) {
                Vacant(v) => v.insert(Trie::new()),
                Occupied(v) => v.into_mut(),
            }
        });
        node.value = Some(value);
    }
    // fn remove<I>(&self, iter: I) -> bool {
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
        trie.insert("fib".chars(), 30);
        trie.insert("fibonacci".chars(), 30);
        trie.insert("hello".chars(), 30);
        println!("{:?}", trie);
    }
}

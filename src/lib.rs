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
        let mut node = self;
        for c in iter {
            let tmp = node;
            node = tmp.children.entry(c).or_insert(Trie::new());
        }
        node.value = Some(value);
    }

    pub fn insert_fold<I>(&mut self, iter: I, value: V)
        where I: Iterator<Item = K>
    {
        let node = iter.fold(self, |cur_node, c| {
            match cur_node.children.entry(c) {
                Vacant(v) => v.insert(Trie::new()),
                Occupied(v) => v.into_mut(),
            }
        });
        node.value = Some(value);
    }

    pub fn insert_raw<I>(&mut self, iter: I, value: V)
        where I: Iterator<Item = K>
    {
        let mut node = self;
        let mut raw_ptr = node as *mut Trie<K, V>;
        for c in iter {
            raw_ptr = node.children.entry(c).or_insert(Trie::new());
            unsafe {
                node = &mut *raw_ptr;
            }
        }
        node.value = Some(value);
    }
    fn contains_prefix<I>(&self, key: I) -> bool
        where I: Iterator<Item = K>
    {
        let mut node = self;
        for c in key {
            if !node.children.contains_key(&c) {
                return false;
            }
            let tmp = node;
            node = tmp.children.get(&c).unwrap();
        }
        return true;
    }
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
    #[test]
    fn test_contains() {
        let mut trie: Trie<char, u8> = Trie::new();
        trie.insert("first".chars(), 20);
        assert!(trie.contains_prefix("f".chars()));
        assert!(trie.contains_prefix("fi".chars()));
    }
}

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
            node = tmp.children.entry(c).or_insert_with(Trie::new);
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
        let mut raw_ptr: *mut Trie<K, V>; // node as *mut Trie<K, V>;
        for c in iter {
            raw_ptr = node.children.entry(c).or_insert_with(Trie::new);
            unsafe {
                node = &mut *raw_ptr;
            }
        }
        node.value = Some(value);
    }
    pub fn contains_prefix<I>(&self, key: I) -> bool
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
        true
    }
    pub fn remove<I>(&mut self, key: I)
        where I: Iterator<Item = K>
    {
        // let mut node = self;
        // for c in key {
        //     let tmp = node;
        //     if let Occupied(mut v) = tmp.entry(c.clone()) {
        //         if let Some(t) = v.get_mut().remove_k(&c) {
        //
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
    #[test]
    fn test_insert() {
        let mut trie: Trie<char, u8> = Trie::new();
        trie.insert("first".chars(), 20);
        trie.insert("fib".chars(), 30);
        trie.insert("fibonacci".chars(), 30);
        trie.insert("hello".chars(), 30);
    }
    #[test]
    fn test_vec_insert() {
        let mut trie = Trie::new();
        let b = vec![1, 2, 3, 4];
        trie.insert(&b[..], 15);
    }
    #[test]
    fn test_contains() {
        let mut trie: Trie<char, u8> = Trie::new();
        trie.insert("first".chars(), 20);
        assert!(trie.contains_prefix("f".chars()));
        assert!(trie.contains_prefix("fi".chars()));
        assert!(trie.contains_prefix("first".chars()));
    }
}

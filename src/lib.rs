use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;
// struct Node<V> {
//     value: V,
//     key: u8,
//     children: Trie<V>,
// }
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

// trait TrieSearch<V>
//     where V: Eq
// {
//     fn insert<I: IntoIterator>(&self, iter: I, value: V);
//     fn remove<I: IntoIterator>(&self, iter: I) -> bool;
//     fn contains<I: IntoIterator>(&self, iter: I) -> bool;
// }
// <V> TrieSearch<V> for
impl<K, V> Trie<K, V>
    where V: Eq,
          K: Eq + Hash + Clone
{
    pub fn insert<I>(&mut self, iter: I, value: V)
        where I: IntoIterator<Item = K>
    {
        let mut node = Trie::new();
        for c in iter.into_iter() {
            if self.children.contains_key(&c) {
                node = self.children.get(&c).unwrap(); // TODO
            } else {
                self.children.insert(&c, Trie::new());
            }
        }
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

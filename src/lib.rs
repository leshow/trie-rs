use std::collections::HashMap;
use std::fmt::Debug;
// struct Node<V> {
//     value: V,
//     key: u8,
//     children: Trie<V>,
// }
#[derive(Debug)]
struct Trie<V> {
    value: Option<V>,
    key: Option<u8>,
    children: HashMap<u8, Trie<V>>,
}

impl<V> Trie<V>
    where V: Eq
{
    fn new() -> Trie<V> {
        Trie {
            children: HashMap::new(),
            value: None,
            key: None,
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
impl<V> Trie<V>
    where V: Eq
{
    fn insert<I>(&self, iter: I, value: V)
        where I: IntoIterator,
              I::Item: Debug
    {
        for c in iter {
            println!("{:?}", c);
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
        let trie = Trie::new();
        trie.insert("first".chars(), 20);
        println!("{:?}", trie);
    }
}

use std::{
    borrow::Borrow,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap,
    },
    hash::Hash,
    iter::FromIterator,
};

pub struct Trie<K, V> {
    value: Option<V>,
    children: HashMap<K, Trie<K, V>>,
}

impl<K, V> Default for Trie<K, V>
where
    K: Eq + Hash,
{
    fn default() -> Self {
        Trie {
            value: None,
            children: HashMap::new(),
        }
    }
}

impl<K, V> Trie<K, V>
where
    K: Eq + Hash,
{
    pub fn new() -> Self {
        Trie::default()
    }

    pub fn insert<Q: ?Sized, I: IntoIterator<Item = K>>(&mut self, prefix: I, value: V)
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut node = self;
        for c in prefix {
            let tmp = node;
            node = tmp.children.entry(c).or_insert_with(Trie::new);
        }
        node.value = Some(value);
    }

    pub fn insert_alt<Q: ?Sized, I: IntoIterator<Item = K>>(&mut self, prefix: I, value: V)
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut node = self;
        let mut ptr: *mut Trie<K, V>;
        for c in prefix {
            ptr = node.children.entry(c).or_insert_with(Trie::new);
            unsafe {
                node = &mut *ptr;
            }
        }
        node.value = Some(value);
    }

    pub fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }

    pub fn get_ref<Q: ?Sized, I: IntoIterator<Item = K>>(&self, prefix: I) -> Option<&Trie<K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut node = self;
        for c in prefix {
            let tmp = node;
            match tmp.children.get(c.borrow()) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }

    pub fn get_mut<Q: ?Sized, I: IntoIterator<Item = K>>(
        &mut self,
        prefix: I,
    ) -> Option<&mut Trie<K, V>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut node = self;
        for c in prefix {
            let tmp = node;
            match tmp.children.get_mut(c.borrow()) {
                Some(next) => node = next,
                None => return None,
            }
        }
        Some(node)
    }

    // pub fn collect_until<Q: ?Sized, I, B>(&self, prefix: I) -> Option<B>
    // where
    //     I: IntoIterator<Item = K>,
    //     B: FromIterator<K>,
    //     K: Borrow<Q>,
    //     Q: Hash + Eq,
    // {
    //     let node = self.get_ref(prefix)?;
    //     Some(FromIterator::from_iter(node.children.keys()))
    // }

    pub fn remove<Q: ?Sized, I: IntoIterator<Item = K>>(&mut self, prefix: I) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut iter = prefix.into_iter();
        match iter.next() {
            None => {
                self.value = None;
            }
            Some(c) => {
                if let Occupied(mut entry) = self.children.entry(c) {
                    let del = entry.get_mut().remove(iter);
                    if del.is_some() {
                        entry.remove();
                        return del;
                    }
                }
            }
        }
        None
    }
}

impl<V> Trie<char, V> {
    pub fn insert_str<S: AsRef<str>>(&mut self, prefix: S, value: V) {
        self.insert(prefix.as_ref().chars(), value)
    }
}

impl<V> Trie<u8, V> {
    pub fn insert_bytes<S: AsRef<str>>(&mut self, prefix: S, value: V) {
        self.insert(prefix.as_ref().bytes(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("foobar".chars(), "val");
        let s = String::from("foobaz");
        trie.insert(s.chars(), "other");
        trie.insert_str("stuff", "okay");
    }
}

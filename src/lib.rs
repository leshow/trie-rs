use std::{
    borrow::Borrow,
    clone::Clone,
    collections::{
        hash_map::{
            self,
            Entry::{Occupied, Vacant},
            RandomState,
        },
        HashMap, VecDeque,
    },
    hash::Hash,
    iter::FromIterator,
    marker::PhantomData,
    ptr::NonNull,
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

    pub fn iter(&'_ self) -> Iter<'_, K, V> {
        Iter {
            prefix: Vec::new(),
            started: false,
            root: self,
            stack: vec![],
        }
    }
}

impl<V> Trie<char, V> {
    pub fn insert_str<S: AsRef<str>>(&mut self, prefix: S, value: V) {
        self.insert(prefix.as_ref().chars(), value)
    }
}

// impl<V, Q> Trie<&Q, V> {
//     pub fn insert_bytes<'a, S: AsRef<[Q]> + 'a>(&mut self, prefix: S, value: V)
//     where
//         Q: Borrow<u8> + Eq + Hash,
//     {
//         self.insert::<&Q, _>(prefix.as_ref(), value)
//     }
// }

pub struct Iter<'a, K, V> {
    prefix: Vec<&'a K>,
    started: bool,
    root: &'a Trie<K, V>,
    stack: Vec<hash_map::Iter<'a, K, Trie<K, V>>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = (Vec<&'a K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            self.stack.push(self.root.children.iter());
        }
        loop {
            match self.stack.last_mut() {
                Some(last) => match last.next() {
                    Some((k, child)) => {
                        self.stack.push(child.children.iter());
                        self.prefix.push(k);
                        if let Some(ref v) = child.value {
                            return Some((self.prefix.clone(), v));
                        }
                    }
                    None => {
                        self.prefix.pop();
                        self.stack.pop();
                    }
                },
                None => return None,
            }
        }
    }
}

// impl<K, V> IntoIterator for Trie<K, V> where K: Eq+Hash{
//     type Item = (K, V);
//     type IntoIter =IntoIter<K, V>;

//     fn into_iter(self) -> IntoIter<K, V> {
//        self.into_iter()
//     }

// }

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
    // #[test]
    // fn test_insert_bytes() {
    //     let mut trie = Trie::new();
    //     trie.insert("foobar".chars(), "val");
    //     let s = String::from("foobaz");
    //     trie.insert(s.chars(), "other");
    //     trie.insert_str("stuff", "okay");
    // }

    #[test]
    fn test_iter() {
        let mut trie = Trie::new();
        trie.insert::<&u8, _>(b"stuff", 1);
        trie.insert::<&u8, _>(b"staff", 2);
        trie.insert::<&u8, _>(b"stack", 3);

        for stuff in trie.iter() {
            println!("{:?}", stuff);
        }
    }
}

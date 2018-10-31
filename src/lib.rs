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
    fmt::Debug,
    hash::Hash,
    iter::FromIterator,
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
    K: Hash + Eq,
{
    fn new() -> Self {
        Self::default()
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
            match node.children.get(c.borrow()) {
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

    pub fn values_vec<Q: ?Sized>(&'_ self) -> Vec<&'_ V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        let mut queue = VecDeque::new();
        queue.push_front(self);
        let mut values = vec![];
        while !queue.is_empty() {
            if let Some(node) = queue.pop_front() {
                for (_, child) in node.children.iter() {
                    queue.push_back(child);
                    if let Some(ref val) = child.value {
                        values.push(val);
                    }
                }
            }
        }
        FromIterator::from_iter(values)
    }

    pub fn values<'a, B, Q: ?Sized>(&'a self) -> B
    where
        B: FromIterator<&'a V>,
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        FromIterator::from_iter(self.values_vec())
    }

    pub fn values_prefix<I, Q: ?Sized>(&'_ self, prefix: I) -> Vec<&'_ V>
    where
        I: IntoIterator<Item = K>,
        V: Debug,
        K: Borrow<Q> + Debug,
        Q: Hash + Eq,
    {
        let mut node = self;
        let mut values = Vec::new();
        for c in prefix {
            if let Some(ref v) = node.value {
                values.push(v);
            }
            match node.children.get(c.borrow()) {
                Some(next) => node = next,
                None => {
                    break;
                }
            }
        }
        if let Some(ref v) = node.value {
            values.push(v);
        }
        values
    }

    pub fn iter(&'_ self) -> Iter<'_, K, V> {
        Iter {
            prefix: Vec::new(),
            started: false,
            node: self,
            stack: Vec::new(),
        }
    }
}

impl<V> Trie<char, V> {
    pub fn insert_str<S: AsRef<str>>(&mut self, prefix: S, value: V) {
        self.insert(prefix.as_ref().chars(), value)
    }

    pub fn get_ref_str<Q: ?Sized, S: AsRef<str>>(&self, prefix: S) -> Option<&Trie<char, V>>
    where
        Q: Hash + Eq,
    {
        self.get_ref(prefix.as_ref().chars())
    }

    pub fn get_mut_str<Q: ?Sized, S: AsRef<str>>(&mut self, prefix: S) -> Option<&mut Trie<char, V>>
    where
        Q: Hash + Eq,
    {
        self.get_mut(prefix.as_ref().chars())
    }

    pub fn remove_str<Q: ?Sized, S: AsRef<str>>(&mut self, prefix: S) -> Option<V>
    where
        Q: Hash + Eq,
    {
        self.remove(prefix.as_ref().chars())
    }
}

impl<V> Trie<u8, V> {
    pub fn insert_bytes<S>(&mut self, prefix: S, value: V)
    where
        S: AsRef<[u8]>,
    {
        self.insert(prefix.as_ref().into_iter().cloned(), value)
    }

    pub fn get_ref_str<Q: ?Sized, S: AsRef<[u8]>>(&self, prefix: S) -> Option<&Self>
    where
        Q: Hash + Eq,
    {
        self.get_ref(prefix.as_ref().into_iter().cloned())
    }

    pub fn get_mut_str<Q: ?Sized, S: AsRef<[u8]>>(&mut self, prefix: S) -> Option<&mut Self>
    where
        Q: Hash + Eq,
    {
        self.get_mut(prefix.as_ref().into_iter().cloned())
    }

    pub fn remove_str<Q: ?Sized, S: AsRef<[u8]>>(&mut self, prefix: S) -> Option<V>
    where
        Q: Hash + Eq,
    {
        self.remove(prefix.as_ref().into_iter().cloned())
    }
}

pub struct Iter<'a, K: 'a, V: 'a> {
    prefix: Vec<&'a K>,
    started: bool,
    node: &'a Trie<K, V>,
    stack: Vec<hash_map::Iter<'a, K, Trie<K, V>>>,
}

#[derive(Debug)]
pub struct IterItem<'a, K: 'a, V: 'a> {
    prefix: Vec<&'a K>,
    value: &'a V,
}

impl<'a, K: 'a, V: 'a> IterItem<'a, K, V> {
    pub fn new(prefix: Vec<&'a K>, value: &'a V) -> Self {
        IterItem { prefix, value }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V>
where
    K: Eq + Hash,
{
    type Item = IterItem<'a, K, V>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.started {
            self.started = true;
            self.stack.push(self.node.children.iter());
        }
        loop {
            let node = match self.stack.last_mut() {
                Some(last) => match last.next() {
                    Some((k, child)) => Some((k, child)),
                    None => None,
                },
                None => return None,
            };
            match node {
                Some((k, child)) => {
                    self.stack.push(child.children.iter());
                    self.prefix.push(k);
                    if let Some(ref value) = child.value {
                        return Some(IterItem::new(self.prefix.clone(), value));
                    }
                }
                None => {
                    self.prefix.pop();
                    self.stack.pop();
                }
            }
            // TODO: requires NLL
            // match self.stack.last_mut() {
            //     Some(last) => match last.next() {
            //         Some((k, child)) => {
            //             self.stack.push(child.children.iter());
            //             self.prefix.push(k);
            //             if let Some(ref value) = child.value {
            //                 return Some(IterItem::new(self.prefix.clone(), value));
            //             }
            //         }
            //         None => {
            //             self.prefix.pop();
            //             self.stack.pop();
            //         }
            //     },
            //     None => return None,
            // }
        }
    }
}

impl<'a, K, V> IntoIterator for &'a Trie<K, V>
where
    K: Eq + Hash,
{
    type IntoIter = Iter<'a, K, V>;
    type Item = IterItem<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, P> Extend<(P, V)> for &'a mut Trie<K, V>
where
    P: IntoIterator<Item = K>,
    K: Eq + Hash,
{
    fn extend<I: IntoIterator<Item = (P, V)>>(&mut self, iter: I) {
        for (prefix, v) in iter {
            self.insert(prefix, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix_values() {
        let mut trie = Trie::new();
        trie.insert("foo".chars(), 1);
        trie.insert("foob".chars(), 2);
        trie.insert("fooba".chars(), 3);
        trie.insert("foobar".chars(), 4);
        trie.insert("foobarzz".chars(), 5);
        assert_eq!(vec![&1, &2, &3, &4], trie.values_prefix("foobar".chars()));
    }

    #[test]
    fn test_other() {
        let mut trie = Trie::new();
        trie.insert("foo".chars(), 1);
        trie.insert("foob".chars(), 2);
        trie.insert("fooba".chars(), 3);
        trie.insert("foobar".chars(), 4);
        trie.insert("foobarzz".chars(), 5);
        assert_eq!(vec![&1, &2, &3, &4, &5], trie.values_vec());
        assert_eq!(vec![&1, &2, &3, &4, &5], trie.values::<Vec<_>, _>());
    }
    #[test]
    fn test_insert() {
        let mut trie = Trie::new();
        trie.insert("foobar".chars(), "val");
        let s = String::from("foobaz");
        trie.insert(s.chars(), "other");
        trie.insert_str("stuff", "okay");
        assert_eq!(trie.get_mut("stuff".chars()).unwrap().value, Some("okay"));
    }

    #[test]
    fn test_bytes_iter() {
        let mut trie = Trie::new();
        trie.insert_bytes(b"stuff", 1);
        trie.insert_bytes(b"staff", 2);
        trie.insert_bytes(b"stack", 3);

        for stuff in trie.iter() {
            println!("{:?}", stuff);
        }
    }

}

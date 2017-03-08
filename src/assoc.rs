// not happy with the previous interface, trying associated types
trait PrefixTrie<V>
    where V: Debug
{
    type K: Key;
    fn new() -> Self;
    fn insert<I: Into<Vec<Self::K>>>(&mut self, iter: I, value: V);
    fn insert_fold<I: Into<Vec<Self::K>>>(&mut self, iter: I, value: V);
    fn contains_prefix<I: Into<Vec<Self::K>>>(&self, iter: I) -> bool;
    fn is_empty(&self) -> bool;
    fn remove<I: Into<Vec<Self::K>>>(&mut self, iter: I) -> bool;
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Trie<V>
    where V: Debug
{
    pub value: Option<V>,
    pub children: HashMap<u8, Trie<V>>,
}

impl<V> PrefixTrie<V> for Trie<V>
    where V: Debug
{
    type K = u8;
    fn new() -> Trie<V> {
        Trie {
            children: HashMap::new(),
            value: None,
        }
    }
    /// inserts something iteratable and a value into the Trie
    /// if that sequence already exists the value will be replaced with the new one
    fn insert<I: Into<Vec<Self::K>>>(&mut self, iter: I, value: V) {
        let mut node = self;
        for c in iter.into() {
            let tmp = node;
            node = tmp.children.entry(c).or_insert_with(Trie::new);
        }
        node.value = Some(value);
    }
    /// Inserts using a fold operation over the iterator passed to it
    /// may possibly be faster than regular insert.
    fn insert_fold<I: Into<Vec<Self::K>>>(&mut self, iter: I, value: V) {
        let node = iter.into().into_iter().fold(self, |cur_node, c| match cur_node.children
            .entry(c.clone()) {
            Vacant(v) => v.insert(Trie::new()),
            Occupied(v) => v.into_mut(),
        });
        node.value = Some(value);
    }
    fn is_empty(&self) -> bool {
        self.value.is_none() && self.children.is_empty()
    }
    fn contains_prefix<I: Into<Vec<Self::K>>>(&self, iter: I) -> bool {
        let mut node = self;
        for c in iter.into() {
            if !node.children.contains_key(&c) {
                return false;
            }
            std::mem::replace(&mut node, &node.children[&c]); // node.children().get(c).unwrap()
        }
        true
    }
    fn remove<I: Into<Vec<Self::K>>>(&mut self, iter: I) -> bool {
        let mut prefix = iter.into().into_iter();
        match prefix.next() {
            None => {
                self.value = None;
            }
            Some(c) => {
                if let Occupied(mut entry) = self.children.entry(c.clone()) {
                    let delete_child = entry.get_mut()
                        .remove(prefix);
                    if delete_child {
                        entry.remove();
                    }
                }
            }
        }
        self.is_empty()
    }
}

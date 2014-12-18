use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use inner::kuchevo::Kuchevo;
use map::map_iterator::MapIterator;
use map::pers_map::PersMap;



pub struct MapRevision<K, V> {
    // TODO fix public field, using ::new()
    pub rev: i64,
    pub root: Rc<Kuchevo<K, V>>,
    pub map_itself: Rc<PersMap<K, V>>,
}

impl<K: Ord, V> MapRevision<K, V> {
    pub fn iter<'a>(&'a self) -> MapIterator<'a, K, V> {
        MapIterator::new(&self.root)
    }

    pub fn len(&self) -> uint {
        0u
    }

    pub fn contains(&self, mid: &K) -> bool {
        let mut node = self.root.clone();
        loop {
            let next_node = match node.deref() {
                &Kuchevo::Nil => return false,
                &Kuchevo::Node(ref key, ref value, priority, ref left, ref right) =>
                    if *key < *mid {
                        right.clone()
                    } else if *key > *mid {
                        left.clone()
                    } else {
                        return true;
                    }
            };
            node = next_node;
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> i64 {
        0i64
    }
}

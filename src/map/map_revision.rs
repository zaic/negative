use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use inner::kuchevo::Kuchevo;



pub struct MapRevision<K, V> {
    // TODO fix public field, using ::new()
    pub rev: i64,
    pub root: Rc<Kuchevo<K, V>>,
}

impl<K: Ord, V> MapRevision<K, V> {
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
}

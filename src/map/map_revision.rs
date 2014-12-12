use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use std::vec::Vec;
use map::kuchevo::Kuchevo;



pub struct MapRevision<T> {
    // TODO fix public field, using ::new()
    pub rev: i64,
    pub root: Rc<Kuchevo<T>>,
}

impl<T: Ord> MapRevision<T> {
    pub fn len(&self) -> uint {
        0u
    }

    pub fn contains(&self, value: &T) -> bool {
        let mut node = self.root.clone();
        loop {
            let next_node = match node.deref() {
                &Kuchevo::Nil => return false,
                &Kuchevo::Node(ref key, priority, ref left, ref right) =>
                    if *key < *value {
                        right.clone()
                    } else if *key > *value {
                        left.clone()
                    } else {
                        return true;
                    }
            };
            node = next_node;
        }
    }
}

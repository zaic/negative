use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use inner::kuchevo::Kuchevo;
use map::map_revision::MapRevision;
use std::default::Default;


pub struct PersMap<T> {
    rev: i64,

    roots: TreeMap<i64, Rc<Kuchevo<T>>>,
    last_root: Rc<Kuchevo<T>>,
}

impl<T: Ord + Clone> PersMap<T> {
    pub fn new() -> PersMap<T> {
        PersMap{rev: 0, roots: TreeMap::new(), last_root: Rc::new(Kuchevo::Nil)}
    }

    pub fn get_by_revision(&self, revision : i64) -> MapRevision<T> {
        assert!(revision <= self.rev);
        MapRevision{rev: revision, root: self.roots[revision].clone()}
    }

    pub fn current_revision(&self) -> i64 {
        self.rev
    }



    pub fn insert(&mut self, value: T) -> i64 {
        self.rev += 1;
        self.last_root = (*self.last_root).insert(Kuchevo::new_leaf(value, &(self.rev as int))); // TODO random!!!111
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }

    pub fn remove(&mut self, value: &T) -> i64 {
        self.rev += 1;
        self.last_root = self.last_root.erase(value);
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }
}

#[test]
fn map_test() {
    let mut m = PersMap::<int>::new();
    m.insert(10);
    m.insert(20);
    m.insert(30);
    let map_before = m.get_by_revision(m.current_revision());
    m.remove(&30);
    let map_after  = m.get_by_revision(m.current_revision());
    m.remove(&25);
    m.remove(&20);

    assert_eq!(map_before.contains(&30), true);
    assert_eq!(map_after.contains(&30), false);
}

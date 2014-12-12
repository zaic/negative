use std::collections::tree_map::TreeMap;
use std::rc::Rc;
use map::kuchevo::Kuchevo;
use map::map_revision::MapRevision;


pub struct PersMap {
    rev: i64,

    roots: TreeMap<i64, Rc<Kuchevo<int>>>,
    last_root: Rc<Kuchevo<int>>,
}

impl PersMap {
    pub fn new() -> PersMap {
        PersMap{rev: 0, roots: TreeMap::new(), last_root: Rc::new(Kuchevo::Nil)}
    }

    pub fn get_by_revision(&self, revision : i64) -> MapRevision {
        assert!(revision <= self.rev);
        MapRevision{rev: revision, root: self.roots[revision].clone()}
    }

    pub fn current_revision(&self) -> i64 {
        self.rev
    }



    pub fn len(&self) -> uint {
        // TODO
        0u
    }
/*
    pub fn push(&mut self, value: T) -> i64 {
        self.rev += 1;
        if self.ary.len() == self.len {
            self.ary.push(VectorElement::new());
        }
        self.ary[self.len].add_value(self.rev, Some(value));
        self.len += 1;
        self.rev
    }

    pub fn pop(&mut self) -> i64 {
        assert!(self.len > 0);

        self.rev += 1;
        self.len -= 1;
        self.ary[self.len].add_value(self.rev, None);
        self.rev
    }

    pub fn modify(&mut self, id: uint, value: T) -> i64 {
        assert!(id < self.len);

        self.rev += 1;
        self.ary[id].add_value(self.rev, Some(value));
        self.rev
    }
*/
    pub fn insert(&mut self, value: int) -> i64 {
        self.rev += 1;
        self.last_root = self.last_root.insert(Kuchevo::new_leaf(value, &(self.rev as int))); // TODO random!!!111
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }

    pub fn remove(&mut self, value: &int) -> i64 {
        self.rev += 1;
        self.last_root = self.last_root.erase(value);
        self.roots.insert(self.rev, self.last_root.clone());
        self.rev
    }
}

#[test]
fn map_test() {
    let mut m = PersMap::new();
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
